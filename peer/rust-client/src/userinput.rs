// src/userinput.rs
use std::fs;
use std::io;
use std::path::Path;

// because config is a top level one should write use crate::config and not mod config
use crate::config::FileProp;
// One port per peer/thread
pub fn get_listen_port() -> u16 {
    fn get_listen_port_rec() -> u16 {
        // TODO : let port be a parameter to avoid creation of a new string
        let mut port = String::new();
        // Takes the port number from the user that the peer will use to receive and forward files
        println!("Port associé à ce peer? Appuie sur Entrée pour utiliser le port par défaut: 8080");
        // Create a new string to store the port number
        // Call the stdin handle to call the read_line method then call the expect method to crash if the System call fails
        io::stdin()
            .read_line(&mut port)
            .expect("Failed to read line");
        // trim to remove whites spaces and generic parse that expect a type with ::
        match port.trim().parse::<u16>() {
            Ok(port) => {
                println!("Port choisi: {port}");
                return port;
            }
            Err(_) => {
                if port.trim().is_empty() {
                    println!("Port vide, utilisation du port par défaut: 8080");
                    return 8080;
                }
                println!("Port invalide, veuillez réessayer.");
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
            hash: "hash_constant".to_string(), // hash constant pour l'instant
        };
    }
    let mut files: Vec<FileProp> = Vec::new();

    println!(
        "Quels fichiers veux-tu envoyer? (séparés par des espaces blancs) tape Entree si tu ne veux rien partager"
    );
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
                println!("Le fichier {} n'existe pas", file);
                continue;
            }
        }
    }

    if files.len() != 0 {
        println!("Tu veux télécharger les fichiers suivants:");
        for file in files.iter() {
            println!(
                "{} -- taille: {} bytes, taille de bloc: {}, hash: {}",
                file.file_name, file.length, file.piece_size, file.hash
            );
        }
    } else {
        println!("Pas de fichiers à partager.");
    }

    files
}
// return
pub fn get_available_files() -> String {
    let mut criterions = String::new();
    println!("Quel fichier cherches-tu? Appuie sur Entrée pour voir tous les fichiers disponibles");
    io::stdin()
        .read_line(&mut criterions)
        .expect("Pas réussi à lire la ligne");

    return criterions.trim().to_string();
}

#[cfg(test)]
mod tests {}
