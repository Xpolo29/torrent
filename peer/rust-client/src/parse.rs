use crate::config::{Message}; // Import enum from config

fn parse_answer_ok(buffer: String) {
    if buffer == "ok\r\n" {
        println!("Réponse correcte du tracker");
    } else {
        println!("Erreur lors de l'envoi du message");
    }
}
fn parse_answer_list(buffer: String) {
    println!("Liste des fichiers disponibles: {}", buffer);
}
pub fn parse_answer(buffer: String, message: Message) {
    match message {
        Message::OK => {
            parse_answer_ok(buffer);
        }
        Message::LIST => {
            parse_answer_list(buffer);
        }
    }
}