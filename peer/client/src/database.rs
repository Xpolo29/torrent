pub fn number_to_file_name(number: u32) -> String {
    format!("file{}.txt", number)
}
pub fn get_peers(key: String) -> Vec<(String, u16)> {
    vec![("1.1.1.1".to_string(), 2222), ("1.1.1.2".to_string(), 3333)]
}
pub fn get_buffermap(key: String) -> Vec<u8> {
    unimplemented!()
}
