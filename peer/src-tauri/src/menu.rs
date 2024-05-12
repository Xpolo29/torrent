use crate::back::start_download;
use crate::com::{connect, lookf, receive, seedf, send};
use crate::data::{get_buffer_size, MetaFile, PeerConfig, TrackerConfig};
use crate::db::{add_seed_file_to_db, log_db, set_peer_to_file};
use crate::respons_handler::{Answer, ExpectList, ExpectOk, ExpectedAnswer};
use crate::tasks::EmptyTask;
use crate::threads::Pool;
use crate::userinput::{choose_file, get_file_names, get_filename, get_filesize};
use log::{error, info, trace, debug};
use std::io;
use crate::ProgramConst;

/// Displays a menu to the user and performs actions based on the user's input.
///
/// This function continuously displays a menu to the user with two options: Upload and Download.
/// It reads the user's input and performs the corresponding action.
/// If the user enters an invalid input, it prints an error message and displays the menu again.
///
/// # Arguments
/// * `tracker_config` - A TrackerConfig object containing the tracker's configuration.
/// * `pool` - A Pool object for managing tasks.
pub fn display_menu(ProgramConst: ProgramConst, tracker_config: TrackerConfig, pool: Pool) {
    loop {
        println!("Main Menu");
        println!("1. Upload");
        println!("2. Download");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            // Escape should get back to menu from search, upload and download
            1 => upload_section(tracker_config.port, &tracker_config.address),
            2 => {
                let pool_clone: Pool = pool.clone();
                download_section(tracker_config.port, &tracker_config.address, pool_clone, ProgramConst.length_tcp as usize)
            }
            _ => println!("Invalid input, please enter 1 or 2"),
        }
    }
}

/// Searches for a file on the tracker.
///
/// This function prompts the user for a filename and optional filesize, then sends a LOOK message to the tracker.
/// It then waits for a response from the tracker and checks the response.
/// If the response is valid, it retrieves the data from the response and returns it.
///
/// # Arguments
/// * `tracker_port` - The port number of the tracker.
/// * `tracker_address` - The address of the tracker.
///
/// # Returns
/// * `Answer` - An Answer object containing the search results.
fn search_section(tracker_port: u16, tracker_adress: &str) -> Answer {
    println!("You're in Search");
    let filename = get_filename(io::stdin());
    let op_filesize = get_filesize(io::stdin());
    let look_message = lookf(filename, op_filesize);
    trace!("Prepared message: {}", look_message);
    let mut present_files: Answer = Answer::List(Vec::new());
    let mut ret: Answer = Answer::List(Vec::new());
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
        send(&mut stream, look_message);
        trace!("Message sent waiting for answer");
        let response = receive(&mut stream, 3000);
        trace!("Received {}", response);

        match ExpectList.check_answer(&response) {
            Ok(_) => {
                //info!("{}", valeur);
                present_files = ExpectList.retrieve_data(response.clone());
                ret = ExpectList.retrieve_data(response);
            }
            Err(valeur) => {
                error!("{}", valeur);
            }
        }
    }
    debug!("files retrieved {:?}", present_files);

    match present_files {
        Answer::List(metafiles) => {
            for file in metafiles {
                let buffmap: Vec<u8> = vec![0; get_buffer_size(&file)];
                let conf: PeerConfig = PeerConfig::new();
                set_peer_to_file(conf, file, buffmap);
            }
        }
        _ => error!("Could not add filelist to db"),
    }

    //log_db();

    ret
}

/// Uploads a file to the tracker.
///
/// This function prompts the user for the names of the files they wish to seed.
/// It then creates a MetaFile object for each file and adds them to the database.
/// It then sends a HAVE message to the tracker for each file.
/// If the tracker responds with an OK message, it logs the response and shuts down the connection.
///
/// # Arguments
/// * `tracker_port` - The port number of the tracker.
/// * `tracker_address` - The address of the tracker.
fn upload_section(tracker_port: u16, tracker_adress: &str) {
    let peer_config = PeerConfig::new();
    println!("You're in upload");
    let seeded_files = get_file_names(io::stdin()); // take the files the user wish to seed
    let seeded_files: Vec<MetaFile> = seeded_files
        .into_iter()
        .map(|file| MetaFile::new(file.to_string()))
        .collect(); // Create vector of Metafiles out of the files name

    let seeded_files2 = seeded_files.clone();
    for seed in seeded_files2 {
        add_seed_file_to_db(seed);
    }

    // TODO set the right leeching string
    let seeded_files = seedf(seeded_files, peer_config.port.to_string(), vec!["".to_string()]); // create the message
    trace!("Prepared message: {}", seeded_files);
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
        // connect to the tracker
        send(&mut stream, seeded_files.clone()); // send the message
        /*
        info!(
            "Sending to {}:{} : {}",
            stream.peer_addr().unwrap().ip(),
            stream.peer_addr().unwrap().port(),
            seeded_files.clone()
        );
        */
        trace!("Message sent waiting for answer");
        let response = receive(&mut stream, 3000); // receive the answer
        trace!("Received: {}", response);
        match ExpectOk.check_answer(&response) {
            Ok(_) => {
            }
            Err(valeur) => {
                error!("{}", valeur);
            }
        }
        ExpectOk.shutdown(&mut stream);
    }
}

/// Downloads a file from the tracker.
///
/// This function prompts the user to choose a file to download from the list of available files.
/// It then starts the download process for the chosen file.
/// If the download process returns a list of tasks, it adds each task to the pool.
/// If the download process fails, it prints an error message.
///
/// # Arguments
/// * `tracker_port` - The port number of the tracker.
/// * `tracker_address` - The address of the tracker.
/// * `pool` - A Pool object for managing tasks.
fn download_section(tracker_port: u16, tracker_adress: &str, mut pool: Pool, length_tcp: usize) {
    // -> Result<(), Box<dyn std::error::Error>> {
    // The list of downloadable files should be the result of search section
    // todo!();
    println!("You're in download");
    // display files along with their size
    // if two files are name the same user should be able to choose which one to download
    let file_key = match choose_file(io::stdin(), &search_section(tracker_port, tracker_adress)) {
        Some(hash) => hash.trim().to_string(),
        None => return,
    };
    println!("You chose to download: {}", file_key);
    let pool_clone: Pool = pool.clone();
    let result = start_download(file_key, tracker_port, tracker_adress, pool_clone, length_tcp);

    match result {
        Ok(task_list) => {
            for task in task_list {
                pool.add_task(task);
            }
        }
        Err(errors) => {
            error!("Could not start download : {}", errors);
        }
    }
}
