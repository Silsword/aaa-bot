use std::{fs::File, io::{Write, Read}};
use super::notes::Notes;

const SAVE : &str = "notes.json";

pub fn save_to_file(notes : &Notes) {
    let mut save = File::create(SAVE).expect("Can not open or file");
    save.write_all(notes.to_json().as_bytes()).expect("Can not write to file");
}

pub fn load_from_file() -> Notes {
    if let Ok(mut load) = File::open(SAVE) {
	let mut json = String::new();
	if let Ok(_) = load.read_to_string(&mut json) {
	    Notes::from_json(json)
	} else {
	    Notes::new()
	}
    } else {
	Notes::new()
    }
}
