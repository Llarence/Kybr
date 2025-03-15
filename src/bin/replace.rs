use std::{env, fs::File, io::Read, panic, process::Command, sync::Arc, time::Duration};

use kybr::{key_converter::{InputKey, IN_KEYS_COUNT, LEFT_KEYS, OUT_KEYS, RIGHT_KEYS}, keyboard::{HIDReader, HIDWriter, CHAR_TO_SHIFTED}, remapper::Remapper};

const PATH: &str = "data/keys.data";

fn pass_through(reader: &mut HIDReader, writer: &mut HIDWriter) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Ok(res) = reader.read_valid() {
            if res.0 == '\x1B' {
                return Ok(());
            }

            writer.press(res.0)?;
        }
    }
}

fn display_hint(reader: &mut HIDReader, params: &[InputKey; IN_KEYS_COUNT]) {
    if let Ok(res) = reader.read_valid() {
        let character = if res.0 == '\x0E' {
            if let Ok(res) = reader.read_valid() {
                if let Some(res) = CHAR_TO_SHIFTED.get(&res.0) {
                    res.to_owned()
                } else {
                    return;
                }
            } else {
                return;
            }
        } else {
            res.0
        };


        if let Some(index) = OUT_KEYS.iter().position(|value| *value == character) {
            let key = params[index];

            Command::new("notify-send")
                .arg(format!("{}:{}", LEFT_KEYS[key.left], RIGHT_KEYS[key.right]))
                .arg("-t")
                .arg("1000")
                .output()
                .expect("Failed to notify-send");
        }
    }
}

fn run(keyboard_id: &str, hid_id: &str, params: &[InputKey; IN_KEYS_COUNT]) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("xinput")
        .arg("float")
        .arg(keyboard_id)
        .output()
        .expect("Failed to disable");

    let mut reader = HIDReader::open(hid_id)?;
    let mut writer = HIDWriter::open()?;
    let mut remapper = Remapper::new(*params, Duration::from_millis(200));

    loop {
        if let Ok(res) = reader.read_valid() {
            if res.0 == '\x1B' {
                pass_through(&mut reader, &mut writer)?;

                continue;
            }

            if res.0 == '\x07' {
                display_hint(&mut reader, params);

                continue;
            }

            let character = remapper.push_key(res.0, res.1);
            if let Some(character) = character {
                writer.press(character)?;
            }
        }
    }
}

fn reenable(id: &str, slave_id: &str) {
    Command::new("xinput")
        .arg("reattach")
        .arg(id)
        .arg(slave_id)
        .output()
        .expect("Failed to reenable");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    sudo::escalate_if_needed()?;

    let mut args_iter = env::args();
    args_iter.next();
    let x_id = Arc::new(args_iter.next().expect("Please specify keyboard id"));
    let slave_id = Arc::new(args_iter.next().expect("Please specify slave keyboard id"));
    let hid_id = args_iter.next().expect("Please specify the hid id");

    // Maybe more elegant way to do this (it probably gets optimized by the compiler though)
    let mut params: [InputKey; IN_KEYS_COUNT] = [InputKey::new(0, 0); IN_KEYS_COUNT];
    let mut file = File::options().read(true).open(PATH)?;
    let mut buf: [u8; 2] = [0, 0];
    for param in params.iter_mut() {
        if file.read(&mut buf)? != buf.len() {
            break;
        }

        *param = InputKey::from_bytes(buf);
    }

    {
        let x_id = x_id.clone();
        let slave_id = slave_id.clone();
        panic::set_hook(Box::new(move |_| { reenable(&x_id, &slave_id); } ))
    }

    {
        let x_id = x_id.clone();
        let slave_id = slave_id.clone();
        ctrlc::set_handler(move || { reenable(&x_id, &slave_id); } )?
    }

    let result = run(&x_id, &hid_id, &params);

    reenable(&x_id, &slave_id);

    result
}
