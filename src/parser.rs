/*
    This file is a part of bookman software.

    This module contains parsers used for handling command line arguments
    and for importing bookmakrs from HTML files.

    Copyright (c) 2025 Pavel Pleskunov.

    bookman is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3 of the License, or (at
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
use std::fs;
use std::io;
use scraper::{Html, Selector};
use clap::{Parser, Subcommand};
use std::error::Error;

/// CLI Bookmark Manager with encryption
#[derive(Parser)]
#[command(name = "bookman", version = "1.0", author = "Pavel", about = "A simple CLI Bookmark Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new bookmark
    Add {
        /// Copy from clipboard
        #[arg(short, long)]
        clipboard: bool,
    },

    /// Search bookmarks
    Search,

    /// Edit a bookmark
    Edit {
        /// ID of the bookmark to edit
        id: i32,
    },

    /// Remove a bookmark
    Remove {
        /// ID of the bookmark to remove
        id: i32,
    },

    /// Copy bookmark URL to clipboard
    Clip,

    /// Import bookmarks from a file
    Import {
        /// Path to the bookmarks file
        path: String,
    },
}

pub fn parse_bookmarks(html_file: &str) -> io::Result<Vec<Bookmark>> {
    let html = fs::read_to_string(html_file)?;
    let document = Html::parse_document(&html);

    let a_selector = Selector::parse("dt > a").unwrap();

    let mut bookmarks = Vec::new();

    let id = 0;
    let description = String::new();

    for element in document.select(&a_selector) {
        let url = element.value().attr("href").unwrap_or("").to_string();
        let name = element.text().collect::<Vec<_>>().join(" ");
        /*
        let _timestamp = element
            .value()
            .attr("add_date")
            .and_then(|s| s.parse::<u64>().ok());
        */
        bookmarks.push(Bookmark { id : id.clone(), name, url, description : description.clone() });
    }

    Ok(bookmarks)
}

pub fn parse_html_text(text: &String) -> Result<String, Box<dyn Error>> {
    let document = Html::parse_document(&text); // Parse HTML
    let selector = Selector::parse("title").unwrap(); // Select `<title>` tag

    if let Some(title) = document.select(&selector).next() {
        Ok(title.text().collect::<String>().trim().to_string()) // Extract and return text
    } else {
        Err("Title not found".into())
    }
}
