#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]
#![feature(type_name_of_val)]
use std::fmt::Debug;
use std::{any::type_name_of_val, io::ErrorKind};

#[macro_use]
extern crate dotenv;
extern crate url;
extern crate urlencoding;
use serde::{Serialize, Deserialize};
use postgres::{Client, NoTls, Error};

use dotenv::dotenv;
use std::{env, println, panic };
use chrono::{NaiveDateTime, DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH, SystemTime};

use reqwest;
use rusqlite::Error as SqliteError;
use chrono::format::ParseError;
use rusqlite::{Connection, Result};
use regex::Regex;


use url::Url;
use urlencoding::encode;

fn remove_html_tags(input: &str) -> String {
    let re = Regex::new(r"<[^>]+>").unwrap();
    re.replace_all(input, "").to_string()
}

fn modify_medium_image_url(url_str: &str,fit: u16) -> String { 
    let url = match Url::parse(url_str) {
        Ok(url) => url,
        Err(_) => panic!("error"), // Invalid URL
    };

    // Check if the host is a Medium image URL
    if url.host_str() != Some("cdn-images-1.medium.com") {
        return url.to_string(); // Not a Medium image URL
    }
    
    let re = Regex::new(r"max\/[0-9]+").unwrap();
    return re.replace_all(url_str, "max/".to_owned() + &fit.to_string()).to_string()
}


