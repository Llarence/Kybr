mod keyboard;
mod problem;
mod gui;

use std::{fs::File, io::{Read, Write}, path::Path, time::Duration};

use argmin::{core::{observers::ObserverMode, Executor, State}, solver::simulatedannealing::SimulatedAnnealing};
use argmin_observer_slog::SlogLogger;
use iced::Task;
use keyboard::{InputKey, IN_KEYS, IN_KEYS_COUNT, OUT_KEYS_COUNT};
use problem::Problem;
use gui::App;
use rand::Rng;

const PATH: &str = "data/keys.data";

fn process() -> Result<(), Box<dyn std::error::Error>> {
    // Temp goes down to fast (maybe) and also it would be better not to hard code the max iters
    let mut runner = Executor::new(Problem, SimulatedAnnealing::new(IN_KEYS_COUNT as f64)?);
    runner = runner.configure(|state| state.param(IN_KEYS).max_iters(10000000));
    runner = runner.add_observer(SlogLogger::term(), ObserverMode::Every(100000));
    let res = runner.run()?;
    let state = res.state();

    let params = match state.get_best_param() {
        Some(params) => params,
        None => return Err("No solution".into())
    };

    let mut file = File::create(PATH)?;
    for (i, key) in params.iter().enumerate() {
        if i == OUT_KEYS_COUNT {
            break;
        }

        file.write_all(&key.as_bytes())?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(PATH).exists() {
        process()?;
    }

    // Maybe more elegant way to do this (it probably gets optimized by the compiler though)
    let mut params: [InputKey; IN_KEYS_COUNT] = [InputKey::new(0, 0); IN_KEYS_COUNT];
    let mut file = File::options().read(true).open(PATH)?;
    let mut buf: [u8; 2] = [0, 0];
    for param in params.iter_mut() {
        file.read_exact(&mut buf)?;
        *param = InputKey::from_bytes(buf);
    }

    let mut rng = rand::thread_rng();
    let mut chars = vec![];
    for char in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        if rng.gen_range(0.0..1.0) > 0.9 {
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
