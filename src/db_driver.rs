use crate::entry::Bookmark;
use rusqlite::{params, Connection};
use std::error::Error;

pub fn new_table(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            url TEXT,
            description TEXT
        );",
        [],
    )?;

    Ok(())
}

pub fn insert_entry(conn: &Connection, name: &String, url: &String, description: &String) -> Result<(), Box<dyn Error>> {
    conn.execute("INSERT INTO bookmarks (name, url, description) VALUES (?1, ?2, ?3)",
        params![name.trim(), url.trim(), description.trim()],)?;

    Ok(())
}

pub fn get_entry(conn: &Connection, id: i32) -> Result::<Bookmark, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT id, name, url, description FROM bookmarks WHERE id = ?1")?;

    let mut query = stmt.query(params![id])?;
    if let Some(row) = query.next()? {
        return Ok(Bookmark {
            id: row.get(0)?, 
            name: row.get(1)?, 
            url: row.get(2)?, 
            description: row.get(3)?,
        });
    }

    Err(Box::new(rusqlite::Error::QueryReturnedNoRows))
}

pub fn get_iterator(conn: &Connection) -> Result<Vec<Bookmark>, Box<dyn Error>> {

    //let mut stmt = conn.prepare("SELECT url FROM bookmarks").unwrap();
    //let urls: Vec<String> = stmt.query_map([], |row| row.get(0)).unwrap().filter_map(Result::ok).collect();

    let mut stmt = conn.prepare("SELECT id, name, url, description FROM bookmarks")?;
    let bookmarks: Vec<_> = stmt.query_map([], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            description: row.get(3)?,
        })
    })?.filter_map(Result::ok).collect();

    return Ok(bookmarks);
}

pub fn update_entry(conn: &Connection, id: i32, name: &String, url: &String, description: &String) -> Result<(), Box<dyn Error>> {
    conn.execute("UPDATE bookmarks SET name = ?1, url = ?2, description = ?3 WHERE id = ?4",
        params![name.trim(), url.trim(), description.trim(), id],)?;

    Ok(())
}

pub fn remove_entry(conn: &Connection, id: i32) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id])?;

    Ok(())
}
