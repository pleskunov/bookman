/*
    This file is a part of bookman software.

    This module contains various helper functions.

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

use std::{thread, time::Duration};
use std::process::{self, Command};
use std::error::Error;

use arboard::Clipboard;

pub fn die<E: Into<Box<dyn Error>>>(message: &str, error: E) -> ! {
    eprintln!("\x1b[1;31mError:\x1b[0m {}: {}", message, error.into());
    process::exit(1);
}

pub fn user_input_error() -> ! {
    let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, "Propmt failed!");
    die("User input error", err);
}

pub fn sql_driver_error<E: Into<Box<dyn Error>>>(error: E) -> ! {
    die("SQL Error", error);
}

pub fn parser_error<E: Into<Box<dyn Error>>>(error: E) -> ! {
    die("Parser Error", error);

}

pub fn sleep(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

pub fn is_installed(binary: &str) -> bool {
    Command::new("which")
        .arg(binary)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn to_wl_copy(text: &str) { 
    let mut daemon = Command::new("sh")
                        .arg("-c")
                        .arg(format!("echo '{}' | wl-copy", text))
                        .spawn()
                        .expect("Failed to copy. Is wl-copy installed?");
    sleep(45);
    let _ = daemon.kill();
}

fn to_clipboard(text: &str, wait: bool) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
    if wait {
        sleep(45);
    }
}

pub fn copy_to_clipboard(text: &str) {
    #[cfg(target_os = "linux")]
    {
        if is_installed("wl-copy") {
            to_wl_copy(text);
        } else {
            to_clipboard(text, true);
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        to_clipboard(text, false);
    }

    #[cfg(debug_assertions)]
    {
        println!("Copied to clipboard: {}", text);
    }
}

pub fn copy_from_clipboard() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let url = clipboard.get_text().unwrap_or_else(|_| "".to_string());

    #[cfg(debug_assertions)]
    {
        println!("Copied from clipboard: {}", url);
    }

    return url;
}

pub fn prompt_user() -> Option<(String, String, String)> {
    let mut url = String::new();
    let mut name = String::new();
    let mut description = String::new();

    println!("Enter name:");
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Enter URL:");
    std::io::stdin().read_line(&mut url).unwrap();

    println!("Enter description:");
    std::io::stdin().read_line(&mut description).unwrap();

    return Some((name.clone(), url.clone(), description.clone()));
}
