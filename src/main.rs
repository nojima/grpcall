use clap::{Arg, Command};

fn main() {
    let matches = Command::new("grpcall")
        .arg(Arg::new("addr").index(1).required(true))
        .arg(Arg::new("method").index(2).required(true))
        .get_matches();

    println!("addr   = {:?}", matches.value_of("addr").unwrap());
    println!("method = {:?}", matches.value_of("method").unwrap());
}
