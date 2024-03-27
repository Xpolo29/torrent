mod com;
mod data;
mod menu;
mod respons_handler;
mod userinput;
use data::TrackerConfig;
use menu::display_menu;
use simplelog::*;
use std::fs::File;
fn main() {
    let log_file = File::create("client.log").unwrap();
    WriteLogger::init(LevelFilter::Trace, Config::default(), log_file).unwrap();
    let tracker_config = TrackerConfig::new();
    display_menu(tracker_config);
}
