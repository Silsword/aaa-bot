//! Describes struct for representation of one note or task
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

static mut TASK_COUNT: u64 = 0;

/// List of states
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
enum State {
    ToDo,
    Doing,
    Done,
    None,
}

/// States implementation
impl State {
    pub fn to_message(&self) -> String {
        //! Represent state as String
        match self {
            State::ToDo => "ToDo".to_string(),
            State::Doing => "Doing".to_string(),
            State::Done => "Done".to_string(),
            State::None => String::new(),
        }
    }

    pub fn from_message(state: String) -> State {
        //! Parse state from String
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "todo" => State::ToDo,
            "doing" => State::Doing,
            "done" => State::Done,
            _ => State::None,
        }
    }
}

/// Struct to representing of one note or task
#[derive(Serialize, Deserialize, Hash, Eq, Clone)]
pub struct Task {
    id: u64,
    header: String,
    chat_id: u64,
    text: String,
    state: State,
    deadline: Option<String>,
}

/// Implementation of ParitialEq trait
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Implementation of Note struct
impl Task {
    pub fn new() -> Task {
        //! Default initialization
        let current;
        unsafe {
            current = TASK_COUNT;
            TASK_COUNT += 1;
        };
        Task {
            id: current,
            header: String::new(),
            chat_id: 0,
            text: String::new(),
            state: State::None,
            deadline: None,
        }
    }

    pub fn dry_new() -> Task {
        //! Initialization without changing `NOTES_COUNT`
        Task {
            id: 0,
            header: String::new(),
            chat_id: 0,
            text: String::new(),
            state: State::None,
            deadline: None,
        }
    }

    pub fn with_id(self, new_id: u64) -> Task {
        //! Change ID in initialization
        Task { id: new_id, ..self }
    }

    pub fn with_header(self, head: String) -> Task {
        //! Change name in initialization
        Task {
            header: head,
            ..self
        }
    }

    pub fn with_chat(self, chat: u64) -> Task {
        //! Change chat_id in initialization
        Task {
            chat_id: chat,
            ..self
        }
    }

    pub fn with_text(self, note: String) -> Task {
        //! Change text in initialization
        Task { text: note, ..self }
    }

    pub fn with_state_todo(self) -> Task {
        //! Change ToDo-state in initialization
        Task {
            state: State::ToDo,
            ..self
        }
    }

    pub fn with_state_doing(self) -> Task {
        //! Change Doing-state in initialization
        Task {
            state: State::Doing,
            ..self
        }
    }

    pub fn with_state_done(self) -> Task {
        //! Change Done-state in initialization
        Task {
            state: State::Done,
            ..self
        }
    }

    pub fn with_deadline(self, date: String) -> Task {
        //! Change due date in initialization
        Task {
            deadline: Some(date),
            ..self
        }
    }

    pub fn from_json(json: String) -> Task {
        //! Parse struct from JSON
        serde_json::from_str(&json).unwrap()
    }

    pub fn to_json(&self) -> String {
        //! Serialize struct as JSON
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

    pub fn set_state_from_string(&mut self, state: String) {
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
        //! Serialize struct to Human-readable view
        format!(
            "{}\n\
		State : {}\n\
		Deadline : {}\n\
		{}\n\
		\n\nid : {}",
            self.header,
            self.state.to_message(),
            if let Some(d) = self.deadline() {
                d.as_str()
            } else {
                "None"
            },
            self.text,
            self.id,
        )
        .to_string()
    }

    pub fn set_count(count: u64) {
        //! Change global `NOTES_COUNT`
        unsafe {
            TASK_COUNT = count;
        }
    }
}
/// Impl `Default` trait
impl Default for Task {
    fn default() -> Self {
        Self::new()
    }
}
