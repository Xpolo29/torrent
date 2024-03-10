use std::io;

// Définir la structure FileProp
struct Config {
    port: u16,
    files: Vec<FileProp>
}
struct FileProp {
    name: String,
    size: u64,
    block_size: u64,
    hash: String,
}


fn announce_listen_port() -> u16 {
        println!("Quel port veux-tu écouter? (appuie sur Entrée pour utiliser le port par défaut)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Nul ce serait mieux si c'était de base dans la struct config mais je suis fatigué
        if input.trim().is_empty() {
            return 8080; // Use default port if input is empty
        }

        let port: u16 = input.trim().parse().expect("Please type a number!");
        port

}
fn announce_files()() -> Vec<FileProp> {
    let mut files: Vec<FileProp> = Vec::new();

    println!("Quels fichiers veux-tu envoyer? (séparés par des virgules, ou tape 'q' pour quitter)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim() == "q" {
        println!("Aucun fichier sélectionné.");
        return files;
    }

    for file in input.trim().split(',') {
        // Créer un FileProp avec des informations constantes
        let file_prop = FileProp {
            name: file.trim().to_string(),
            size: 1024, // taille constante pour l'instant
            block_size: 1024, // taille de bloc constante pour l'instant
            hash: String::from("hash_constant"), // hash constant pour l'instant
        };
        files.push(file_prop);
    }

    println!("Tu veux télécharger les fichiers suivants:");
    for (index, file) in files.iter().enumerate() {
        println!("fichier{}: {}, taille: {}, taille de bloc: {}, hash: {}", index + 1, file.name, file.size, file.block_size, file.hash);
    }

    files
}

fn main() {
   let mut config = Config {
       port: 8080,
       files: Vec::new(),
   };
   // nul si le port change pas et en plus ça marche pas
   Config.port = announce_listen_port();
   Config.files = announce_files();
   // Idee faire des mocks du server en simulant les echanges TCP avec des réponses constantes

}