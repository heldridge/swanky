use fancy_garbling::circuit::BinaryCircuit;
use fancy_garbling::classic::garble;
use fancy_garbling::{WireLabel, WireMod2};

use std::env;
use std::fs::File;
use std::io::BufReader;

fn circuit(fname: &str) -> BinaryCircuit {
    BinaryCircuit::parse(BufReader::new(File::open(fname).unwrap())).unwrap()
}

fn run_circuit(circ: &mut BinaryCircuit, gb_inputs: Vec<u16>) {
    let (enc, _gc) = garble::<WireMod2, _>(circ).unwrap();

    let garbler_inputs = enc.encode_garbler_inputs(&gb_inputs);

    let encoder_zero_inputs = enc.encode_evaluator_inputs(&vec![0; 2]);
    let encoder_one_inputs = enc.encode_evaluator_inputs(&vec![1; 2]);

    for gi in garbler_inputs.iter() {
        print!("{} ", gi.as_block());
    }

    println!("");
    for ei_z in encoder_zero_inputs.iter() {
        print!("{} ", ei_z.as_block());
    }

    println!("");
    for ei_o in encoder_one_inputs.iter() {
        print!("{} ", ei_o.as_block());
    }
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let circuit_file_name = &args[1];

    let gb_inputs: Vec<u16> = vec![1; 2];
    let mut circ = circuit(circuit_file_name);

    run_circuit(&mut circ, gb_inputs);
}
