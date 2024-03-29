use log::{info, warn};
use std::io::{self, BufRead, BufReader, Read, Result, Write};
use std::path::Path;

// for hash - md5
use md5::{Digest, Md5};
use std::fs::File;

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

    let input = input.trim();
    input.to_string()
}
pub fn get_filesize<R: Read>(reader: R) -> String {
    let mut reader = BufReader::new(reader);
    let mut input = String::new();

    print!("Enter the operator and the filesize you wish to search for: (Ex: <\"10\") ");
    io::stdout().flush().unwrap();
    reader.read_line(&mut input).unwrap();

    let criterion = input.trim();
    criterion.to_string()
}
pub fn display_downloadable_files() {
    // use metafiles
    println!("Files available for download:");
    println!("1. file1.txt size 10MB"); // hash 
    println!("2. file2.txt size 20MB");
    println!("3. file3.txt size 30MB");
    println!("4. file4.txt size 40MB");
    println!("5. file5.txt size 50MB");
}

// hash - md5
pub fn get_file_key(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 8192];
    let mut context = Md5::new();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    let result = context.finalize();
    Ok(format!("{:x}", result))
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

    /*
    use tempfile::NamedTempFile;
    #[test]
    async fn test_get_file_key() {
        let mut tmpfile: File = NamedTempFile::new().unwrap().into_file();
        writeln!(tmpfile, "Hello, world!").unwrap();

        let expected_hash = "6cd3556deb0da54bca060b4c39479839";
        let actual_hash = get_file_key(tmpfile.path().to_str().unwrap()).unwrap();

        assert_eq!(expected_hash, actual_hash);
    } */
}
