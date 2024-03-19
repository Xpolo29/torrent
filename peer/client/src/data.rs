use std::path::Path;
pub struct MetaFile {
    pub file_name: String,
    pub length: u64,
    pub piece_size: u64,
    pub hash: String,
}
impl MetaFile {
    pub fn new(file_name: String) -> Self {
        let path = Path::new(&file_name);
        let length = path.metadata().unwrap().len();
        MetaFile {
            file_name,
            length,
            piece_size: 1024, 
            hash: "hash".to_string(), 
    }
}
}
pub struct PeerConfig {
    pub address: String,
    pub port: i64,
    pub seeded_file: Vec<MetaFile>,
}

pub struct TrackerConfig {
    pub address: String,
    pub port: i64,
}

