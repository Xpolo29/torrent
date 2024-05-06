mod back;
mod com;
mod data;
mod db;
mod menu;
mod parser;
mod process;
mod respons_handler;
mod tasks;
mod userinput;
mod threads;
use data::{TrackerConfig, PeerConfig};
use menu::display_menu;
use simplelog::*;
use std::fs::File;
//use tasks::EmptyTask;
use threads::Pool;
use std::mem;


#[tauri::command]
