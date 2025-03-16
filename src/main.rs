/*
    This file is a part of bookman software.

    Copyright (c) 2025 Pavel Pleskunov.

    bookman is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or (at
    your option) any later version.

    bookman is distributed in the hope that it will be useful, but
    WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
    USA
*/

pub mod entry;
pub mod utils;
pub mod parser;
pub mod db_driver;
pub mod manager;
pub mod config;
pub mod notify;

use std::fs;
use std::env;
use std::path;
use crate::parser::Commands;
use clap::Parser;
use rusqlite::Connection;

fn main() {
    let home: String  = env::var("HOME").expect("env variable '$HOME' is not set.");
    let db_file = path::PathBuf::from(home).join(config::DB_FILE);

    // Make sure the db path exists
    let db_dir = path::Path::new(&db_file).parent().unwrap();
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).expect("Failed to create database directory");
    }

    // Connect to the database
    let conn = Connection::open(db_file).expect("Failed to open database");

    // Establish database encryption
    conn.pragma_update(None, "key", config::DB_PASS).unwrap();

    manager::new(&conn);

    let cli = parser::Cli::parse();
    match cli.command {
        Commands::Add { clipboard } => {
            manager::add(&conn, clipboard);
        },
        Commands::Search => {
            if let Some(url) = manager::search(&conn) {
                #[cfg(debug_assertions)]
                {
                    notify::send_notification("Bookman", &format!("Selected: {}", url));
                    println!("Selected URL: {}", url);
                }
            }
        },
        Commands::Edit { id } => {
            manager::edit(&conn, id);
        },
        Commands::Remove { id } => {
            manager::remove(&conn, id);
        },
        Commands::Clip => {
            manager::clip(&conn);
        },
        Commands::Import { path } => {
            manager::import(&conn, &path);
        }
    }
}
