use std::{env, panic, process::Command, sync::Arc};

use kybr::keyboard::HIDReader;

fn run(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("xinput")
        .arg("float")
        .arg(id)
        .output()
        .expect("Failed to disable");

    let mut reader = HIDReader::open("6")?;

    for _ in 0..100 {
        if let Some(t) = reader.read()? {
            println!("{}, {:?}", t.0, t.1);
        }
    }

    Ok(())
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

    {
        let id = id.clone();
        let slave_id = slave_id.clone();
        panic::set_hook(Box::new(move |_| { reenable(&id, &slave_id); } ));
    }

    let result = run(&id);

    reenable(&id, &slave_id);

    result
}
