mod com;
mod data;
mod database;
mod menu;
mod respons_handler;
mod userinput;
mod threads;
use data::TrackerConfig;
use menu::display_menu;
use simplelog::*;
use std::fs::File;
use threads::{Pool, Task};

fn main() {

    let pool : Pool = Pool::new();

    for i in 0..100 {
        let task = Task::new(i);
        pool.add_task(task);
    }

    pool.execute();


    let log_file = File::create("client.log").unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Trace, Config::default(), log_file),
    ])
    .unwrap();
    let tracker_config = TrackerConfig::new();
    display_menu(tracker_config);
}
