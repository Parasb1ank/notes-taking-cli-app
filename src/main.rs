use rusqlite::{Connection, Result};
use std::io::{self,Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "create table if not exists notes(
        id integer primary key,
        body text not null unique
        )",
        [],
    )?;

    loop {
        print!("-->");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let trimmed_body = buffer.trim();
        match trimmed_body {
            "" => break,
            "\\list" => {
                let mut stmt = conn.prepare("SELECT id,body FROM notes")?;
                let mut rows = stmt.query(rusqlite::params![])?;
                while let Some(row) = rows.next()? {
                    let id: i32 = row.get(0)?;
                    let body: String = row.get(1)?;
                    println!("{}. {}", id, body);
                }
            }
            "\\help" => {
                println!("\n<text>            : write msg or text directly to store it in database");
                println!("\\list             : list All the msg stored in database ");
                println!("\\del <id>         : delete text or msg stored at the given id");
                println!("\\edit <id> <text> : replace the given text with the text at the given id");
                println!("\\help             : To show this message\n");
            }
            body if body.starts_with("\\del") => {
                let cmd_split = body.split_once(" ");

                if let Some(cmd_split) = cmd_split {
                    let id = cmd_split.1;
                    conn.execute("DELETE FROM notes where id = (?1)", [id])?;
                } else {
                    println!("Usage: \\del <id>");
                }
            }
            body if body.starts_with("\\edit") => {
                let msg = body.replace("\\edit", "");
                if let Some(msg_split) = msg.trim().split_once(" ") {
                    let id = msg_split.0;
                    let body = msg_split.1;
                    conn.execute("UPDATE notes set body = (?1) where id =  (?2)", [body, id])?;
                } else {
                    println!("Usage: \\edit <id> <text>");
                }
            }
            body if body.starts_with("\\") => {
                println!("Not a valid Command"); 
            }
            body => {
                conn.execute("INSERT INTO notes (body) values (?1)", [body])?;
            }
        }
    }

    Ok(())
}
