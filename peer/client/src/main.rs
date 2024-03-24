mod com;
mod data;
mod menu;
mod respons_handler;
mod userinput;
use data::TrackerConfig;
use menu::display_menu;
fn main() {
    env_logger::init();
    let tracker_config = TrackerConfig::new();
    display_menu(tracker_config);
}
