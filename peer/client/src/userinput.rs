use log::{info, warn};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
pub fn get_file_names<R: Read>(reader: R) -> Vec<String> {
    let mut reader = BufReader::new(reader);
    let mut input = String::new();
    let mut valid_files = Vec::new();

    print!("Enter the file names you wish to upload (separated by spaces): ");
    io::stdout().flush().unwrap();
    reader.read_line(&mut input).unwrap();

    let file_names = input.trim().split_whitespace();

    for file_name in file_names {
        if Path::new(file_name).exists() {
            info!("File {} exists", file_name);
            valid_files.push(file_name.to_string());
        } else {
            warn!("File {} does not exist. Skipping.", file_name);
        }
    }

    valid_files
}
pub fn get_filename<R: Read>(reader: R) -> String {
    let mut reader = BufReader::new(reader);
    let mut input = String::new();

    print!("Enter the file name you wish to search for: ");
    io::stdout().flush().unwrap();
    reader.read_line(&mut input).unwrap();

    let criterion = input.trim();
    criterion.to_string()
}
pub fn get_filesize<R: Read>(reader: R) -> String {
    let mut reader = BufReader::new(reader);
    let mut input = String::new();

    print!("Enter the operator and the filesize you wish to search for: (Ex: <\"10\")");
    io::stdout().flush().unwrap();
    reader.read_line(&mut input).unwrap();

    let criterion = input.trim();
    criterion.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_name_existing_file() {
        let input = b"Cargo.toml";
        let result = get_file_names(&input[..]);
        assert_eq!(result, vec!["Cargo.toml".to_string()]);
    }

    #[test]
    fn test_get_file_name_non_existing_file() {
        let input = b"non_existing_file.txt";
        let result = get_file_names(&input[..]);
        assert_eq!(result, Vec::<String>::new());
    }
}
