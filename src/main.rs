#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate dotenv;

use dotenv::dotenv;
use std::{env, println};
use reqwest;

use rusqlite::{Connection, Result};
#[derive(Debug)]

struct Group {
    groupid: i32,
    checksum: String,
}
struct MeetupGroup {
    id: i32,
}
    

fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = Connection::open(database_url)?;


    let mut stmt = conn.prepare(
        "select groupid,checksum from \"group\""
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Group {
            groupid: row.get(0)?,
            checksum: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}
