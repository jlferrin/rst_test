extern crate chrono;
extern crate rusqlite;

use rusqlite::{Connection, Result, NO_PARAMS};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use rusqlite::types::ToSql;
use time::Timespec;

use std::collections::HashMap;

#[derive(Debug)]
struct Cat {
    name: String,
    date_of_birth: DateTime<Utc>,
    color: String
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {

//    let conn = Connection::open("../data/cats.db")?;
    let conn = Connection::open_in_memory().unwrap();


    // ---------- Person -------------------
    println!("Person");
    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
        NO_PARAMS,
    )?;

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
        &[&me.name as &ToSql, &me.time_created, &me.data],
    )?;

    let you = Person {
        id: 1,
        name: "Jonas".to_string(),
        time_created: time::get_time(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
        &[&you.name as &ToSql, &you.time_created, &you.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person").unwrap();
    let person_iter = stmt.query_map(NO_PARAMS, |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
            time_created: row.get(2),
            data: row.get(3),
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    // -------------------------------------

    let date = NaiveDate::from_ymd(2018,1,25);
    let time = NaiveTime::from_hms_milli(21,34,4,678);
    let dt = NaiveDateTime::new(date,time);
    let date_of_birth = Utc.from_utc_datetime(&dt);

    // ---------- foo ----------------------
    println!("foo");
    conn.execute_batch("CREATE TABLE foo (t TEXT, i INTEGER, f FLOAT, b BLOB)").unwrap();
    conn.execute("INSERT INTO foo (t) VALUES (?)", &[&date_of_birth]).unwrap();
    conn.execute("INSERT INTO foo (i) VALUES (?)", &[&5]).unwrap();
    conn.execute("INSERT INTO foo (i,t) VALUES (?,?)", &[&5 as &ToSql, &date_of_birth]).unwrap();

    let s: String = conn.query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0)).unwrap();
    let v1: DateTime<Utc> = conn.query_row("SELECT t FROM foo", NO_PARAMS, |r| r.get(0)).unwrap();

    println!("Cadena: {:?} {:?}", s,v1);  

    // ---------- Cats ---------------------
    println!("Cats");
    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             date_of_birth datetime,
             color_id integer not null references cat_colors(id)
         )",
        NO_PARAMS,
    )?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors{
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
    let last_id = conn.last_insert_rowid();

    for cat in catnames{
        conn.execute(
            "INSERT INTO cats (name, date_of_birth, color_id) values (?1, ?2, ?3)",
            &[ &cat.to_string() as &ToSql, &date_of_birth, &last_id]
        )?;
        }
    }



    let mut stmt = conn.prepare("SELECT c.name, date_of_birth, cc.name from cats c 
                                 INNER JOIN cat_colors cc ON cc.id = c.color_id;")?;
    let cats = stmt.query_map(NO_PARAMS, |row| {
        Cat {
            name: row.get(0),
            date_of_birth: row.get(1),
            color: row.get(2)
        }
    })?;
    
    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}

