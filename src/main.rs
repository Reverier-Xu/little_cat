use std::env;
use std::io;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = String::new();
    if args.len() > 1 {
        filename = args[1].clone();
    } else {
        io::stdin().read_line(&mut filename).unwrap();
    }
    let mut file = std::fs::File::open(filename).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
    loop {
        let mut buffer = [0; 1024];
        let n = file.read(&mut buffer).expect("Error in reading file");
        if n == 0 {
            break;
        }
        io::stdout().write(&buffer[..n]).unwrap();
    }
}
