use crate::com::{connect, getfilef, receive, seedf, send};
use crate::data::{MetaFile, PeerConfig};
use crate::db::{
    get_buffermap, get_file, get_leeching_files, get_peer, get_peer_key, get_peers_from_file,
    get_seeding_files, set_buffermap,
};
use crate::respons_handler::{Answer, ExpectOk, ExpectPeers, ExpectedAnswer};
use crate::tasks::{Have, Peer};
use crate::threads::Pool;
use log::{debug, error, info, trace};
use md5::digest::block_buffer::Error;
use rayon::prelude::*;
use std::cmp::min;
use std::collections::BinaryHeap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Seek, SeekFrom, Write};
use std::net::TcpStream;
use std::sync::Mutex;

pub fn is_stream_open(stream: &TcpStream) -> bool {
    trace!("checking if stream is open");
    match stream.take_error() {
        Ok(Some(err)) => false,
        Ok(None) => true,
        Err(err) => false,
    }
}

/// Starts the download process for a file.
///
/// This function connects to a tracker, sends a request for the file, and receives a response.
/// It then checks the response and retrieves the peers that hold the file.
/// For each peer, it creates a new task and adds it to a vector of tasks.
///
/// # Arguments
/// * `key` - A string that holds the key of the file to be downloaded.
/// * `tracker_port` - A u16 that represents the port of the tracker.
/// * `tracker_adress` - A string slice that holds the address of the tracker.
///
/// # Returns
/// * `Result<Vec<Box<Peer>>, Error>` - A Result which is either:
///     * `Ok(Vec<Box<Peer>>)` - A vector of boxed tasks if the operation is successful.
///     * `Err(Error)` - An error if the operation fails.
pub fn start_download(
    key: String,
    tracker_port: u16,
    tracker_adress: &str,
    pool: Pool,
    length_tcp: usize,
) -> Result<Vec<Box<Peer>>, Error> {
    // extract the meta data from the file
    // let meta_file = get_file(&key).unwrap();
    // let chunk_size = meta_file.piece_size;
    // get the peers thare hold buffermap for the file
    if let Some(mut stream) = connect(tracker_port, &tracker_adress) {
        let getfile_message = getfilef(key);

        // send getfile
        send(&mut stream, getfile_message);

        // get answer
        let response = receive(&mut stream, 3000);
        // check if answer is valid
        match ExpectPeers.check_answer(&response) {
            Ok(valeur) => {
                // peers that hold each buffermap
                // Here retrieve peers from correct answer
                let peers: Answer = ExpectPeers.retrieve_data(valeur);

                // now we should ask each peer for their buffermap that is a task

                // say that i am downloading it
                let seeded_files: Vec<MetaFile> = get_seeding_files();
                let mut leeching_files_strings: Vec<String> = Vec::new();
                let leeching_files: Vec<MetaFile> = get_leeching_files();
                for leech in leeching_files {
                    leeching_files_strings.push(leech.hash);
                }
                let mut stream: TcpStream = connect(tracker_port, &tracker_adress).unwrap();
                let message = seedf(
                    seeded_files,
                    PeerConfig::new().port.to_string(),
                    leeching_files_strings,
                ); // create the message
                send(&mut stream, message.clone());
                debug!("OPTION -p Sent: {}", message);
                let response = receive(&mut stream, 3000); // receive the answer
                trace!("Received: {}", response);
                match ExpectOk.check_answer(&response) {
                    Ok(_) => {}
                    Err(valeur) => {
                        error!("{}", valeur);
                    }
                }

                match peers {
                    Answer::Peers(peers) => {
                        let mut tasks = Vec::new();
                        for mut peer in peers {
                            // retrieve data init pool with an empty one
                            // so we need to overwrite it
                            let pool: Pool = pool.clone();
                            peer.pool = pool;
                            peer.length_tcp = length_tcp;
                            tasks.push(Box::new(peer));
                        }
                        Ok(tasks)
                    }
                    _ => {
                        error!("couldn't retrieve peers from tracker");
                        Err(Error)
                    }
                }
            }
            Err(valeur) => {
                error!("{}", valeur);
                Err(Error)
            }
        }
    } else {
        Err(Error)
    }
}

/// Retrieves the specified chunks from a file.
///
/// This function iterates over a vector of chunk indices, retrieves each chunk from the file,
/// and stores it in a HashMap where the key is the chunk index and the value is the chunk data.
///
/// # Arguments
/// * `key` - A string that holds the key of the file.
/// * `chunk_size` - A u32 that represents the size of each chunk.
/// * `chunk_array` - A vector of u32s that represents the indices of the chunks to be retrieved.
///
/// # Returns
/// * `HashMap<u32, Vec<u8>>` - A HashMap where the key is the chunk index and the value is the chunk data.
pub fn get_chunks_from_file(
    key: String,
    chunk_size: usize,
    chunk_indexes: &Vec<usize>,
) -> Vec<(usize, Vec<u8>)> {
    // get filepath
    let meta_file = get_file(&key)
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ))
        .unwrap();
    let file_path = &meta_file.file_name;

    // open the file only once to reduce io
    let file: File = File::open(file_path).unwrap();

    // cycle throught all the chunks
    let mut chunks: Vec<(usize, Vec<u8>)> = Vec::new();
    for chunk_index in chunk_indexes {
        let chunk_index: usize = chunk_index.clone();
        let chunk = get_chunk(file.try_clone().unwrap(), chunk_size, chunk_index).unwrap();
        chunks.push((chunk_index, chunk));
    }
    let ret: Vec<(usize, Vec<u8>)> = chunks.clone();
    ret
}

