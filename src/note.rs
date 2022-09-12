use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

static mut NOTES_COUNT : u64 = 0;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
enum State {
    ToDo,
    Doing,
    Done,
    None,
}

impl State {
    pub fn to_message(&self) -> String {
	match self {
	    State::ToDo => "ToDo".to_string(),
	    State::Doing => "Doing".to_string(),
	    State::Done => "Done".to_string(),
	    State::None => String::new(),
	}
    }

    pub fn from_message(state : String) -> State {
	let state = state.trim().to_lowercase();
	match state.as_str() {
	    "todo" => State::ToDo,
	    "doing" => State::Doing,
	    "done" => State::Done,
	    _ => State::None,
	}
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, Clone)]
pub struct Note {
    id : u64,
    header : String,
    chat_id : u64,
    text : String,
    state : State,
    deadline : Option<String>,
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
	self.id == other.id
    }
}

impl Note {
    pub fn new() -> Note {
	let current;
	unsafe {
	    current = NOTES_COUNT;
	    NOTES_COUNT += 1;
	};
	Note {
	    id : current,
	    header : String::new(),
	    chat_id : 0,
	    text : String::new(),
	    state : State::None,
	    deadline : None,
	}
    }

    pub fn dry_new() -> Note {
	Note {
	    id : 0,
	    header : String::new(),
	    chat_id : 0,
	    text : String::new(),
	    state : State::None,
	    deadline : None,
	}
    }

    pub fn with_id(self, new_id : u64) -> Note {
 	Note {
	    id : new_id,
	    ..self
	}
    }
    
    pub fn with_header(self, head : String) -> Note {
 	Note {
	    header : head,
	    ..self
	}
    }

    pub fn with_chat(self, chat : u64) -> Note {
	Note {
	    chat_id : chat,
	    ..self
	}
    }

    pub fn with_text(self, note : String) -> Note {
	Note {
	    text : note,
	    ..self
	}
    }

    pub fn with_state_todo(self) -> Note {
	Note {
	    state : State::ToDo,
	    ..self
	}
    }

    pub fn with_state_doing(self) -> Note {
	Note {
	    state : State::Doing,
	    ..self
	}
    }

    pub fn with_state_done(self) -> Note {
	Note {
	    state : State::Done,
	    ..self
	}
    }

    pub fn with_deadline(self, date : String) -> Note {
	Note {
	    deadline : Some(date),
	    ..self
	}
    }
    
    pub fn from_json(json : String) -> Note {
	serde_json::from_str(&json).unwrap()
    }
    
    pub fn to_json(&self) -> String {
	serde_json::to_string(self).unwrap()
    }    

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_header(&mut self, header: String) {
        self.header = header;
    }

    pub fn set_chat_id(&mut self, chat_id: u64) {
        self.chat_id = chat_id;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn set_state_from_string(&mut self, state : String) {
	self.state = State::from_message(state);
    }

    pub fn set_deadline(&mut self, deadline: Option<String>) {
        self.deadline = deadline;
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    
    pub fn header(&self) -> &str {
        self.header.as_ref()
    }
    
    pub fn chat_id(&self) -> u64 {
        self.chat_id
    }
    
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn deadline(&self) -> Option<&String> {
        self.deadline.as_ref()
    }

    pub fn todo(&self) -> bool {
	self.state == State::ToDo
    }

    pub fn doing(&self) -> bool {
	self.state == State::Doing
    }
    
    pub fn done(&self) -> bool {
	self.state == State::Done
    }

    pub fn to_message(&self) -> String {
	format!("{}\n\
		State : {}\n\
		Deadline : {}\n\
		{}\n\
		\n\n__id : {}__",
		self.header,
		self.state.to_message(),
		if let Some(d) = self.deadline { d.as_str() } else { "None" },
		self.text,
		self.id,
	).to_string()
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}
