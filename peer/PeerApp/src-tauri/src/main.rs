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
use clap::Parser;
use data::{
    set_config_path, set_peer_port, set_tracker_address, set_tracker_port, PeerConfig,
    TrackerConfig,
};
use for_frontend::*;
use log::{error, info};
use menu::display_menu;
use num_traits::ToPrimitive;
use regex::Regex;
use simplelog::*;
use std::fs::File;
use std::{env, thread};
use threads::Pool;

fn main() {
    let interface = "tauri"; // "terminal" or "tauri" or string (-> tauri by default)

    if interface == "terminal" {
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
        let args = Args::parse();
        let program_const = handle_program_const(args);
        // config vars
        let num_threads = program_const.num_threads;
        let update_period_secs = program_const.update_period_secs;

        // multi thread part
        // create pool
        let mut pool: Pool = Pool::new(num_threads.to_i32().unwrap());

        let tracker_config = program_const.tracker_config;

        //start update thread
        pool.start_update(tracker_config.clone(), update_period_secs.to_i32().unwrap());

        //start have thread
        pool.start_have(update_period_secs.to_i32().unwrap());

        //start listening thread
        let peer_config = program_const.peer_config;
        pool.start_listening(peer_config);

        let pool_clone = pool.clone();

        display_menu(tracker_config, pool_clone);

        // auto download section for profiling
        /*
            {
        use crate::com::{connect, lookf, receive, seedf, send};
        use crate::respons_handler::{Answer, ExpectList, ExpectOk, ExpectedAnswer};

        use crate::userinput::{choose_file, get_file_names, get_filename, get_filesize};

        use crate::data::{get_buffer_size};
        use crate::db::{set_peer_to_file};


                let filename = "".to_string();
                let op_filesize = "".to_string();
                let look_message = lookf(filename, op_filesize);
                let mut present_files: Answer = Answer::List(Vec::new());
                let mut ret: Answer = Answer::List(Vec::new());
                if let Some(mut stream) = connect(12345, &"jibelibeju.fr") {
                    send(&mut stream, look_message);
                    let response = receive(&mut stream);
                    match ExpectList.check_answer(&response) {
                        Ok(valeur) => {
                            present_files = ExpectList.retrieve_data(response.clone());
                            ret = ExpectList.retrieve_data(response);
                        }
                        Err(valeur) => {
                            error!("{}", valeur);
                        }
                    }
                }

                match present_files {
                Answer::List(metafiles) => {
                    for file in metafiles {
                        let buffmap: Vec<u8> = vec![0; get_buffer_size(&file)];
                        let conf: PeerConfig = PeerConfig::new();
                        set_peer_to_file(conf, file, buffmap);
                    }
                },
                _ => error!("Could not add filelist to db"),
            }


                let result = start_download("30b3f671a7ba2dede25c0e44721da703".to_string(), 12345, "jibelibeju.fr", pool_clone);

                match result {
                    Ok(task_list) => {
                        for task in task_list{
                            pool.add_task(task);
                        }
                    }
                    Err(errors) => {
                        error!("Could not start download : {}", errors);
                    }
                }
                sleep(Duration::from_secs(20));
            }
            */

        //delete pool
        pool.drop();
    } else {
        // config vars
        let num_threads = 1;
        let update_period_secs = 30;

        // multi thread part
        // create pool
        let mut pool: Pool = Pool::new(num_threads.to_i32().unwrap());

        let tracker_config = TrackerConfig::new();

        //start update thread
        pool.start_update(tracker_config.clone(), update_period_secs.to_i32().unwrap());

        //start have thread
        pool.start_have(update_period_secs.to_i32().unwrap());

        //start listening thread
        let peer_config = PeerConfig::new();
        pool.start_listening(peer_config);

        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![get_files_data])
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
    verbose: Option<u8>,
    // nombre de threads
    #[clap(short, long)]
    max_connection: Option<u32>,
    // chemin de la config
    #[clap(short, long)]
    config: Option<String>,
    // taille des chunks de données
    #[clap(short, long)]
    size_chunk: Option<u32>,
    // nombre de chunks par requête getfile
    #[clap(short, long)]
    number_chunkr: Option<u32>,
    // période de mise à jour
    #[clap(short, long)]
    update_period_secs: Option<u32>,
}
struct ProgramConst {
    peer_config: PeerConfig,
    tracker_config: TrackerConfig,
    num_threads: u32,
    update_period_secs: u32,
    size_chunk: u32,
    number_chunk: u32,
    /*
     */
}

fn handle_program_const(args: Args) -> ProgramConst {
    // handle peer config
    let mut peer_config = PeerConfig::new();
    let mut tracker_config = TrackerConfig::new();
    // if user specify a port
    if let Some(port) = args.port {
        peer_config.port = port;
        set_peer_port(port);
    }
    // handle tracker config

    if let Some(tracker) = args.tracker {
        let ip_regex = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
        let port_regex = Regex::new(r"\d{1,5}").unwrap();
        let ip_port_regex = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}").unwrap();
        if ip_port_regex.is_match(&tracker) {
            let mut split = tracker.split(":");
            set_tracker_address(tracker_config.address.clone());
            tracker_config.address = split.next().unwrap().to_string();
            set_tracker_port(tracker_config.port);
            tracker_config.port = split.next().unwrap().parse::<u16>().unwrap();
        } else if ip_regex.is_match(&tracker) {
            set_tracker_address(tracker_config.address.clone());
            tracker_config.address = tracker;
        } else if port_regex.is_match(&tracker) {
            set_tracker_port(tracker_config.port);
            tracker_config.port = tracker.parse::<u16>().unwrap();
        } else {
            error!("Wrong tracker format, please use ip:port or ip or port, using config value")
        }
    }

    // handle config
    let config_path = args.config.unwrap_or("config.ini".to_string());
    info!("Choosen config file : {:?}", config_path);
    set_config_path(config_path.clone());

    // handle number of threads
    let num_threads = args.max_connection.unwrap_or(5);
    info!("Choosen number of threads : {:?}", num_threads);
    // handle update period
    let update_period_secs = args.update_period_secs.unwrap_or(30);
    info!("Choosen update period : {:?}", update_period_secs);
    // handle size of chunk
    let size_chunk = args.size_chunk.unwrap_or(1024);
    info!("Choosen size of chunk : {:?}", size_chunk);
    // handle number of chunk per request
    let number_chunk = args.number_chunkr.unwrap_or(10);
    info!("Choosen number of chunk per request : {:?}", number_chunk);
    ProgramConst {
        peer_config,
        tracker_config,
        num_threads,
        update_period_secs,
        size_chunk,
        number_chunk,
    }
}
