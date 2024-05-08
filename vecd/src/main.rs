use fancy_garbling::circuit::BinaryCircuit;
use fancy_garbling::classic::{garble, GarbledCircuit};
use fancy_garbling::WireMod2;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

use serde_json;

fn circuit(fname: &str) -> BinaryCircuit {
    BinaryCircuit::parse(BufReader::new(File::open(fname).unwrap())).unwrap()
}

fn garble_circuit(circ: &mut BinaryCircuit, gb_inputs: Vec<u16>) {
    let (enc, gc) = garble::<WireMod2, _>(circ).unwrap();
    let num_inputs = gb_inputs.len();

    let garbler_inputs = enc.encode_garbler_inputs(&gb_inputs);
    let evaluator_zero_inputs = enc.encode_evaluator_inputs(&vec![0; num_inputs]);
    let evaluator_one_inputs = enc.encode_evaluator_inputs(&vec![1; num_inputs]);

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
            if let Some(circuit_path) = args.get(2) {
                let circ = circuit(&circuit_path);

                let ell = args.get(3).unwrap().parse::<usize>().unwrap();
                let input_data_path = args.get(4).unwrap();

                let file = File::open(input_data_path).unwrap();
                let reader = io::BufReader::new(file);

                let mut gb_inputs: Vec<WireMod2> = Vec::new();
                let mut ev_inputs: Vec<WireMod2> = Vec::new();

                for (index, line) in reader.lines().enumerate() {
                    match line {
                        Ok(line_content) => {
                            if index < ell {
                                gb_inputs.push(serde_json::from_str(&line_content).unwrap());
                            } else if index < 2 * ell {
                                ev_inputs.push(serde_json::from_str(&line_content).unwrap());
                            } else {
                                let gc: GarbledCircuit<WireMod2, BinaryCircuit> =
                                    serde_json::from_str(&line_content).unwrap();

                                let res = gc.eval(&circ, &gb_inputs, &ev_inputs).unwrap()[0];
                                println!("{}", res)
                            }
                        }
                        Err(_err) => panic!("Failed to read line"),
                    }
                }
            } else {
                println!("Must provide a circuit path")
            }
        } else {
            println!("Neither");
        }
    }
}
