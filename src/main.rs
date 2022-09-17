use chrono::NaiveDate;
use file::{load_from_file, save_to_file};
use note::Note;
use notes::Notes;
use once_cell::sync::Lazy;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{error::Error, str::FromStr};

use teloxide::{
    prelude::*,
    utils::command::{BotCommands, ParseError},
};

mod file;
mod note;
mod notes;

static NOTES: Lazy<Mutex<Notes>> = Lazy::new(|| Mutex::new(load_from_file()));

#[tokio::main]
async fn main() {
    log::info!("Starting command bot...");

    let bot = Bot::new("<token here>").auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}


/// Declaration and description of commands
#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "start bot, display welcome message.")]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(
        description = "<name> - create task with <name>.",
        parse_with = "one_line_parser"
    )]
    Create(String),
    #[command(
        description = "<id> <ToDo | Doing | Done> - set task with <id> to <state>.",
        parse_with = "split"
    )]
    SetState(u64, String),
    #[command(
        description = "<id> <yyyy-mm-dd> - set task with <id> due date to <dead>.",
        parse_with = "split"
    )]
    SetDead(u64, String),
    #[command(
        description = "<id> <text> - change text of task with <id> to <text>",
        parse_with = "text_parser"
    )]
    Edit(u64, String),
    #[command(
        description = "<id> <name> - set name of task with <id> to <name>.",
        parse_with = "text_parser"
    )]
    EditName(u64, String),
    #[command(description = "<id> - delete task with <id>", parse_with = "split")]
    Delete(u64),
    #[command(description = "<id> - show task with <id>", parse_with = "split")]
    Show(u64),
    #[command(description = "- list ToDo and Doing tasks")]
    List,
    #[command(description = "list all tasks")]
    ListAll,
    #[command(description = "list all tasks for next week", parse_with = "split")]
    Agenda,
}

/// Incapsulation of work with static global variable NOTES

fn note_add(inst: Note) {
    //! Incapsulate add method
    NOTES.lock().unwrap().add(inst.clone());
    save_to_file(&Notes::from_json(NOTES.lock().unwrap().to_json()));
}

fn note_set_state(id: u64, state: String) -> bool {
    //! Incapsulate set_state_from_string method
    if let Some(mut note) = NOTES.lock().unwrap().note_as_mut(id) {
        note.set_state_from_string(state);
        true
    } else {
        false
    }
}

fn note_set_deadline(id: u64, deadline: String) -> bool {
    //! Incapsulate set_deadline method
    if let Some(mut note) = NOTES.lock().unwrap().note_as_mut(id) {
        note.set_deadline(Some(deadline));
        save_to_file(&Notes::from_json(NOTES.lock().unwrap().to_json()));
        true
    } else {
        false
    }
}

fn note_set_text(id: u64, text: String) -> bool {
    //! Incapsulate set_text method
    if let Some(mut note) = NOTES.lock().unwrap().note_as_mut(id) {
        note.set_text(text);
        save_to_file(&Notes::from_json(NOTES.lock().unwrap().to_json()));
        true
    } else {
        false
    }
}

fn note_set_name(id: u64, name: String) -> bool {
    //! Incapsulate set_name method
    if let Some(mut note) = NOTES.lock().unwrap().note_as_mut(id) {
        note.set_header(name);
        save_to_file(&Notes::from_json(NOTES.lock().unwrap().to_json()));
        true
    } else {
        false
    }
}

fn note_delete(id: u64) {
    //! Incapsulate delete method
    NOTES.lock().unwrap().delete(id);
    save_to_file(&Notes::from_json(NOTES.lock().unwrap().to_json()));
}

fn note_show(id: u64) -> Option<Note> {
    //! Incapsulate note_by_id method
    if let Some(note) = NOTES.lock().unwrap().note_by_id(id) {
        Some(note.clone())
    } else {
        None
    }
}

fn note_list(chat_id: u64) -> Vec<Note> {
    //! Incapsulate selection by chat_id
    NOTES.lock().unwrap().notes_by_chat(chat_id)
}

fn note_list_all(chat_id: u64) -> Vec<Note> {
    //! Incapsulate selection notes by chat with any state
    NOTES.lock().unwrap().notes_by_chat_all(chat_id)
}

fn note_agenda(chat_id: u64) -> Vec<Note> {
    //! Incapsulate selection notes by date
    NOTES.lock().unwrap().notes_agenda(chat_id)
}

async fn answer(
    //! Function for handling commands
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Start => {
            bot.send_message(
                message.chat.id,
                "Hello! This is AAA Bot. \
		 I am yor personal \
		 task-manger. Type /help \
		 to display command list",
            )
            .await?
        }
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Create(name) => {
            let inst = Note::new()
                .with_header(name)
                .with_chat(message.chat.id.0 as u64);
            note_add(inst.clone());
            bot.send_message(message.chat.id, format!("Ok:) id : {}", inst.id()))
                .await?
        }
        Command::SetState(id, state) => {
            if note_set_state(id, state) {
                bot.send_message(message.chat.id, "State changed").await?
            } else {
                bot.send_message(message.chat.id, "Unknown id").await?
            }
        }
        Command::SetDead(id, deadline) => {
            if let Err(_) = NaiveDate::from_str(&deadline) {
                bot.send_message(message.chat.id, "Invalid date format")
                    .await?
            } else if note_set_deadline(id, deadline) {
                bot.send_message(message.chat.id, "Deadline changed")
                    .await?
            } else {
                bot.send_message(message.chat.id, "Unknown id").await?
            }
        }
        Command::Edit(id, text) => {
            if note_set_text(id, text) {
                bot.send_message(message.chat.id, "Text changed").await?
            } else {
                bot.send_message(message.chat.id, "Unknown id").await?
            }
        }
        Command::EditName(id, name) => {
            if note_set_name(id, name) {
                bot.send_message(message.chat.id, "Name changed").await?
            } else {
                bot.send_message(message.chat.id, "Unknown id").await?
            }
        }
        Command::Delete(id) => {
            note_delete(id);
            bot.send_message(message.chat.id, "Ok:)").await?
        }
        Command::Show(id) => {
            if let Some(note) = note_show(id) {
                bot.send_message(message.chat.id, note.to_message()).await?
            } else {
                bot.send_message(message.chat.id, "Unknown id").await?
            }
        }
        Command::List => {
            for i in note_list(message.chat.id.0 as u64) {
                bot.send_message(message.chat.id, i.to_message()).await?;
            }
            bot.send_message(message.chat.id, "Ok:)").await?
        }
        Command::ListAll => {
            for i in note_list_all(message.chat.id.0 as u64) {
                bot.send_message(message.chat.id, i.to_message()).await?;
            }
            bot.send_message(message.chat.id, "Ok:)").await?
        }
        Command::Agenda => {
            for i in note_agenda(message.chat.id.0 as u64) {
                bot.send_message(message.chat.id, i.to_message()).await?;
            }
            bot.send_message(message.chat.id, "Ok:)").await?
        }
    };

    Ok(())
}

fn text_parser(input: String) -> Result<(u64, String), ParseError> {
    //! Parse messages with text like <id> <name> or <id> <text>
    let tmp: Vec<&str> = input.split(" ").collect();
    if let Ok(id) = tmp.get(0).unwrap().parse::<u64>() {
        let text = &tmp[1..].join(" ").to_string();
        Ok((id, text.clone()))
    } else {
        Err(ParseError::Custom(
            "First argument should be a number!".into(),
        ))
    }
}

fn one_line_parser(input: String) -> Result<(String,), ParseError> {
    //! Parse messages with text like <name>
    Ok((input,))
}
