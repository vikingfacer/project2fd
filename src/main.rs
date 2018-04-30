extern crate csv;


use std::io::{self};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::env;


#[derive(Debug)]
struct TuringMachine {
    states: Vec<String>,
    transition_func : HashMap<String, Qstate>,
    language : Vec<String>,
    tape : Vec<u8>,
    q0 : Qstate,
    qa : Vec<Qstate>,
    qr : Vec<Qstate>
}


#[derive(Debug)]
struct Qstate{
    name: String,
    transition: HashMap<String, Vec<String>>,
    isfinal: bool

}

fn build_qstate(n : String, trans : HashMap<String, String>) -> Qstate{

	let finn : bool =
	match n.find('f') {
		None => false,
		_ => true
	};
	let mut map : HashMap<String, Vec<String>>= HashMap::new();
	for (k, v) in trans.iter(){
		let mut moves :Vec< String> = Vec::new();
		for st in v.split_whitespace(){

			moves.push(String::from(st));
		}
		map.insert(k.clone(), moves);
	}
	Qstate{
		name : n,
		transition : map,
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

fn get_tape() -> Vec<u8>{
	let mut buffer = String::new();
	match io::stdin().read_line(&mut buffer){
		Ok(n) => println!("{} bytes read", n),
	    Err(error) => println!("error: {}", error),
	}

	buffer.replace("\\n", "");
	print!("{:?}",buffer );
	Vec::from(buffer.into_bytes())
}


// fn translate(mut tape : String, machine : HashMap<String, Qstate>, tape_index: u64){

// }

fn main() {
	//  get file from args
	let file_path  =  env::args_os().nth(1).unwrap();
	let file = File::open(&file_path).unwrap();

	//  create states from file
	let _states : HashMap<String, Qstate> = read_tm_from_file(file).unwrap();

	// get the tape
	let mut tape = get_tape();
	println!("{:?}", tape);

	let mut tape_index : usize = 0;
	let mut cur_state = match _states.get("q1"){
		Some(n) => n,
		None => return
	};

	let  mut input = tape[tape_index] as char;

	loop {
	    // this is where the code to translate 

	    let moves = cur_state.transition.get(&input.to_string()).unwrap();

	    println!("{:?} {:?} ",input, cur_state);
	    for thing in moves{
	    	println!("{:?}", thing);
	    }

	    if moves[1] == "R" {
	    	tape_index +=1;
	    }
	    else if moves[1] == "L" {
	        tape_index -= 1;
	    }

	    tape[tape_index] = moves[0].as_bytes()[0];

	    if tape.len() == tape_index {
	    	tape.push('_' as u8);

	    }else{
	    	input = tape[tape_index] as char;
	    }

	    // break
	    if  moves.len() == 0{
	    	break;
	    }else {
	        cur_state = _states.get(&moves[2]).unwrap();
	    }
	}
	for item in tape {
		print!("{:?}",item as char);
	}
}
// the hard part of this assignment is managing the tape























