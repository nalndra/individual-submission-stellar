#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Vec, Symbol
};

// Key untuk storage
const NOTE_DATA: Symbol = symbol_short!("NOTES");

// Struct Note
#[contracttype]
#[derive(Clone)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
}

// Declare contract
#[contract]
pub struct NotesContract;

// Implementasi contract
#[contractimpl]
impl NotesContract {

    // Get semua notes
    pub fn get_notes(env: Env) -> Vec<Note> {
        env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Create note
    pub fn create_note(env: Env, title: String, content: String) -> String {
        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        let note = Note {
            id: env.prng().gen::<u64>(),
            title,
            content,
        };

        notes.push_back(note);

        env.storage().instance().set(&NOTE_DATA, &notes);

        String::from_str(&env, "Notes berhasil ditambahkan")
    }

    // Delete note
    pub fn delete_note(env: Env, id: u64) -> String {
        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                notes.remove(i);
                env.storage().instance().set(&NOTE_DATA, &notes);

                return String::from_str(&env, "Berhasil hapus notes");
            }
        }

        String::from_str(&env, "Notes tidak ditemukan")
    }
}

mod test;