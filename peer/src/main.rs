mod back;
mod com;
mod data;
mod db;
mod menu;
mod parser;
mod process;
mod respons_handler;
mod tasks;
//mod threads;
mod userinput;
use data::TrackerConfig;
use menu::display_menu;
use simplelog::*;
use std::fs::File;
//use tasks::EmptyTask;
//use threads::Pool;

fn main() {
    /*
    //multi thread part
    //create pool
    let mut pool: Pool = Pool::new(2);

    //add task
    for _ in 0..2 {
        let task: tasks::EmptyTask = EmptyTask { stream: None };
        pool.add_task(Box::new(task));
    }

    //start update thread
    let tracker_config = TrackerConfig::new();
    pool.start_update(tracker_config, 30);

    //start listening thread
    let peer_config = PeerConfig::from_config();
    pool.start_listening(peer_config);

    //delete pool
    //pool.drop();
    */

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
