use std::io;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use std::fs;
// Définir la structure FileProp
struct Config {
    port: u16,
    files: Vec<FileProp>,
}
struct FileProp {
    file_name: String,
    length: u64,
    piece_size: u64,
    hash: String,
}
// One port per peer/thread
fn get_listen_port() -> u16 {
    fn get_listen_port_rec() -> u16 {
        // TODO : let port be a parameter to avoid creation of a new string
        let mut port = String::new();
        // Takes the port number from the user that the peer will use to receive and forward files
        println!("Quel port veux-tu écouter? (appuie sur Entrée pour utiliser un port random)");
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
fn get_proposed_files() -> Vec<FileProp> {
    // PRECOND : les fichiers existent
    fn fill_file_prop(file: &str) -> FileProp{
                return FileProp {
                    file_name: file.trim().to_string(),
                    length: fs::metadata(file).unwrap().len(),    
                    piece_size: 1024,                  // taille de bloc constante pour l'instant
                    hash: "hash_constant".to_string(), // hash constant pour l'instant
                };            

    }
    let mut files: Vec<FileProp> = Vec::new();

    println!(
        "Quels fichiers veux-tu envoyer? (séparés par des espaces blancs) tape Entree si tu ne veux rien partager sale leecher de merde."
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
                "{} -- taille: {}, taille de bloc: {}, hash: {}",
                file.file_name,
                file.length,
                file.piece_size,
                file.hash
            );
        }
    } else {
        println!("Pas de fichiers à partager.");
    }

    files
}

// Send port and available files to tracker
fn send_port_seed_to_tracker(port: u16, files: Vec<FileProp>) {
    // Connect to the tracker unwrap function takes the Ok variant of the Result and returns the value inside
    // expect() si le résultat est Err, le programme crash avec le message passé en paramètre
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Pas réussis à se connecter au tracker");
    /*
    into_iter() : transform the vector into an iterator
    map() : apply a function to each element of the iterator
    collect() : transform the iterator into a vector
    join() : concatenate the elements of the vector into a single string
     */
    let files_string: Vec<String> = files
        .into_iter()
        .map(|file| {
            format!(
                "{} {} {} {}",
                file.file_name, file.length, file.piece_size, file.hash
            )
        })
        .collect();

    let msg = format!(
        "< announce listen {} seed [{}]",
        port,
        files_string.join(" ")
    );
    stream.write(msg.as_bytes()).unwrap();
}
fn main() {
    let mut config = Config {
        port: 8080,
        files: Vec::new(),
    };
    // nul si le port change pas et en plus ça marche pas
    config.port = get_listen_port();
    config.files = get_proposed_files();
    send_port_seed_to_tracker(config.port, config.files);
    // Idee faire des mocks du server en simulant les echanges TCP avec des réponses constantes
}
