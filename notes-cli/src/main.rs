use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;

const FILE: &str = "notes.json";

#[derive(Serialize, Deserialize, Debug)]
struct NotesList {
    notes: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        note: String,
    },
    List,
    Remove {
        #[arg(short, long)]
        index: usize,
    },
}

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

impl NotesList {

    fn load_data() -> Self {
        if let Ok(data) = fs::read_to_string(FILE) {
            serde_json::from_str(&data).unwrap_or(NotesList { notes: vec![] })
        } else {
            NotesList { notes: vec![] }
        }
    }
    fn save(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(FILE)
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

fn main() {
    println!("Hello, world!, Welcome to Notes");
    // Define the structs
    // Define the commands
    // Define how they will be stored

    // let mut notes = NotesList::new_note("Note1".to_string());
    //    let mut notes = NotesList::load_data();
    //let mut notes = NotesList {
    //    notes: vec!["This is note from code!".to_string()],
    //};
    //println!("{:?}", notes);
    //let data = "Hi My new notes".to_string();
    //notes.notes.push(data.clone());
    //notes.save();
    //println!("{:?}", notes);

    let cli = CLI::parse();
    let mut notes_list = NotesList::load_data();

    match cli.command {
        Commands::Add { note } => {
            notes_list.notes.push(note.clone());
            notes_list.save();
            println!("Note Added");
        }
        Commands::List => {
            for (i, notes) in notes_list.notes.iter().enumerate() {
                println!("{}: {}", i, notes);
            }
        }
        Commands::Remove { index } => {
            if index < notes_list.notes.len() {
                let _removed = notes_list.notes.remove(index);
                notes_list.save();
                println!("Note removed");
            } else {
                println!("There are no notes");
            }
        }
    }
}
