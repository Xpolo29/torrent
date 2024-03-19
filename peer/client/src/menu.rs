use std::io;
pub fn display_menu() {
    loop {

        println!("Main Menu");
        println!("1. Search");
        println!("2. Upload");
        println!("3. Download");
        
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match input {
            1 => search_section(),
            2 => upload_section(),
            3 => download_section(),
            _ => println!("Invalid input, please enter 1, 2 or 3"),
        }
        
    }
    }
    fn search_section() 
    {
        println!("You're in Search")
    }
fn upload_section() {
    println!("You're in upload")
}
fn download_section() {
    println!("You're in download")
}