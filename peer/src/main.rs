mod com;
mod process;
mod data;
mod db;
mod menu;
mod respons_handler;
mod threads;
mod tasks;
mod userinput;
//mod parser;
use data::TrackerConfig;
use menu::display_menu;
use simplelog::*;
use std::fs::File;
use threads::Pool;
use tasks::EmptyTask;

fn main() {
    //multi thread part
    //create pool
    let mut pool: Pool = Pool::new(2);

    //add task
    for _ in 0..2 {
        let task : tasks::EmptyTask = EmptyTask {stream:None};
        pool.add_task(task);
    }

    //start update thread
    let tracker_config = TrackerConfig::new();
    pool.start_update(tracker_config, 30);

    //delete pool
    pool.drop();

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
