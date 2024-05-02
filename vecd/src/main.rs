use std::env;
use std::{fs::File, io::BufReader};

use fancy_garbling::circuit::BinaryCircuit as Circuit;

fn circuit(fname: &str) -> Circuit {
    println!("* Circuit {}", fname);
    Circuit::parse(BufReader::new(File::open(fname).unwrap())).unwrap()
}

fn main() {
    println!("Start");

    let args: Vec<String> = env::args().collect();
    let circuit_file_name = &args[1];

    let mut _circ = circuit(circuit_file_name);
}
