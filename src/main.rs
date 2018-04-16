#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::io::prelude::*;
use std::io::{BufReader};
use std::fs;
use std::sync::Mutex;
use rocket::State;
use rocket_contrib::{Json, Value};

mod cors;
mod utils;

type Messages = Vec<String>;
type CacheMessages = Mutex<Messages>;

#[derive(FromForm)]
struct MessageConfig {
    count: usize,
}

#[get("/")]
fn get_messages(messages: State<CacheMessages>) -> Json<Value> {
    let list = messages.lock().unwrap();
    let items = utils::get_uniq_items(list.iter().collect(), 10);
    Json(json!(items))
}

#[get("/?<config>")]
fn get_messages_with_params(config: MessageConfig, messages: State<CacheMessages>) -> Json<Value> {
    let list = messages.lock().unwrap();
    let items = utils::get_uniq_items(list.iter().collect(), config.count);
    Json(json!(items))
}

fn main() {
    let mut buf = BufReader::new(fs::File::open("commit_messages.txt").unwrap());
    let mut line = String::new();
    let mut messages: Messages = vec![];

    while buf.read_line(&mut line).unwrap() > 0 {
        messages.push(line.trim().to_string());
        line.clear();
    }

    rocket::ignite()
        .attach(cors::CORS)
        .mount("/", routes![
               get_messages,
               get_messages_with_params,
        ])
        .manage(Mutex::new(messages))
        .launch();
}
