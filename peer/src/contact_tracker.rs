use std::io::prelude::*;
use std::net::TcpStream;

struct MetaFile {
    filename: String,
    length: usize,
    piece_size: usize,
    key: String,
}

/// Generates an announce message with the current seeding and leeching files.
///
/// This function takes a port number, a list of seeding files, and a list of leeching files.
/// It formats these lists into strings, where each file is represented by its filename,
/// length, piece size, and key for seeding files, and by its hash for leeching files.
/// Files are separated by spaces.
/// It then generates an announce message containing these formatted lists of seeding and leeching files and the port number.
///
/// # Arguments
/// * `port` - A u16 representing the port number.
/// * `seed` - A vector of `MetaFile` representing the seeding files.
/// * `leech` - A vector of strings representing the leeching files.
///
/// # Returns
/// * `String` - The announce message containing the port number and the formatted lists of seeding and leeching files.
fn announce_listen_seed_leech(port: u16, seed: Vec<MetaFile>, leech: Vec<String>) -> String {
    let seed_str = seed.into_iter()
        .map(|file| format!("{} {} {} {}", file.filename, file.length, file.piece_size, file.key))
        .collect::<Vec<String>>()
        .join(" ");
    let leech_str = leech.join(" ");
    format!("< announce listen {} seed [{}] leech [{}]\n> ok", port, seed_str, leech_str)
}

/// Generates a look message with the given criteria.
///
/// This function takes a list of criteria.
/// It formats this list into a string, where each criterion is separated by a space.
/// It then generates a look message containing this formatted list of criteria.
///
/// # Arguments
/// * `criterion` - A vector of strings representing the criteria.
///
/// # Returns
/// * `String` - The look message containing the formatted list of criteria.
fn look(criterion: Vec<String>) -> String {
    let criterion_str = criterion.join(" ");
    format!("< look [{}]\n> list", criterion_str)
}

/// Generates a getfile message with the given key.
///
/// # Arguments
/// * `key` - A string representing the key.
///
/// # Returns
/// * `String` - The getfile message containing the key.
fn getfile(key: String) -> String {
    format!("< getfile {}\n> peers", key)
}

/// Generates an interested message with the given key.
///
/// # Arguments
/// * `key` - A string representing the key.
///
/// # Returns
/// * `String` - The interested message containing the key.
fn interested(key: String) -> String {
    format!("< interested {}\n> have", key)
}

/// Generates a getpieces message with the given key and indices.
///
/// This function takes a key and a list of indices as strings.
/// It formats this list into a string, where each index is separated by a space.
/// It then generates a getpieces message containing this key and the formatted list of indices.
///
/// # Arguments
/// * `key` - A string representing the key.
/// * `indices` - A vector of strings representing the indices.
///
/// # Returns
/// * `String` - The getpieces message containing the key and the formatted list of indices.
fn getpieces(key: String, indices: Vec<String>) -> String {
    let indices_str = indices.join(" ");
    format!("< getpieces {} [{}]\n> data", key, indices_str)
}

// Function that takes a string and an ip address and a port, and send a tcp message with the string to the address and port
fn send_tcp_message(message: String, ip: String, port: u16) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
    stream.write_all(message.as_bytes())?;
    Ok(())
}