use std::{fs::File, io::Read, time::Duration};

use iced::Task;
use kybr::key_converter::{InputKey, IN_KEYS_COUNT};
use kybr::gui::App;
use rand::Rng;

const PATH: &str = "data/keys.data";

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut rng = rand::thread_rng();
    let mut chars = vec![];
    for char in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        if rng.gen_range(0.0..1.0) > 0.0 {
            chars.push(char);
        }
    }

    let size = chars.len();
    for i in 0..size - 1 {
        chars.swap(i, rng.gen_range(i..size) as usize);
    }
    iced::application("Tester", App::update, App::view)
        .subscription(App::subscription)
        .run_with(move || (App::new(params, Duration::from_millis(200), String::from_iter(chars)), Task::none()))?;

    Ok(())
}
