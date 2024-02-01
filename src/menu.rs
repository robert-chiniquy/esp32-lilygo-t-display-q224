#![allow(unused)]

use embedded_graphics::{
    pixelcolor::Rgb565,
    primitives::Rectangle,
    text::{renderer::CharacterStyle, DecorationColor},
};

use super::*;

use std::collections::{HashMap, HashSet, VecDeque};

/// state = menu item
type MenuId = usize;

enum MenuMode {
    // like scrolling terminal window
    HistoryMode,
    // periodically updating msg fixed in place with a cancel button
    ContinuousOutputMode,
}

enum ButtonHighlight {
    L,
    R,
}

#[derive(Clone)]
pub(super) enum MenuAction {
    Back,
}

pub(crate) struct Menu<const W: usize, const H: usize> {
    changed: bool,
    btn_changed: bool,
    theme: Theme,
    mode: MenuMode,
    status_message: Vec<[char; W]>,
    top_left: Point,
    lines: [[char; W]; H],
    scroll_line_buffer: VecDeque<String>,
    write_line_cursor: usize,
    // ? or a stack instead?
    // todo: yes a stack
    pub current: Option<MenuId>,
    pub selected: Option<MenuId>,
    menu_count: MenuId,
    menu_labels: Vec<&'static str>,
    menu_actions: HashMap<MenuId, MenuAction>,
    // substates are consistently ordered
    menu_graph: HashMap<MenuId, Vec<MenuId>>,
    l_btn_label: &'static str,
    r_btn_label: &'static str,
    btn_highlight: Option<ButtonHighlight>,
    font: embedded_graphics::mono_font::MonoFont<'static>,
}

impl<const W: usize, const H: usize> Menu<W, H> {
    pub(crate) fn new(
        top_left: Point,
        l_btn_label: &'static str,
        r_btn_label: &'static str,
        font: embedded_graphics::mono_font::MonoFont<'static>,
    ) -> Self {
        Self {
            changed: true,
            btn_changed: true,
            theme: Default::default(),
            mode: MenuMode::HistoryMode,
            status_message: vec![],
            top_left,
            lines: [[' '; W]; H],
            scroll_line_buffer: Default::default(),
            write_line_cursor: 0,
            current: None,
            selected: None,
            menu_count: 0,
            menu_labels: Default::default(),
            menu_actions: Default::default(),
            menu_graph: Default::default(),
            l_btn_label,
            r_btn_label,
            btn_highlight: None,
            font,
        }
    }

    pub(crate) fn l_click(&mut self) {
        self.btn_changed = true;
        self.btn_highlight = Some(ButtonHighlight::L)
    }

    pub(crate) fn r_click(&mut self) {
        self.btn_changed = true;
        self.btn_highlight = Some(ButtonHighlight::R)
    }

    pub(crate) fn draw(
        &mut self,
        display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    ) {
        self.draw_buttons(display);
        if !self.changed {
            return;
        }
        self.changed = false;
        match self.mode {
            MenuMode::HistoryMode => {
                if self.write_line_cursor == 0 {
                    // state reset
                    self.render_to_scroll_buffer();
                }
            }
            MenuMode::ContinuousOutputMode => (),
        }
        self.render_to_line_buffer();
        self.draw_lines(display);
        self.draw_selection_query(display);
        // TODO
    }

    fn draw_selection_query(
        &self,
        display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    ) {
        // display
        //     .clipped(&Rectangle::new(Point::new(0, 240 - 27), Size::new(135, 14)))
        //     .clear(embedded_graphics::pixelcolor::Rgb565::BLACK)
        //     .map_err(|_| 3)
        //     .unwrap();
        if let Some(sel) = self.selected {
            let query = format!("? {} ?", self.menu_label(sel));
            let width = query.len() * 8; // for centering
            let position = Point::new(((130 - width) / 2) as i32, 240 - 27);
            let mut style = MonoTextStyle::new(&FONT_8X13, self.theme.green);
            style.set_underline_color(DecorationColor::Custom(self.theme.red));
            style.set_background_color(Some(Rgb565::BLACK));
            // show a ? with the selected menu item label in some special color
            // todo: verb
            Text::new(&query, position, style)
                .draw(display)
                .map_err(|_| "sel q")
                .unwrap();
        }
    }

