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

const PATH: &str = "data/keys.data";

fn process() -> Result<(), Box<dyn std::error::Error>> {
    // Temp goes down to fast (maybe) and also it would be better not to hard code the max iters
    let mut runner = Executor::new(Problem, SimulatedAnnealing::new(IN_KEYS_COUNT as f64)?);
    runner = runner.configure(|state| state.param(IN_KEYS).max_iters(10_000_000));
    runner = runner.add_observer(SlogLogger::term(), ObserverMode::Every(100_000));
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

        file.write(&key.to_bytes())?;
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
    for i in 0..IN_KEYS_COUNT {
        file.read(&mut buf)?;
        params[i] = InputKey::from_bytes(buf);
    }

    iced::application("A cool counter", App::update, App::view)
        .subscription(App::subscription)
        .run_with(move || (App::new(params, Duration::from_millis(200), "abcdefghijklmnopqrstuvwxyz".to_owned()), Task::none()))?;

    Ok(())
}
