#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan notes
#[contracttype]
#[derive(Clone, Debug)]
pub struct GuestEntry {
    id: u64,
    guest_name: String,
    message: String,

}

// Storage key untuk data notes
const GUEST_DATA: Symbol = symbol_short!("GUEST_DATA");

#[contract]
pub struct GuestbookContract;

#[contractimpl]
impl GuestbookContract {

    pub fn get_guests(env: Env) -> Vec<GuestEntry> {
        // 1. ambil data dari storage
        return env.storage().instance().get(&GUEST_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat entry baru
    pub fn add_guest(env: Env, guest_name: String, message: String) -> String {
        // 1. ambil data dari storage
        let mut guests: Vec<GuestEntry> = env.storage().instance().get(&GUEST_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object baru
        let guest = GuestEntry {
            id: env.prng().gen::<u64>(),
            guest_name: guest_name,
            message: message,
        };
        
        // 3. tambahkan data baru ke yang lama
        guests.push_back(guest);
        
        // 4. simpan ke storage
        env.storage().instance().set(&GUEST_DATA, &guests);
        
        return String::from_str(&env, "Pesan tamu berhasil ditambahkan");
    }

    // Fungsi untuk menghapus berdasarkan id
    pub fn delete_guest(env: Env, id: u64) -> String {
        // 1. ambil data dari storage 
        let mut guests: Vec<GuestEntry> = env.storage().instance().get(&GUEST_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index yang akan dihapus menggunakan perulangan
        for i in 0..guests.len() {
            if guests.get(i).unwrap().id == id {
                guests.remove(i);

                env.storage().instance().set(&GUEST_DATA, &guests);
                return String::from_str(&env, "Berhasil hapus pesan tamu");
            }
        }

        return String::from_str(&env, "Pesan tamu tidak ditemukan")
    }
}

mod test;













/* --- CONTOH SCRIPT ---

pub fn get_notes(env: Env) -> Vec<Note> {
    // 1. ambil data notes dari storage
    return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
}

// Fungsi untuk membuat note baru
pub fn create_note(env: Env, title: String, content: String) -> String {
    // 1. ambil data notes dari storage
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object note baru
    let note = Note {
        id: env.prng().gen::<u64>(),
        title: title,
        content: content,
    };
    
    // 3. tambahkan note baru ke notes lama
    notes.push_back(note);
    
    // 4. simpan notes ke storage
    env.storage().instance().set(&NOTE_DATA, &notes);
    
    return String::from_str(&env, "Notes berhasil ditambahkan");
}

// Fungsi untuk menghapus notes berdasarkan id
pub fn delete_note(env: Env, id: u64) -> String {
    // 1. ambil data notes dari storage 
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index note yang akan dihapus menggunakan perulangan
    for i in 0..notes.len() {
        if notes.get(i).unwrap().id == id {
            notes.remove(i);

            env.storage().instance().set(&NOTE_DATA, &notes);
            return String::from_str(&env, "Berhasil hapus notes");
        }
    }

    return String::from_str(&env, "Notes tidak ditemukan")
}


*/