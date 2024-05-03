mod back;
mod com;
mod data;
mod db;
mod menu;
mod parser;
mod process;
mod respons_handler;
mod tasks;
mod threads;
mod userinput;
use data::{PeerConfig, TrackerConfig};
use menu::display_menu;
use simplelog::*;
use std::fs::File;
use threads::Pool;


fn main() {
    //multi thread part
    //create pool
    let mut pool: Pool = Pool::new(1);

    /*
    //add task
    for _ in 0..2 {
        let task: tasks::EmptyTask = EmptyTask { stream: None };
        pool.add_task(Box::new(task));
    }
    */
    let args: Vec<String> = std::env::args().collect();
    let tracker_config: TrackerConfig;
    match args.len() {
        1 => {
            // no arguments
            tracker_config = TrackerConfig::new();
        }
        3 => {
            // 2 arguments ip first port second
            tracker_config = TrackerConfig::new_with_args(&args);
        }
        _ => {
            // wrong number of arguments
            println!("Wrong number of arguments");
            tracker_config = TrackerConfig::new();
        }
    }
    //start update thread
    pool.start_update(tracker_config.clone(), 30);

    //start listening thread
    let peer_config = PeerConfig::from_config();
    pool.start_listening(peer_config);

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

    let pool_clone = pool.clone();

    display_menu(tracker_config, pool_clone);

    //delete pool
    pool.drop();
}