/// Retrieves a specific chunk from a file.
///
/// This function opens the file, seeks to the start of the specified chunk, reads the chunk into a buffer, and returns the buffer.
///
/// # Arguments
/// * `file_path` - A string slice that holds the path of the file.
/// * `chunk_size` - A u32 that represents the size of each chunk.
/// * `chunk_index` - A u32 that represents the index of the chunk to be retrieved.
///
/// # Returns
/// * `std::io::Result<Vec<u8>>` - A Result which is either:
///     * `Ok(Vec<u8>)` - A vector of bytes representing the chunk if the operation is successful.
///     * `Err(std::io::Error)` - An error if the operation fails.
fn get_chunk(mut file: File, chunk_size: usize, chunk_index: usize) -> std::io::Result<Vec<u8>> {
    let start = chunk_size * chunk_index;
    file.seek(SeekFrom::Start(start as u64))?;
    let mut buffer = vec![0; chunk_size as usize];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}

// take a have task and update buffermap of file
pub fn store_have_to_db(peer: PeerConfig, have: Have) {
    let file_key: String = have.key;
    let peer_key: String = get_peer_key(peer);
    let buffermap: Vec<u8> = have.buffermap;

    set_buffermap(file_key, peer_key, buffermap);
}

static LOCK: Mutex<()> = Mutex::new(());

// can be optimised ? without computation -> double the download speed
pub fn get_wanted_piece_from_peer(peer_key: &str, file_key: &str, nb_pieces: usize) -> Vec<usize> {
    // to allow only one thread at a time here
    let _guard = LOCK.lock().unwrap();

    let file_key_clone = String::from(file_key);
    let peers: Vec<PeerConfig> = get_peers_from_file(file_key_clone);
    let me: PeerConfig = PeerConfig::new();

    let me_clone = me.clone();
    let distant_peer: Mutex<PeerConfig> = Mutex::new(me_clone.clone());

    let mut buffmaps: Vec<Vec<u8>> = Vec::new();

    // cycle through all peers to get their buffmap
    for peer in peers {
        //peers.par_iter().for_each( |peer| {
        if peer_key == get_peer_key(peer.clone()) {
            *distant_peer.lock().unwrap() = peer.clone();
        }
        let file_key_clone = file_key;
        let option: Option<Vec<u8>> = get_buffermap(peer.clone(), file_key_clone);
        match option {
            Some(buffmap) => buffmaps.push(buffmap),
            None => {}
        }
    } //);

    let distant_peer: PeerConfig = distant_peer.lock().unwrap().clone();

    if distant_peer.address == me.address && distant_peer.port == me.port {
        error!("Can't detect who is the distant peer");
        let ret: Vec<usize> = Vec::new();
        return ret;
    }

    // get distant peer buffmap
    let distant_buffmap: Vec<u8> = get_buffermap(distant_peer, file_key).unwrap();

    // now we need to calculate the rarest parts scores
    //let buffmaps_clone = buffmaps.clone();

    let mut main_buffmap: Vec<u8>;
    match get_buffermap(PeerConfig::new(), file_key) {
        Some(arr) => main_buffmap = arr,
        None => main_buffmap = vec![0 as u8; buffmaps[0].len()],
    }
    let len: usize = main_buffmap.len();
    let mut scores: Vec<usize> = vec![0; len];
    let mut remaining: usize = 0;

    let mut done: usize = 0;
    for e in &main_buffmap {
        if *e == 1 {
            done += 1;
        }
    }

    for i in 0..len {
        if main_buffmap[i] == 0 {
            remaining += 1;
            for buffmap in &buffmaps {
                if (buffmap[i] + distant_buffmap[i]) == 2 {
                    scores[i] += 1;
                }
            }
        }
    }

    // now we have a score per index
    // we just need to the get lowest scores
    let nb_pieces: usize = min(nb_pieces, remaining);

    // now get the nb_pieces smallest score index
    let mut ret: Vec<usize> = Vec::new();
    let mut heap = BinaryHeap::new();

    for (i, &score) in scores.iter().enumerate() {
        if main_buffmap[i] == 0 {
            heap.push((score, i));
        }
        if heap.len() > nb_pieces {
            heap.pop();
        }
    }

    for _ in 0..nb_pieces {
        let (_, index) = heap.pop().unwrap();
        ret.push(index);
    }

    ret.reverse();

    info!(
        "Dowloaded: {} chunks over {} ({}%)",
        done,
        len,
        ((done as f64) / (len as f64) * 100.0) as u64
    );
    trace!("Rarest parts are {:?}", ret);

    for piece in ret.clone() {
        main_buffmap[piece] = 1;
    }

    set_buffermap(file_key.to_string(), get_peer_key(me_clone), main_buffmap);

    ret
}

pub struct FileAssembler {
    file: std::fs::File,
    chunk_size: u32,
}
impl FileAssembler {
    pub fn new(file_path: &str, chunk_size: u32) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        Ok(Self { file, chunk_size })
    }

    pub fn add_chunk(&mut self, index: u32, data: Vec<u8>) -> std::io::Result<()> {
        self.file
            .seek(SeekFrom::Start((index * self.chunk_size) as u64))?;
        self.file.write_all(&data)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_chunk() -> std::io::Result<()> {
        // Create a test file with known content
        let file_path = "test_file.txt";
        let mut file = File::create(file_path)?;
        file.write_all(b"Hello, world!")?;

        // Read the first chunk from the file
        let chunk = get_chunk(file, 5, 0)?;

        // Check that the chunk content is correct
        assert_eq!(chunk, b"Hello");

        Ok(())
    }
}
