mod menu;
mod userinput;
mod com;
mod data;
use menu::display_menu;
use data::{TrackerConfig};
fn main() {
    env_logger::init();
    let tracker_config = TrackerConfig::new();
    display_menu(tracker_config);
    
}
