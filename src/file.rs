//! Work with files
use std::{fs::File, io::{Write, Read}};
use super::todo_list::ToDoList;

const SAVE : &str = "notes.json";

/// Save to existing file or create new
pub fn save_to_file(notes : &ToDoList) {
    let mut save = File::create(SAVE).expect("Can not open or file");
    save.write_all(notes.to_json().as_bytes()).expect("Can not write to file");
}

/// Load from existing file or initialize new `Notes` struct
pub fn load_from_file() -> ToDoList {
    if let Ok(mut load) = File::open(SAVE) {
	let mut json = String::new();
	if let Ok(_) = load.read_to_string(&mut json) {
	    ToDoList::from_json(json)
	} else {
	    ToDoList::new()
	}
    } else {
	ToDoList::new()
    }
}
