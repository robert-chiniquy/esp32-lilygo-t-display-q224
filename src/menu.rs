#![allow(unused)]

use embedded_graphics::primitives::Rectangle;

use super::*;

use std::collections::{HashMap, HashSet};

/// state = menu item
type StateId = usize;

enum MenuMode {
    // like scrolling terminal window
    HistoryMode,
    // periodically updating msg fixed in place with a cancel button
    ContinuousOutputMode,
}

pub(crate) struct Menu<const W: usize, const H: usize> {
    theme: Theme,
    mode: MenuMode,
    status_message: Vec<[char; W]>,
    top_left: Point,
    lines: [[char; W]; H],
    // ? or a stack instead?
    current: Option<StateId>,
    selected: Option<StateId>,
    state_count: StateId,
    state_labels: Vec<&'static str>,
    // substates are consistently ordered
    state_graph: HashMap<StateId, Vec<StateId>>,
    l_btn_label: &'static str,
    r_btn_label: &'static str,
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
            theme: Default::default(),
            mode: MenuMode::HistoryMode,
            status_message: vec![],
            top_left,
            lines: [[' '; W]; H],
            current: None,
            selected: None,
            state_count: 0,
            state_labels: Default::default(),
            state_graph: Default::default(),
            l_btn_label,
            r_btn_label,
            font,
        }
    }

    pub(crate) fn draw(
        &self,
        display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    ) {
        // TODO
        self.draw_buttons(display)
    }

    fn draw_buttons(
        &self,
        display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    ) {
        match self.mode {
            MenuMode::HistoryMode => {
                // draw L and R buttons
                // L
                Rectangle::new(
                    Point::new(-2, 240 - 17),
                    Size::new(self.l_btn_label.len() as u32 * 9, 23),
                )
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_color(Theme::default().magenta)
                        .stroke_width(1)
                        .build(),
                )
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

                // R
                Rectangle::new(
                    Point::new(135 - (self.r_btn_label.len() as i32 * 10), 240 - 17),
                    Size::new(self.r_btn_label.len() as u32 * 12, 23),
                )
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_color(Theme::default().magenta)
                        .stroke_width(1)
                        .build(),
                )
                .draw(display)
                .map_err(|_| ">")
                .unwrap();

                Text::new(
                    self.r_btn_label,
                    Point::new(135 - (self.r_btn_label.len() as i32 * 10), 240 - (8 / 2)),
                    MonoTextStyle::new(&FONT_8X13, self.theme.yellow),
                )
                .draw(display)
                .map_err(|_| "⑂")
                .unwrap();
            }
            MenuMode::ContinuousOutputMode => {
                // print status msg
                todo!();
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

    pub(crate) fn new_state(&mut self, label: &'static str) -> StateId {
        let id = self.state_count;
        self.state_labels.insert(id, label);
        self.state_count += 1;
        id
    }

    pub(crate) fn set_current_state(&mut self, id: StateId) {
        self.current = Some(id);
    }

    pub(crate) fn set_substates(&mut self, id: StateId, substates: &[StateId]) {
        self.state_graph.insert(id, substates.to_vec());
    }

    pub(crate) fn get_substates(&self, id: StateId) -> &Vec<StateId> {
        self.state_graph.get(&id).unwrap()
    }

    fn state_label(&self, id: StateId) -> &'static str {
        self.state_labels.get(id).unwrap()
    }
}