#[derive(Debug)]
struct Transmission {
    id: i32,
    hash: String,
}
#[derive(Debug)]
struct Article {
    articleid: i32,
    name: String,
    description: String,
    source: String,
    author: String,
    time_m: String,
    publish_at: std::time::SystemTime,
    link: String,
    created_at: std::time::SystemTime,
    highres_link: String,
    photo_link: String,
    thumb_link: String,
    extid: String,
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
struct Event {
    eventid: i32,
    name: String,
    description: String,
    location: String,
    created_at: std::time::SystemTime,
    extid: String,
    image: String,
    groupid: i32,
    status: String,
    in_person: bool,
    is_online: bool,
    time: std::time::SystemTime,
    duration: i32,
    link: String,
    waitlist_count: i32,
    fee: bool,
    yes_rsvp_count: i32,
    rsvp_limit: Option<i32>,
    highres_link: String,
    photo_link: String,
    thumb_link: String,
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
#[derive(Deserialize)]
#[derive(Debug)]
struct SeqGroup {
    id: i32,
    created_at: String,
    new: bool,
}
#[derive(Deserialize)]
#[derive(Debug)]
struct MeetupEvent {
    id: String,
    duration: i32,
    name: String,
    date_in_series_pattern: bool,
    status: String,
    time: u64,
    local_date: String,
    local_time: String,
    updated: i64,
    utc_offset: i32,
    waitlist_count: i32,
    yes_rsvp_count: i32,
    is_online_event: bool,
    link: String,
    eventType: String,
    description: String,
    visibility: String,
    member_pay_fee: bool,
    venue: Option<Venue>,
    rsvp_limit: Option<i32>,
}
#[derive(Debug)]
#[derive(Deserialize)]
struct Venue {
    id: i32,
    name: String,
    lat: f64,
    lon: f64,
    repinned: bool,
    address_1: String,
    city: String,
    country: String,
    localized_country_name: String,
    zip: String,
    state: String,
}
#[derive(Deserialize)]
#[derive(Debug)]
struct MediumFeed {
    title: String,
    link: String,
    description: String,
    image: String,
}
#[derive(Deserialize)]
#[derive(Debug)]
struct MediumArticle {
    guid: String,
    link: String,
    author: String,
    pubDate: String,
    title: String,
    thumbnail: String,
    description: String,
    content: String,
    categories: Vec<String>,
}
#[derive(Deserialize)]
#[derive(Debug)]
struct MediumResponse {
    status: String,
    feed: MediumFeed,
    items: Vec<MediumArticle>,
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
fn convert_to_system_time(timestamp: u64) -> SystemTime {
    let duration = Duration::from_millis(timestamp);
    UNIX_EPOCH + duration
}
fn parse_string_to_system_time(datetime_str: &str) -> Result<SystemTime, ParseError> {
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")?;
    let utc_datetime: DateTime<Utc> = DateTime::from_utc(datetime, Utc);
    let system_time = SystemTime::from(utc_datetime);
    Ok(system_time)
}
fn new_seq_article(value: &String) -> Result<SeqGroup> {

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = Connection::open(database_url).unwrap();

    conn.execute("insert into s_article(value) values(?)",&[&value]).unwrap();

    let mut stmt = conn.prepare(
        "select id,created_at from \"s_article\" where value = ?"
        ).unwrap();

     Ok(stmt.query_row(&[&value], |row| {
        Ok(SeqGroup {
            id: row.get(0)?,
            created_at: row.get(1)?,
            new: true,
        })
    }).unwrap())
}
fn new_seq_group(value: &String) -> Result<SeqGroup> {

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = Connection::open(database_url).unwrap();

    conn.execute("insert into s_group(value) values(?)",&[&value]).unwrap();

    let mut stmt = conn.prepare(
        "select id,created_at from \"s_group\" where value = ?"
        ).unwrap();

     Ok(stmt.query_row(&[&value], |row| {
        Ok(SeqGroup {
            id: row.get(0)?,
            created_at: row.get(1)?,
            new: true,
        })
    }).unwrap())
}
fn new_seq_event(value: &String) -> Result<SeqGroup> {

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = Connection::open(database_url).unwrap();

    conn.execute("insert into s_event(value) values(?)",&[&value]).unwrap();

    let mut stmt = conn.prepare(
        "select id,created_at from \"s_event\" where value = ?"
        ).unwrap();

     Ok(stmt.query_row(&[&value], |row| {
        Ok(SeqGroup {
            id: row.get(0)?,
            created_at: row.get(1)?,
            new: true,
        })
    }).unwrap())
}
   fn fetchMeetupEvent(groupid: i32,group_name: &String,group_item: &Group) -> Result<(), Error>    {
        let meetings = reqwest::blocking::get("https://api.meetup.com/".to_owned()+ &group_name + "/events").unwrap().json::<Vec<MeetupEvent>>().unwrap();
        let mut event_items: Vec<Event> = Vec::new();

        let pg_database_url = env::var("PG_DATABASE_URL").expect("set PG_DATABASE_URL");
        let mut client = Client::connect(&pg_database_url,NoTls).unwrap();
        for meeting in meetings {
            let extid = String::from("m") + &meeting.id;
            let seq_event = new_seq_event(&extid).unwrap();
            let photo_album = reqwest::blocking::get("https://api.meetup.com/".to_owned()+ &group_name + "/events/" + &meeting.id + "/photos").unwrap().json::<Vec<MeetupGroupPhoto>>().unwrap();
            let highres_link = if let Some(first_photo) = photo_album.first() {
                first_photo.highres_link.clone()
            } else {
                group_item.highres_link.clone()
            };
            let thumb_link = if let Some(first_photo) = photo_album.first() {
                first_photo.highres_link.clone()
            } else {
                group_item.thumb_link.clone()
            };
            let photo_link = if let Some(first_photo) = photo_album.first() {
                first_photo.highres_link.clone()
            } else {
                group_item.photo_link.clone()
            };

            let meeting_item = Event{
                eventid: seq_event.id,
                name: meeting.name,
                highres_link: highres_link,
                thumb_link: thumb_link,
                photo_link: photo_link,
                description: remove_html_tags(&meeting.description),
                extid:extid, 
                location: String::from("m") + &meeting.id,
                status: meeting.status,
                groupid: groupid,
                in_person: meeting.eventType == "PHYSICAL",
                is_online: meeting.is_online_event,
                link: meeting.link,
                rsvp_limit: meeting.rsvp_limit,
                waitlist_count: meeting.waitlist_count,
                yes_rsvp_count: meeting.yes_rsvp_count,
                created_at: parse_string_to_system_time(&seq_event.created_at).unwrap(),
                image: String::from(""),
                duration: meeting.duration,
                time: convert_to_system_time(meeting.time).try_into().unwrap(),
                fee: meeting.member_pay_fee
            };
            if seq_event.new {

            client.execute(
                "INSERT INTO \"event\" (\"eventid\", \"name\", \"description\", \"created_at\", \"extid\", \"location\", \"groupid\", \"status\", \"in_person\", \"time\", \"duration\", \"link\", \"waitlist_count\", \"is_online\", \"yes_rsvp_count\", \"fee\", \"rsvp_limit\",\"highres_link\",\"photo_link\",\"thumb_link\") VALUES
($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,$19,$20);",
                &[
                &meeting_item.eventid,
                &meeting_item.name,
                &meeting_item.description,
                &meeting_item.created_at,
                &meeting_item.extid,
                &meeting_item.location,
                &meeting_item.groupid,
                &meeting_item.status,
                &meeting_item.in_person,
                &meeting_item.time,
                &meeting_item.duration,
                &meeting_item.link,
                &meeting_item.waitlist_count,
                &meeting_item.is_online,
                &meeting_item.yes_rsvp_count,
                &meeting_item.fee,
                &meeting_item.rsvp_limit,
                &meeting_item.highres_link,
                &meeting_item.photo_link,
                &meeting_item.thumb_link,
                ],
                ).unwrap();
            }
            event_items.push( meeting_item);
        }

       Ok(())
   }
   fn fetch_article(author: String) -> Result<(), Error>    {
       
        let resp = reqwest::blocking::get("https://api.rss2json.com/v1/api.json?rss_url=https://medium.com/feed/@".to_owned() + &author).unwrap().json::<MediumResponse>().unwrap();
        let mut article_items: Vec<Article> = Vec::new();
        let pg_database_url = env::var("PG_DATABASE_URL").expect("set PG_DATABASE_URL");
        let mut client = Client::connect(&pg_database_url,NoTls).unwrap();

        let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
        let conn = Connection::open(database_url).unwrap();
        for article in resp.items {
            let extid = String::from("m") + &article.guid;
            let article_seq = new_seq_article(&extid).unwrap();
        let mut stmt = conn.prepare(
            "select id,created_at from \"s_article\" where value = ?"
            ).unwrap();

        let seq_article = match stmt.query_row(&[&extid], |row| {
            Ok(SeqGroup {
                id: row.get(0)?,
                created_at: row.get(1)?,
                new: false,
            })
        }) {
            Ok(rows) => rows,
            Err(e) => match e {
                SqliteError::QueryReturnedNoRows => new_seq_article(&extid).unwrap(),
                _ => panic!("test"),
            },
            _ => panic!("test"),
        }; 
            println!("{}",seq_article.id);
            let img = modify_medium_image_url(&article.thumbnail,400);
            let mut article_item = Article{
                articleid: article_seq.id,
                name: article.title,
                source: String::from("medium"),
                author: article.author,
                time_m: String::from("5"),
                publish_at: parse_string_to_system_time(&article.pubDate).unwrap(),
                description: remove_html_tags(&article.description).to_string(),
                extid: extid,
                link: article.link,
                created_at: parse_string_to_system_time(&article_seq.created_at).unwrap(),
                highres_link: img.clone(),
                photo_link: img.clone(),
                thumb_link: img.clone(),
            };
            article_item.highres_link =modify_medium_image_url(&article_item.highres_link,200);
            article_item.thumb_link =modify_medium_image_url(&article_item.highres_link,200);
            article_item.photo_link =modify_medium_image_url(&article_item.highres_link,200);
            if seq_article.new {
            client.execute(
                "INSERT INTO \"article\" (\"articleid\", \"created_at\", \"extid\", \"title\", \"description\", \"time_m\", \"publish_at\", \"source\", \"link\", \"author\", \"highres_link\", \"photo_link\", \"thumb_link\") VALUES
($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13);",
                &[
                &article_item.articleid,
                &article_item.created_at,
                &article_item.extid,
                &article_item.name,
                &article_item.description,
                &article_item.time_m,
                &article_item.publish_at,
                &article_item.source,
                &article_item.link,
                &article_item.author,
                &article_item.highres_link,
                &article_item.photo_link,
                &article_item.thumb_link,
                ],
                ).unwrap();
                }
            article_items.push( article_item);
        }
        Ok(())
   }
   fn fetchMeetupGroup(group_name: String) -> Result<(), Error>    {
       
        let resp = reqwest::blocking::get("https://api.meetup.com/".to_owned() + &group_name).unwrap().json::<MeetupGroup>().unwrap();

        let mut group = Group{
            groupid: 3,
            name: resp.name.clone(),
            slug: resp.urlname,
            description: remove_html_tags(&resp.description),
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

        let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
        let conn = Connection::open(database_url).unwrap();


        let mut stmt = conn.prepare(
            "select id,created_at from \"s_group\" where value = ?"
            ).unwrap();

        let seq_group = match stmt.query_row(&[&group.extid], |row| {
            Ok(SeqGroup {
                id: row.get(0)?,
                created_at: row.get(1)?,
                new: false,
            })
        }) {
            Ok(rows) => rows,
            Err(e) => match e {
                SqliteError::QueryReturnedNoRows => new_seq_group(&group.extid).unwrap(),
                _ => panic!("test"),
            },
            _ => panic!("test"),
        }; 
        let city = City{
            cityid: cityQuery.get(0),
            name: cityQuery.get(1),
        };
        group.groupid = seq_group.id;
        group.cityid = city.cityid;
        if seq_group.new {
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
        }
        fetchMeetupEvent(group.groupid,&group_name,&group).unwrap();
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


    let mut article_items = Vec::new();
    article_items.push(String::from("josephreis"));
    article_items.push(String::from("tkudlicka"));
    article_items.push(String::from("barrmoses"));
    article_items.push(String::from("zachl-quinn"));
    article_items.push(String::from("darshilp"));
    article_items.push(String::from("viv1kv"));
    article_items.push(String::from("Divithraju"));
    article_items.push(String::from("mateusclira"));
    article_items.push(String::from("frank-andrade"));
    article_items.push(String::from("molly.ruby"));
    article_items.push(String::from("ben.putney"));
    let mut group_items = Vec::new();
    group_items.push(String::from("the-vancouver-business-network"));
    group_items.push(String::from("LegalHackersVAN"));
    group_items.push(String::from("internet-masterminds"));
    group_items.push(String::from("vancouver-entrepreneurs-toastmasters-club"));
    group_items.push(String::from("Vancouver-Single-Professionals"));
    group_items.push(String::from("Vancouver-Data-Visualization"));
    group_items.push(String::from("Product-Hunt-Vancouver"));
    group_items.push(String::from("TechVancouverOrg"));
    group_items.push(String::from("vanpydata"));
    group_items.push(String::from("Vancouver-Startup-Founder-101"));
    group_items.push(String::from("learndatascience"));
    // for article in article_items {
    //     println!("{}",&article);
    //     fetch_article(article).unwrap();
    // }
    for group in group_items {
        fetchMeetupGroup(group).unwrap();
    }
    Ok(())
}
