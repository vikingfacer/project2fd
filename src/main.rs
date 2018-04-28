extern crate csv;


use std::io::{self, Read, stdin};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::ffi::OsString;
use std::env;


#[derive(Debug)]
struct Qstate {
    name: String,
    transition: HashMap<String, String>,
    isfinal: bool

}

fn build_qstate(n : String, trans : HashMap<String, String>) -> Qstate{

	let finn : bool =
	match n.find('f') {
		None => false,
		_ => true
	};

	Qstate{
		name : n,
		transition : trans,
		isfinal : finn
	}
}

type Record = HashMap<String, String>;

fn read_tm_from_file(file : File) -> Result<HashMap<String, Qstate>, Box<Error>> {
	
    let mut rdr = csv::Reader::from_reader(file);
	let mut states: HashMap<String, Qstate> = HashMap::new();    
    
    for result in rdr.deserialize() {
    
        let mut record: Record = result?;
        let state = record.remove("states").unwrap();
        states.insert(state.clone(), build_qstate(state, record ));
    }

    Ok(states)
}

fn get_tape() -> String{
	let mut buffer = String::new();
	match io::stdin().read_line(&mut buffer){
		Ok(n) => println!("{} bytes read", n),
	    Err(error) => println!("error: {}", error),
	}

	buffer
}

fn translate(mut tape : String, machine : HashMap<String, Qstate>, tape_index: u64){

}

fn main() {
	//  get file from args
	let file_path  =  env::args_os().nth(1).unwrap();
	let file = File::open(&file_path).unwrap();

	//  create states from file
	let states : HashMap<String, Qstate> = read_tm_from_file(file).unwrap();

	// get the tape
	let mut tape = get_tape();
	println!("{:?}", tape);

	loop {
	    // this is where the code to translate 
	    // 
	}

}
















