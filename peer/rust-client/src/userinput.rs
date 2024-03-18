// src/userinput.rs
use crate::network_config::FileProp;
use md5::{Digest, Md5};
use std::fs;
use std::io::{self, Write};
use std::path::Path; // because config is at top level
                     // One port per peer/thread
pub fn get_hash(file: &str) -> io::Result<String> {
    let mut file = fs::File::open(file)?; // Add ? to handle the Result
    let mut hasher = Md5::new(); // Create new hasher
    io::copy(&mut file, &mut hasher)?; // Add ? to handle the Result
    let result = hasher.finalize(); // Use compute instead of finalize
    Ok(format!("{:x}", result)) // format the result in hexadecimal
}
pub fn get_listen_port() -> u16 {
    fn get_listen_port_rec() -> u16 {
        // TODO : let port be a parameter to avoid creation of a new string
        let mut port = String::new();
        // Takes the port number from the user that the peer will use to receive and forward files
        print!(
            "Enter the port number you want to use to receive and forward files from other peers (hit enter to choose a random port): "
        );
        io::stdout().flush().unwrap();
        // Create a new string to store the port number
        // Call the stdin handle to call the read_line method then call the expect method to crash if the System call fails
        io::stdin()
            .read_line(&mut port)
            .expect("Failed to read line");
        // trim to remove whites spaces and generic parse that expect a type with ::
        match port.trim().parse::<u16>() {
            Ok(port) => {
                print!("Choosen port: {port}");
                io::stdout().flush().unwrap();

                return port;
            }
            Err(_) => {
                if port.trim().is_empty() {
                    print!("Random port: {}", "8080");
                    return 8080;
                }
                print!("Not a valid port number. Please enter a valid port number.");
                io::stdout().flush().unwrap();
                return get_listen_port_rec();
            }
        }
    }
    get_listen_port_rec()
}
pub fn get_proposed_files() -> Vec<FileProp> {
    // PRECOND : les fichiers existent
    fn fill_file_prop(file: &str) -> FileProp {
        return FileProp {
            file_name: file.trim().to_string(),
            length: fs::metadata(file).unwrap().len(),
            piece_size: 1024, // taille de bloc constante pour l'instant
            hash: get_hash(file).unwrap(),
        };
    }
    let mut files: Vec<FileProp> = Vec::new();

    print!("\nEnter the files you want to share, separated by white spaces (press Enter if you don't want to share anything) : ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    for file in input.trim().split_whitespace() {
        // Créer un FileProp avec des informations constantes
        match Path::new(file).exists() {
            true => {
                let file_prop = fill_file_prop(file);
                files.push(file_prop);
            }
            false => {
                print!("Le fichier {} n'existe pas", file);
                io::stdout().flush().unwrap();

                continue;
            }
        }
    }

    if files.len() != 0 {
        println!("Here are the files you are sharing : ");
        io::stdout().flush().unwrap();

        for file in files.iter() {
            print!(
                "{} -- taille: {} bytes, taille de bloc: {}, hash: {}",
                file.file_name, file.length, file.piece_size, file.hash
            );
        }
    } else {
        print!("You are not sharing any file.");
    }

    files
}
// return
pub fn get_available_files() -> String {
    let mut criterions = String::new();
    print!("Enter the name of the file you are looking for (press Enter if you want to see the list of all available files) : ");
    io::stdin()
        .read_line(&mut criterions)
        .expect("Pas réussi à lire la ligne");

    return criterions.trim().to_string();
}

pub fn get_file() -> String {
    print!("Enter the name of the file you want to download : ");
    let mut file = String::new();
    io::stdin()
        .read_line(&mut file)
        .expect("Failed to read line");
    return file.trim().to_string();
}
#[cfg(test)]
mod tests {}
