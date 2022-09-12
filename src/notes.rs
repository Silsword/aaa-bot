use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use std::{collections::HashSet, str::FromStr};
use super::note::Note;

#[derive(Serialize, Deserialize)]
pub struct Notes {
    notes : HashSet<Note>,
    size : u64,
}

impl Notes {
    pub fn new() -> Notes {
	Notes {
	    notes : HashSet::new(),
	    size : 0,
	}
    }

    pub fn add(&mut self, note : Note) {
	self.notes.insert(note);
    }

    pub fn delete(&mut self, id : u64) {
	let inst = Note::dry_new().with_id(id);
	self.notes.remove(&inst);
    }

    pub fn to_json(&self) -> String {
	serde_json::to_string(self).unwrap()
    }

    pub fn from_json(notes : String) -> Notes {
	let notes : Notes = serde_json::from_str(&notes).unwrap();
	Note::set_count(notes.notes.len() as u64);
	notes
    }

    pub fn notes_by_chat_all(&self, chat_id : u64) -> Vec<Note> {
	self.notes.iter()
	    .filter(|note| note.chat_id() == chat_id)
	    .cloned()
	    .collect()
    }

    pub fn notes_by_chat(&self, chat_id : u64) -> Vec<Note> {
	self.notes_by_chat_all(chat_id)
	    .iter().filter(| note | !note.done())
	    .cloned()
	    .collect()
    }

    pub fn note_by_id(&self, id : u64) -> Option<&Note> {
	self.notes
	    .iter()
	    .filter(|a| a.id() == id)
	    .collect::<Vec<&Note>>()
	    .get(0)
	    .copied()
    }
 
    pub fn note_as_mut(&mut self, id : u64) -> Option<Note> {
	if let Some(inst) = self.notes
	    .cloned()
	    .filter(|a| a.id() == id)
	    .collect::<Vec<Note>>()
	    .get(0) {
		self.notes.take(&inst)
	    } else {
		None
	    }
    }

    pub fn notes_agenda(&self, chat_id : u64) -> Vec<Note> {
	todo!()
	// let date : NaiveDate = chrono::Utc::now().date();
	// self.notes_by_chat(chat_id)
	//     .iter()
	//     .filter(|a| date.signed_duration_since(NaiveDate::from_str(a.deadline())).num_days() <= 7)
	//     .cloned()
	//     .collect()
    }
}
