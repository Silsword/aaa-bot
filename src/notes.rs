use super::note::Note;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Serialize, Deserialize)]
pub struct Notes {
    notes: HashMap<u64, Note>,
    size: u64,
}

impl Notes {
    pub fn new() -> Notes {
        Notes {
            notes: HashMap::new(),
            size: 0,
        }
    }

    pub fn add(&mut self, note: Note) {
        self.notes.insert(note.id(), note);
    }

    pub fn delete(&mut self, id: u64) {
        self.notes.remove(&id);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(notes: String) -> Notes {
        let notes: Notes = serde_json::from_str(&notes).unwrap();
        Note::set_count(notes.notes.len() as u64);
        notes
    }

    pub fn notes_by_chat_all(&self, chat_id: u64) -> Vec<Note> {
        self.notes
            .iter()
            .filter(|note| note.1.chat_id() == chat_id)
            .map(|note| note.1)
            .cloned()
            .collect()
    }

    pub fn notes_by_chat(&self, chat_id: u64) -> Vec<Note> {
        self.notes_by_chat_all(chat_id)
            .iter()
            .filter(|note| !note.done())
            .cloned()
            .collect()
    }

    pub fn note_by_id(&self, id: u64) -> Option<&Note> {
        self.notes.get(&id)
    }

    pub fn note_as_mut(&mut self, id: u64) -> Option<&mut Note> {
        self.notes.get_mut(&id)
    }

    pub fn notes_agenda(&self, chat_id: u64) -> Vec<Note> {
        let date: NaiveDate = chrono::Utc::now().naive_local().date();
        self.notes_by_chat(chat_id)
            .iter()
            .filter(|a| a.deadline() != None)
            .filter(|a| {
                date.signed_duration_since(NaiveDate::from_str(a.deadline().unwrap()).unwrap())
                    .num_days()
                    >= -7
            })
            .cloned()
            .collect()
    }
}
