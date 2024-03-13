// src/com.rs
use std::io::Write;
use std::net::TcpStream;
enum Message {
    OK,
    LIST,
    ERROR,
    NONE,
}
use crate::config::FileProp;
// Wait for the answer of the tracker
fn goes_well(message: Message, buffer: String) {
    match message {
        Message::OK => {
            println!("The message was sent successfully");
        }
        Message::LIST => {
            println!("The list of files is: {}", buffer);
        }
        Message::ERROR => {
            println!("An error occurred: {}", buffer);
        }
    }
}
fn send_message(msg: String, Message: Message) {
    // Connect to the tracker unwrap function takes the Ok variant of the Result and returns the value inside
    // expect() si le résultat est Err, le programme crash avec le message passé en paramètre
    let mut stream =
        TcpStream::connect("127.0.0.1:7878").expect("Pas réussis à se connecter au tracker");
    stream.write(msg.as_bytes()).unwrap();
    // Listen for the answer
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    goes_well(Message::OK, buffer);
}
// Send port and available files to tracker
pub fn send_port_seed_to_tracker(port: u16, files: Vec<FileProp>) {
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
        "< announce listen {} seed [{}]\r\n",
        port,
        files_string.join(" ")
    );
    send_message(msg);
}
// works if there is one criterion
pub fn send_search_to_tracker(criterions: String) {
    if criterions.is_empty() {
        send_message("< look\r\n".to_string(), Message::LOOK);
    } else {
        send_message(format!("< look [filename=\"{}\"]\r\n", criterions));
    }
}

#[cfg(test)]
mod tests {}
