// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// add #[tauri::command] before every function you want to call from the frontend (js file)

mod back;
mod com;
mod data;
mod db;
mod for_frontend;
mod menu;
mod parser;
mod process;
mod respons_handler;
mod tasks;
mod threads;
mod userinput;
use clap::{builder::NonEmptyStringValueParser, Parser};
use data::{
    set_config_path, set_peer_port, set_tracker_address, set_tracker_port, PeerConfig,
    TrackerConfig,
};
use for_frontend::*;
use ini::Ini;
use lazy_static::lazy_static;
use log::{debug, error, info};
use menu::display_menu;
use num_traits::ToPrimitive;
use regex::Regex;
use simplelog::*;
use std::sync::Arc;
use std::sync::Mutex;

use std::fs::File;
use threads::Pool;

fn main() {
    let interface = "tauri"; // "terminal" or "tauri" or string (-> tauri by default)
    let args = Args::parse();
    let program_const = handle_program_const(args);
    //let mut global_prorgam_const = PROGRAM_CONST.lock().unwrap();
    //*global_prorgam_const = Some(program_const.clone());
    let log_level = program_const.log_level;
    let log_file = File::create("client.log").unwrap();
    CombinedLogger::init(vec![
        TermLogger::new(
            log_level,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(log_level, Config::default(), log_file),
    ])
    .unwrap();

    if interface == "terminal" {
        // config vars
        let num_threads = program_const.num_threads;
        let update_period_secs = program_const.update_period_secs;

        // multi thread part
        // create pool
        let mut pool: Pool = Pool::new(num_threads.to_i32().unwrap());

        let tracker_config = program_const.tracker_config.clone();
        debug!("MAIN: tracker_config : {:?}", tracker_config);
        //start update thread
        pool.start_update(tracker_config.clone(), update_period_secs.to_i32().unwrap());

        //start have thread
        pool.start_have(update_period_secs.to_i32().unwrap());

        //start listening thread
        let peer_config = program_const.peer_config.clone();
        pool.start_listening(peer_config);

        let pool_clone = pool.clone();

        display_menu(program_const, tracker_config, pool_clone);

        //delete pool
        pool.drop();
    } else {
        // config vars
        let num_threads = 1;
        let update_period_secs = 30;

        // multi thread part
        // create pool
        let mut pool: Pool = Pool::new(num_threads.to_i32().unwrap());

        let tracker_config = program_const.tracker_config;

        //start update thread
        pool.start_update(tracker_config.clone(), update_period_secs.to_i32().unwrap());

        //start have thread
        pool.start_have(update_period_secs.to_i32().unwrap());

        //start listening thread
        let peer_config = PeerConfig::new();
        pool.start_listening(peer_config);
        let pool_mutex = Mutex::new(pool.clone());
        let pool_arc = Arc::new(pool_mutex);
        let download_params = DownloadParams {
            pool: pool_arc.clone(),
            tcp_size: program_const.length_tcp,
        };

        tauri::Builder::default()
            .manage(download_params)
            .invoke_handler(tauri::generate_handler![
                get_files_data,
                searchFunction,
                uploadFiles,
                handle_download,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        pool.drop();
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // port d'écoute du peer
    #[clap(short, long)]
    port: Option<u16>,
    // adresse du tracker
    #[clap(short, long)]
    tracker: Option<String>,
    // niveau de debug
    #[clap(short, long)]
    verbose: Option<String>,
    // nombre de threads
    #[clap(short, long)]
    max_connection: Option<u32>,
    // chemin de la config
    #[clap(short, long)]
    config: Option<String>,
    #[clap(short, long)]
    length_tcp: Option<u32>,
    #[clap(short, long)]
    update_period_secs: Option<u32>,
}
#[derive(Debug, Clone)]
struct ProgramConst {
    peer_config: PeerConfig,
    tracker_config: TrackerConfig,
    num_threads: u32,
    update_period_secs: u32,
    length_tcp: u32,
    log_level: LevelFilter,
}

fn handle_program_const(args: Args) -> ProgramConst {
    // handle config
    let config_path = args.config.unwrap_or("config.ini".to_string());
    info!("Choosen config file : {:?}", config_path);
    set_config_path(config_path.clone());
    // handle peer config
    let mut peer_config = PeerConfig::new();
    let mut tracker_config = TrackerConfig::new();
    // open config
    let conf = Ini::load_from_file(&config_path).unwrap();
    // get section
    let peer_section = conf.section(Some("Peer")).unwrap();
    // if user specify a port
    if let Some(port) = args.port {
        peer_config.port = port;
        set_peer_port(port);
    }
    // handle tracker config

    if let Some(tracker) = args.tracker {
        let ip_domain_regex =
            Regex::new(r"(?:\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}|[a-zA-Z0-9.-]+)").unwrap();
        let port_regex = Regex::new(r"\d{1,5}").unwrap();
        let ip_port_domain_regex =
            Regex::new(r"(?:\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}|[a-zA-Z0-9.-]+):\d{1,5}").unwrap();
        if ip_port_domain_regex.is_match(&tracker) {
            let mut split = tracker.split(":");
            set_tracker_address(tracker_config.address.clone());
            tracker_config.address = split.next().unwrap().to_string();
            set_tracker_port(tracker_config.port);
            tracker_config.port = split.next().unwrap().parse::<u16>().unwrap();
            debug!(
                "tracker address : {:?} port : {:?}",
                tracker_config.address, tracker_config.port
            );
        } else if ip_domain_regex.is_match(&tracker) {
            set_tracker_address(tracker_config.address.clone());
            tracker_config.address = tracker;
            debug!("tracker address : {:?}", tracker_config.address);
        } else if port_regex.is_match(&tracker) {
            set_tracker_port(tracker_config.port);
            tracker_config.port = tracker.parse::<u16>().unwrap();
            debug!("tracker port : {:?}", tracker_config.port);
        } else {
            error!("Wrong tracker format, please use ip:port, domain:port, ip, domain or port, using config value")
        }
    }

    // handle number of threads
    let num_threads = args.max_connection.unwrap_or(
        peer_section
            .get("max-connections")
            .unwrap()
            .parse()
            .unwrap(),
    );

    // handle update period
    let update_period_secs = args
        .update_period_secs
        .unwrap_or(peer_section.get("update-period").unwrap().parse().unwrap());
    let length_tcp = args
        .length_tcp
        .unwrap_or(peer_section.get("length-tcp").unwrap().parse().unwrap());
    // handle verbose
    let log_level = match args
        .verbose
        .unwrap_or(peer_section.get("log-level").unwrap().to_string())
    {
        level => match level.as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        },
    };
    let ret = ProgramConst {
        peer_config,
        tracker_config,
        num_threads,
        update_period_secs,
        log_level,
        length_tcp,
    };
    debug!("ProgramConst : {:?}", ret);
    ret
}