    fn draw_buttons(
        &mut self,
        display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    ) {
        if !self.btn_changed {
            return;
        }
        self.btn_changed = false;
        match self.mode {
            MenuMode::HistoryMode => {
                // draw L and R buttons
                let mut l_style = PrimitiveStyleBuilder::new()
                    .stroke_color(Theme::default().cyan)
                    .fill_color(Rgb565::BLACK)
                    .stroke_width(2);
                let mut r_style = PrimitiveStyleBuilder::new()
                    .stroke_color(Theme::default().cyan)
                    .fill_color(Rgb565::BLACK)
                    .stroke_width(2);

                if let Some(highlight) = self.btn_highlight.take() {
                    // cause un-highlight to occur next draw
                    // TODO: break out separate change state for buttons vs the rest of the display
                    self.btn_changed = true;
                    match highlight {
                        ButtonHighlight::L => {
                            l_style = l_style
                                .fill_color(Theme::default().magenta)
                                .stroke_color(Theme::default().red)
                                .stroke_width(4)
                        }
                        ButtonHighlight::R => {
                            r_style = r_style
                                .fill_color(Theme::default().magenta)
                                .stroke_color(Theme::default().red)
                                .stroke_width(4)
                        }
                    }
                }

                // L
                Rectangle::new(
                    Point::new(-2, 240 - 17),
                    Size::new(self.l_btn_label.len() as u32 * 9, 23),
                )
                .into_styled(l_style.build())
                .draw(display)
                .map_err(|_| ">")
                .unwrap();

                Text::new(
                    self.l_btn_label,
                    Point::new(0, 240 - (8 / 2)),
                    MonoTextStyle::new(&FONT_8X13, self.theme.yellow),
                )
                .draw(display)
                .map_err(|_| "⑂")
                .unwrap();

                // hack, hardcode behavior assuming r button function is always "next"
                if self.get_submenus(self.current.unwrap()).len() > 1 {
                    // R
                    Rectangle::new(
                        Point::new(135 - (self.r_btn_label.len() as i32 * 10), 240 - 17),
                        Size::new(self.r_btn_label.len() as u32 * 12, 23),
                    )
                    .into_styled(r_style.build())
                    .draw(display)
                    .map_err(|_| ">")
                    .unwrap();

                    Text::new(
                        self.r_btn_label,
                        Point::new(135 - (self.r_btn_label.len() as i32 * 9), 240 - (8 / 2)),
                        MonoTextStyle::new(&FONT_8X13, self.theme.yellow),
                    )
                    .draw(display)
                    .map_err(|_| "⑂")
                    .unwrap();
                }
            }
            MenuMode::ContinuousOutputMode => {
                // draw a cancel button
                Text::new(
                    "cancel",
                    Point::new(0, 240 - 14),
                    MonoTextStyle::new(&FONT_8X13, self.theme.magenta),
                )
                .draw(display)
                .map_err(|_| "⑂")
                .unwrap();
            }
        }
    }

    fn draw_lines(
        &self,
        display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    ) {
        // todo: lines changed flag?
        let character_style = MonoTextStyle::new(&FONT_8X13, self.theme.cyan);
        let mut line_offset = 0;
        for line in self.lines {
            Text::new(
                line.into_iter().collect::<String>().as_str(),
                Point::new(self.top_left.x, self.top_left.y + line_offset),
                character_style,
            )
            .draw(display)
            .map_err(|_| "⑂")
            .unwrap();
            line_offset += self.font.character_size.height as i32 + 3;
        }
    }

