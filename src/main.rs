extern crate csv;


use std::io::{self};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::env;
use std::process::exit;

#[derive(Debug)]
struct TuringMachine {
    states: Vec<String>,
    transition_func : HashMap<String, Qstate>,
    language : Vec<String>,
    tape : Vec<u8>,
    q0 : Qstate,
    qa : Qstate,
    qr : Qstate
}


#[derive(Debug, Clone)]
struct Qstate{
    name: String,
    transition: HashMap<String, NextMove>,
    isfinal: bool

}

#[derive(Debug, Clone)]
struct NextMove {
    state : String,
    dir : Direction,
    write : String
}
#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Nowhere
}

use Direction::*;

fn build_NextMove(s : String, d : Direction, w : String) -> NextMove{
	NextMove{
		state : s,
		dir : d,
		write : w
	}
}

fn build_direction(d : &String) -> Direction{
	match d.to_lowercase().as_ref() {
		"lt" => Left,
		"rt" => Right,
		"r" => Right,
		"l" => Left,
		_ => Nowhere
	}
}

fn build_qstate(n : String, trans : HashMap<String, String>) -> Qstate{

	let finn : bool =
	match n.find('f') {
		None => false,
		_ => true
	};
	
	let mut map : HashMap<String, NextMove> = HashMap::new();


	for (k, v) in trans.iter(){
		let mut moves :Vec< String> = Vec::new();
		let mut nm_temp = build_NextMove(String::new(), Nowhere, String::new());

		for st in v.split_whitespace(){

			moves.push(String::from(st));
		}
		// if the value is empty then reject
		match moves.len() {
			0 => nm_temp.state = String::from("reject"),

			2 => {nm_temp.dir = build_direction(moves.get(0).unwrap());
			 	  nm_temp.state = moves.get(1).unwrap().to_string();},

			3 => {nm_temp.write = moves.get(0).unwrap().to_string(); 
				  nm_temp.dir = build_direction(moves.get(1).unwrap());
				  nm_temp.state = moves.get(2).unwrap().to_string();}

			_ => {println!("error in file need none, 2 or 3"); exit(0x1);},
		}

		map.insert(k.clone(), nm_temp);
	}

	// println!("{:?}",map );

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
        states.insert(state.clone(), build_qstate(state, record.clone() ));
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
	let mut v = Vec::from(buffer.into_bytes());

	v.insert(0, '_' as u8);
	v.pop();
	v.push('_' as u8);
	v
}

fn get_input() ->String{
	let mut buffer = String::new();
	match io::stdin().read_line(&mut buffer){
		Ok(n) => println!("{} bytes read", n),
		Err(error) => println!("error: {}", error),
	}
	buffer
}

fn main() {
	//  get file from args
	let file_path  =  env::args_os().nth(1).unwrap();
	let file = File::open(&file_path).unwrap();

	//  create states from file
	let _states : HashMap<String, Qstate> = read_tm_from_file(file).unwrap();
	
	// get the start state
	println!("type in start state");
	let mut start_state = get_input();	
	start_state.pop();

	// get the accept state
	println!("type in accept_state");
	let mut accept_state = get_input();
	accept_state.pop();

	// get the tape
	println!("type in tape");
	let mut tape = get_tape();
	println!("{:?}", tape);

	// get all states
	let mut state_set : Vec<String> = Vec::new();
	for k in _states.keys(){
		state_set.push(k.clone());
		println!("{:?}",k);
	}
	println!("{:?}",_states );

	// get language 
	let mut language :Vec<String> = Vec::new();
	for x in _states.get(&start_state).unwrap().transition.keys(){
		language.push(x.to_string());
	}

	let tm = TuringMachine{
		states : state_set,
    	transition_func : _states.clone(),
    	language : language,
    	tape : tape.clone(),
    	q0 : _states.get(&accept_state).unwrap().clone(),
    	qa : _states.get(&accept_state).unwrap().clone(),
    	qr : Qstate{
    		name : "reject".to_string(),
    		isfinal : true,
    		transition: HashMap::new()
    	}

	};

	let mut tape_index : usize = 1;
	let mut cur_state = match _states.get(&start_state){
		Some(n) => n,
		None => {println!("rejected"); return; }
	};

	loop {
	//     // this is where the code to translate 
		let input = tape[tape_index] as char;
	    let moves = cur_state.transition.get(&input.to_string()).unwrap();

	    
	    println!("{:?}", moves);

		*tape.get_mut(tape_index).unwrap() =  moves.write.as_bytes()[0];
	    // incroment tape
		match moves.dir {
		    Left => tape_index -= 1,
		    Right => tape_index += 1,
		    Nowhere => tape_index = tape_index
		};

	    // write to tape
	    cur_state = match _states.get(&moves.state){
	    	Some(n) => n,
	    	None    => &tm.qr
	    };

		for item in &tape {
			print!("{:?}",*item as char);
		}
		println!("{:?}", cur_state.name );

		if *tape.get(tape_index).unwrap() == '_' as u8 {
			break;
		}
	}
	if cur_state.name == tm.qa.name{
		println!("accept");
	}else {
	    println!("rejected");
	}

}
// the hard part of this assignment is managing the tape























