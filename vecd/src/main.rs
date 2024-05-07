use fancy_garbling::circuit::BinaryCircuit;
use fancy_garbling::classic::garble;
use fancy_garbling::WireMod2;

use std::env;
use std::fs::File;
use std::io::BufReader;

use serde_json;

fn circuit(fname: &str) -> BinaryCircuit {
    BinaryCircuit::parse(BufReader::new(File::open(fname).unwrap())).unwrap()
}

fn garble_circuit(circ: &mut BinaryCircuit, gb_inputs: Vec<u16>) {
    let (enc, gc) = garble::<WireMod2, _>(circ).unwrap();

    let garbler_inputs = enc.encode_garbler_inputs(&gb_inputs);
    let evaluator_zero_inputs = enc.encode_evaluator_inputs(&vec![0; 2]);
    let evaluator_one_inputs = enc.encode_evaluator_inputs(&vec![1; 2]);

    println!("{}", serde_json::to_string(&garbler_inputs).unwrap());
    println!("{}", serde_json::to_string(&evaluator_zero_inputs).unwrap());
    println!("{}", serde_json::to_string(&evaluator_one_inputs).unwrap());
    println!("{}", serde_json::to_string(&gc).unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(directive) = args.get(1) {
        if directive == "garble" {
            if let Some(circuit_path) = args.get(2) {
                let mut circ = circuit(&circuit_path);

                let gb_inputs: Vec<u16> = args[3..]
                    .iter()
                    .map(|s| s.parse::<u16>().unwrap())
                    .collect();

                garble_circuit(&mut circ, gb_inputs);
            } else {
                println!("Must provide a circuit path")
            }
        } else if directive == "evaluate" {
            println!("Evaluating");
        } else {
            println!("Neither");
        }
    }
}
