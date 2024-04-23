use std::net::TcpStream;
use std::collections::HashMap;
use regex::Regex;
use 

fn main() {
    let re = Regex::new(r"^(data) ([[:alnum:]]*) \\[((?:[[:digit:]]*:[01]* ?)*)\\]$").unwrap();
    let request = String::from("8905e92afeb80fc7722ec89eb0bf0966 [3:%piece3% 5:%piece5% 7:%piece7% 8:%piece8% 9:%piece9%]");
    let stream = TcpStream::connect("127.0.0.1:9090");

    let task = data_request(re, request, stream);
    task.process();
}