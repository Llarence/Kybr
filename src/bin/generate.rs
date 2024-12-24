use std::{fs::File, io::Write};

use argmin::{core::{observers::ObserverMode, Executor, State}, solver::simulatedannealing::SimulatedAnnealing};
use argmin_observer_slog::SlogLogger;
use kybr::key_converter::{IN_KEYS, IN_KEYS_COUNT, OUT_KEYS_COUNT};
use kybr::anneal::Problem;

const PATH: &str = "data/keys.data";

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
