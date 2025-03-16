/*
    This file is a part of bookman software.

    This module contains the main logic for all operations.

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

use crate::entry::Bookmark;
use crate::db_driver;
use crate::utils;
use crate::parser;

use rusqlite::Connection;
use skim::prelude::SkimOptionsBuilder;
use skim::{Skim, SkimItemReceiver, SkimItemSender};
use std::sync::Arc;

pub fn new(conn: &Connection) {
    match db_driver::new_table(conn) {
        Ok(()) => {
            return;
        }
        Err(err) => {
            utils::sql_driver_error(err);
        }
    }
}

pub fn add(conn: &Connection, _from_clipboard: bool) {
    if let Some((name, url, description)) = utils::prompt_user() {
        match db_driver::insert_entry(conn, &name, &url, &description) {
            Ok(()) => {
                #[cfg(debug_assertions)]
                {
                    println!("Bookmark added!");
                }
            }
            Err(err) => {
               utils::sql_driver_error(err);
            }
        }
    } else {
        utils::user_input_error()
    }
}

pub fn search(conn: &Connection) -> Option<String> {
    match db_driver::get_iterator(conn) {
        Ok(bookmarks) => {
            let bookmarks: Vec<Bookmark> = bookmarks;

            let options = SkimOptionsBuilder::default().build().unwrap();
            let (tx, rx): (SkimItemSender, SkimItemReceiver) = skim::prelude::unbounded();
            for bm in &bookmarks {
                let line = format!("{} | {} | {} | {}", bm.id, bm.name, bm.url, bm.description);
                let _ = tx.send(Arc::new(line));
            }
            drop(tx);

            let selected = Skim::run_with(&options, Some(rx))?.selected_items;
            if let Some(selected_item) = selected.first() {
                let selected_text = selected_item.output();
                if let Some(bookmark) = bookmarks.iter().find(|b| selected_text.contains(&b.url)) {
                    return Some(bookmark.url.clone());
                }
            }
            None
        }
        Err(err) => {
            utils::sql_driver_error(err);
        }
    }
}

pub fn edit(conn: &Connection, id: i32) {
    match db_driver::get_entry(conn, id) {
        Ok(bookmark) => {
            let old: Bookmark = bookmark;

            if let Some((new_name, new_url, new_description)) = utils::prompt_user() {

                let name = if new_name.trim().is_empty() { old.name.to_string() } else { new_name.trim().to_string() };
                let url = if new_url.trim().is_empty() { old.url.to_string() } else { new_url.trim().to_string() };
                let description = if new_description.trim().is_empty() { old.description.to_string() } else { new_description.to_string() };

                match db_driver::update_entry(conn, id, &name, &url, &description) {
                    Ok(()) => {
                        #[cfg(debug_assertions)]
                        {
                            println!("Bookmark updated!");
                        }
                        return;
                    }
                    Err(err) => {
                        utils::sql_driver_error(err);
                    }
                }
            } else {
                utils::user_input_error();
            }
        }
        Err(err) => {
            utils::sql_driver_error(err);
        }
    }
}

pub fn remove(conn: &Connection, id: i32) {
    match db_driver::remove_entry(conn, id) {
        Ok(()) => {
            #[cfg(debug_assertions)]
            {
                println!("Bookmark removed!");
            }
        }
        Err(err) => {
            utils::sql_driver_error(err);
        }
    }
}

pub fn clip(conn: &Connection) {
    if let Some(url) = search(conn) {
        utils::copy_to_clipboard(&url);
    }
}

pub fn import(conn: &Connection, source: &str) {
    match parser::parse_bookmarks(source) {
        Ok(imported) => {
            for bm in imported {
                match db_driver::insert_entry(conn, &bm.name, &bm.url, &bm.description) {
                    Ok(()) => {
                        continue;
                    }
                    Err(err) => {
                        utils::die("Import error", err);

                    }
                }
            }
        }
        Err(err) => {
            utils::parser_error(err);
        }
    }
}
