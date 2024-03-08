use std::io;

// Définir la structure FileProp
struct FileProp {
    name: String,
    size: u64,
    block_size: u64,
    hash: String,
}

fn get_proposed_files() -> Vec<FileProp> {
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
   let files = get_proposed_files();
   // Utilisez `files` ici si nécessaire
}