fn main() {
    let machaine = "12  3";
    let nouvellechaine: Result<i32, _> = machaine.trim()
    match nouvellechaine {
        Ok(n) => println!("{}", n),
        Err(_) => println!("Erreur"),
    }
}
