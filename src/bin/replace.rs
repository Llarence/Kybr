use std::{env, error::Error, fs::File, io::Read, panic, process::Command, sync::Arc, time::Duration};

use kybr::{key_converter::{InputKey, IN_KEYS_COUNT}, keyboard::{HIDReader, HIDWriter}, remapper::Remapper};

const PATH: &str = "data/keys.data";

fn run(id: &str, params: &[InputKey; IN_KEYS_COUNT] ) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("xinput")
        .arg("float")
        .arg(id)
        .output()
        .expect("Failed to disable");

    let mut reader = HIDReader::open("6")?;
    let mut writer = HIDWriter::open()?;
    let mut remapper = Remapper::new(*params, Duration::from_millis(200));

    loop {
        let res: Result<Option<(char, Duration)>, Box<dyn Error>> = reader.read();
        if let Ok(Some(res)) = res {
            if res.0 == 'q' {
                return Ok(());
            }

            let character = remapper.push_key(res.0, res.1);
            if let Some(character) = character {
                let _ = writer.press(character);
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
    let id = Arc::new(args_iter.next().expect("Please specify keyboard id"));
    let slave_id = Arc::new(args_iter.next().expect("Please specify slave keyboard id"));

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
        let id = id.clone();
        let slave_id = slave_id.clone();
        panic::set_hook(Box::new(move |_| { reenable(&id, &slave_id); } ));
    }

    let result = run(&id, &params);

    reenable(&id, &slave_id);

    result
}
