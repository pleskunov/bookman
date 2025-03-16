/*
    This file is a part of bookman software.

    This module contains helper functions respobnsible for notifications.

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

#[cfg(target_os = "linux")]
use notify_rust::Notification;

#[cfg(target_os = "macos")]
use std::process::Command;

pub fn send_notification(title: &str, message: &str) {
    #[cfg(target_os = "linux")]
    {
        Notification::new()
            .summary(title)
            .body(message)
            .icon("dialog-information") // Use a system icon or specify your own
            .show()
            .expect("Failed to send notification");
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("osascript")
            .arg("-e")
            .arg(format!("display notification \"{}\" with title \"{}\"", message, title))
            .output()
            .expect("Failed to send notification");
    }
}
