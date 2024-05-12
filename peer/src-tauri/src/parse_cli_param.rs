use clap::Parser;
/*
-p, --port : port dans peer config
-t, --t : adresse:port | adresse (va prendre port de la config) (du tracker)
-v, --verbose : niveau de debug
-m,--max_connection : nombre de threads
-h, --help :
-c, --config : precise le chemin de la config
-s, - - size_chunk // size of a chunk
-n, - - number_chunk //number of per getfile request
*/
struct Args {
    #[arg(short, long)]
    port: u16, // port d'écoute du peer

    #[arg(short, long)] // adresse et/ou port du tracker
    tracker: String,
}
fn main() {
    let args = Args::parse();
    println!("Port: {}", args.port);
    println!("Tracker: {}", args.tracker);
}
