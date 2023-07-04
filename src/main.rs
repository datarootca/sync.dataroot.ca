#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]
#![feature(type_name_of_val)]
use std::any::type_name_of_val;

#[macro_use]
extern crate dotenv;
use serde::{Serialize, Deserialize};
use postgres::{Client, NoTls, Error};

use dotenv::dotenv;
use std::{env, println };

use reqwest;

use rusqlite::{Connection, Result};
#[derive(Debug)]
struct Transmission {
    id: i32,
    hash: String,
}
#[derive(Debug)]
struct Group {
    groupid: i32,
    name: String,
    description: String,
    organizer: String,
    slug: String,
    members: i32,
    private: bool,
    active: bool,
    cityid: i32,
    highres_link: String,
    photo_link: String,
    thumb_link: String,
    extid: String,
}
#[derive(Debug)]
#[derive(Deserialize)]
struct MeetupOrganizer {
    id: i32,
    name: String,
    bio: String,
    photo: MeetupGroupPhoto,
}
#[derive(Debug)]
#[derive(Deserialize)]
struct MeetupGroupPhoto {
    id: i32,
    highres_link: String,
    photo_link: String,
    thumb_link: String,
}
#[derive(Debug)]
#[derive(Deserialize)]
struct MeetupGroup {
    id: i32,
    name: String,
    join_mode: String,
    lat: f32,
    lon: f32,
    status: String,
    link: String,
    description: String,
    urlname: String,
    state: String,
    visibility: String,
    city: String,
    members: i32,
    member_pay_fee: bool,
    wepay_fee_deprecated: bool,
    lang: String,
    timezone: String,
    who: String,
    key_photo: MeetupGroupPhoto,
    group_photo: MeetupGroupPhoto,
    organizer: MeetupOrganizer,
}
struct City {
    cityid: i32,
    name: String,
}
   fn fetchMeetupGroup() -> Result<(), Error>    {
       
        let resp = reqwest::blocking::get("https://api.meetup.com/learndatascience").unwrap().json::<MeetupGroup>().unwrap();

        let mut group = Group{
            groupid: 3,
            name: resp.name.clone(),
            slug: resp.urlname,
            description: resp.description,
            organizer: resp.organizer.name,
            active: resp.status == "active",
            members: resp.members,
            private: resp.join_mode != "open",
            cityid: 1,
            extid: String::from("meetup") + &resp.id.to_string(),
            photo_link:  resp.key_photo.photo_link,
            thumb_link:  resp.key_photo.thumb_link,
            highres_link:  resp.key_photo.highres_link,
        };

        let pg_database_url = env::var("PG_DATABASE_URL").expect("set PG_DATABASE_URL");
let mut client = Client::connect(&pg_database_url, 
                                    NoTls)?;
let cityQuery = client.query_one("select cityid,name from city where name = $1",&[&resp.city])?;

let city = City{
    cityid: cityQuery.get(0),
    name: cityQuery.get(1),
};
group.cityid = city.cityid;
client.execute(
    "INSERT INTO \"group\" (\"groupid\", \"name\", \"description\", \"created_at\", \"extid\", \"slug\", \"active\", \"private\", \"members\", \"cityid\", \"organizer\", \"highres_link\", \"photo_link\", \"thumb_link\") VALUES ($1,$2,$3,NOW(),$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)",
                &[
                 &group.groupid,
                 &group.name,
                 &group.description,
                 &group.extid,
                 &group.slug,
                 &group.active,
                 &group.private,
                 &group.members,
                 &group.cityid,
                 &group.organizer,
                 &group.highres_link,
                 &group.photo_link,
                 &group.thumb_link,
                ],
        ).unwrap();
        println!("{:#?}", group);
        Ok(())
   }
fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = Connection::open(database_url)?;
    

    let mut stmt = conn.prepare(
        "select groupid,checksum from \"group\""
    )?;

    let groups = stmt.query_map([], |row| {
        Ok(Transmission {
            id: row.get(0)?,
            hash: row.get(1)?,
        })
    })?;

    for group in groups {
        println!("Found cat {:?}", group);
    }

    fetchMeetupGroup().unwrap();
    Ok(())
}