    fn render_to_scroll_buffer(&mut self) {
        debug_assert!(matches!(self.mode, MenuMode::HistoryMode));
        if self.write_line_cursor == 0 {
            self.scroll_line_buffer = Default::default();
            self.scroll_line_buffer
                .push_back(self.menu_label(self.current.unwrap()).to_string());
            for menu_item in self.get_submenus(self.current.unwrap()).clone() {
                self.scroll_line_buffer
                    .push_back(self.menu_label(menu_item).to_string());
            }
        }
    }

    fn render_to_line_buffer(&mut self) {
        self.changed = true;
        match self.mode {
            MenuMode::HistoryMode => {
                // if any lines are in the scroll buffer, add one line at the current write line
                // once the current write line reaches the end, append to the bottom
                if let Some(next_line) = self.scroll_line_buffer.pop_front() {
                    let next_line = next_line
                        .chars()
                        .take(W)
                        .chain(std::iter::repeat(' '))
                        .take(W)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    if self.write_line_cursor == H {
                        let mut new_lines = self.lines.iter().skip(1).cloned().collect::<Vec<_>>();
                        new_lines.push(next_line);
                        self.lines = new_lines.try_into().unwrap();
                    } else {
                        self.lines[self.write_line_cursor] = next_line;
                        self.write_line_cursor += 1;
                    }
                }
            }
            MenuMode::ContinuousOutputMode => todo!(),
        }
    }

    pub(crate) fn new_menu(&mut self, label: &'static str) -> MenuId {
        self.changed = true;
        let id = self.menu_count;
        self.menu_labels.insert(id, label);
        self.menu_count += 1;
        id
    }

    pub(super) fn new_menu_item(&mut self, label: &'static str, action: MenuAction) -> MenuId {
        self.changed = true;
        let id = self.new_menu(label);
        self.menu_actions.insert(id, action);
        id
    }

    fn take_action(&mut self, action: MenuAction) {
        self.changed = true;
        match action {
            MenuAction::Back => todo!(),
        }
    }

    pub fn select(&mut self) {
        self.changed = true;
        // "enter" or "select" the currently selected thing
        if let Some(action) = self
            .menu_actions
            .get(self.selected.as_ref().unwrap())
            .cloned()
        {
            self.take_action(action);
        } else {
            self.set_current_menu(self.selected.unwrap())
        }
    }

    /// set the selected item to the id of the next consecutive menu item mod the total count for looping
    pub fn cursor_next(&mut self) {
        self.changed = true;
        // unwrap city
        let submenus = self.get_submenus(self.current.unwrap());
        let current_selected_index = submenus
            .iter()
            .position(|item| item == self.selected.as_ref().unwrap())
            .unwrap(); // 0-indexing
        let len = submenus.len();
        let next_index = ((current_selected_index + 1) % (len)); // 0-indexing

        log::info!(
            "⇟ {}/{} -> {}/{}",
            current_selected_index,
            len,
            next_index,
            len
        );

        self.set_selected_item(*submenus.get(next_index).unwrap());
    }

    pub(crate) fn set_current_menu(&mut self, id: MenuId) {
        self.changed = true;
        self.current = Some(id);
        self.set_selected_item(
            *self
                .get_submenus(id)
                .get(0)
                .expect("must populate menu items before setting a menu to current"),
        );
    }

    pub(crate) fn set_selected_item(&mut self, id: MenuId) {
        self.changed = true;
        self.selected = Some(id);
    }

    pub(crate) fn set_submenus(&mut self, id: MenuId, substates: &[MenuId]) {
        self.changed = true;
        self.menu_graph.insert(id, substates.to_vec());
    }

    pub(crate) fn get_submenus(&self, id: MenuId) -> &Vec<MenuId> {
        self.menu_graph.get(&id).unwrap()
    }

    fn menu_label(&self, id: MenuId) -> &'static str {
        self.menu_labels.get(id).unwrap()
    }
}
