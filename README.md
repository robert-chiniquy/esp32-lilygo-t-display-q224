# esp32-lilygo-t-display-q224

- https://github.com/Xinyuan-LilyGO/T-Display-S3?tab=readme-ov-file
- https://www.lilygo.cc/products/lilygo%C2%AE-ttgo-t-display-1-14-inch-lcd-esp32-control-board?variant=42159376433333
  - factory firmware scans for wifi SSIDs and monitors voltage

# usage

```sh
. ~/export-esp.sh
cargo test
# or cargo build
# or cargo run
```

## dump firmware

```sh
./.embuild/espressif/python_env/idf5.1_py3.9_env/bin/esptool.py --port /dev/cu.usbserial-56D10978951 --baud 115200 read_flash 0x00000 0x400000 backup_firmware.bin
```

### extract partition table from firmware

```sh
dd if=backup_firmware.bin of=partition_table.bin bs=1 skip=32768 count=4096
python3 ./.embuild/espressif/esp-idf/v5.1.2/components/partition_table/gen_esp32part.py --no-verify ./partition_table.bin
# ESP-IDF Partition Table
# Name, Type, SubType, Offset, Size, Flags
nvs,data,nvs,0x9000,20K,
otadata,data,ota,0xe000,8K,
app0,app,ota_0,0x10000,1280K,
app1,app,ota_1,0x150000,1280K,
spiffs,data,spiffs,0x290000,1472K,
```

## extract partition table from artifact built for esp32s3 to see why it is wrong for this board

```sh
python3 ./.embuild/espressif/esp-idf/v5.1.2/components/partition_table/gen_esp32part.py --no-verify target/xtensa-esp32s3-espidf/release/partition-table.bin
Parsing binary partition input...
# ESP-IDF Partition Table
# Name, Type, SubType, Offset, Size, Flags
nvs,data,nvs,0x9000,24K,
phy_init,data,phy,0xf000,4K,
factory,app,factory,0x10000,1M,
```

## restore firmware

```sh
./.embuild/espressif/python_env/idf5.1_py3.9_env/bin/esptool.py --port /dev/cu.usbserial-56D10978951 --baud 115200 write_flash 0x00000 backup_firmware.bin
```

# info

```
Chip is ESP32-D0WDQ6-V3 (revision v3.1)
Features: WiFi, BT, Dual Core, 240MHz, VRef calibration in efuse, Coding Scheme None
Crystal is 40MHz

# cargo espflash board-info
Chip type:         esp32 (revision v3.1)
Crystal frequency: 40MHz
Flash size:        4MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
```
