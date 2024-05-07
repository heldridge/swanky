use fancy_garbling::circuit::{BinaryCircuit as Circuit, EvaluableCircuit};
use fancy_garbling::{Evaluator, Garbler, WireMod2};
use scuttlebutt::{AesRng, SymChannel};

use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct SharedData {
    data: Arc<Mutex<Vec<u8>>>,
}

impl SharedData {
    fn new(data: Arc<Mutex<Vec<u8>>>) -> Self {
        SharedData { data }
    }

    fn read_inner(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.data.lock().unwrap().as_slice().read(buf)
    }

    fn write_inner(&self, buf: &[u8]) -> io::Result<usize> {
        self.data.lock().unwrap().write(buf)
    }

    fn flush_inner(&self) -> io::Result<()> {
        self.data.lock().unwrap().flush()
    }
}

impl Read for SharedData {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        println!("READING!!");
        self.read_inner(buf)
    }
}

impl Write for SharedData {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        println!("WRITING!!!");
        self.write_inner(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_inner()
    }
}

fn circuit(fname: &str) -> Circuit {
    println!("* Circuit {}", fname);
    Circuit::parse(BufReader::new(File::open(fname).unwrap())).unwrap()
}

fn run_circuit(circ: &mut Circuit, gb_inputs: Vec<u16>, _ev_inputs: Vec<u16>) {
    let rng = AesRng::new();

    // dbg!(circ);

    let sd = Arc::new(Mutex::new(Vec::new()));
    let garbler_data = SharedData::new(sd.clone());
    let evaluator_data = SharedData::new(sd.clone());

    let s_garb = SymChannel::new(garbler_data);
    let s_eval = SymChannel::new(evaluator_data);

    let mut gb = Garbler::<SymChannel<SharedData>, AesRng, WireMod2>::new(s_garb, rng);

    let n_gb_inputs = gb_inputs.len();
    // let n_ev_inputs = ev_inputs.len();

    let (a, b) = gb
        .encode_many_wires(&gb_inputs, &vec![2; n_gb_inputs])
        .unwrap();

    let _res = circ.eval(&mut gb, &a, &[]).unwrap();

    // let mut ev = Evaluator::<SymChannel<SharedData>, WireMod2>::new(s_eval);
    // let res = circ.eval(&mut ev, &a, &[]).unwrap();
    // dbg!(res);
}

fn main() {
    println!("Start");

    let args: Vec<String> = env::args().collect();
    let circuit_file_name = &args[1];

    let gb_inputs: Vec<u16> = vec![0; 1];
    let ev_inputs: Vec<u16> = vec![0; 1];
    let mut circ = circuit(circuit_file_name);

    run_circuit(&mut circ, gb_inputs, ev_inputs);
}
