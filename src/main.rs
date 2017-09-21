#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket_contrib::Json;
use rocket::State;
use rocket::fairing::AdHoc;

type Counters = Mutex<HashMap<String, i32>>;

#[get("/counters")]
fn get_counters(counters: State<Counters>) -> Json<HashMap<String, i32>> {
    let counters = counters.lock().unwrap();
    Json(counters.clone())
}

#[post("/counters/<thing>/increment")]
fn increment_counter(counters: State<Counters>, thing: String) -> Json<HashMap<String, i32>> {
    let mut counters = counters.lock().unwrap();
    {
        let entry = counters.entry(thing).or_insert(0);
        *entry += 1;
    }

    Json(counters.clone())
}

#[post("/counters/<thing>/decrement")]
fn decrement_counter(counters: State<Counters>, thing: String) -> Json<HashMap<String, i32>> {
    let mut counters = counters.lock().unwrap();
    {
        let entry = counters.entry(thing).or_insert(0);
        *entry -= 1;
    }

    Json(counters.clone())
}

fn main() {
    let mut counters:HashMap<String, i32> = HashMap::new();

    counters.insert("Biler".to_string(), 2);
    counters.insert("Båter".to_string(), 4);
    counters.insert("Hunder".to_string(), 12);
    counters.insert("Katter".to_string(), 14);

    rocket::ignite()
        .manage(Mutex::new(counters))
        .mount("/", routes![get_counters, increment_counter, decrement_counter])
        .attach(AdHoc::on_response(|_, response| {
            response.set_raw_header("Access-Control-Allow-Origin", "*");
        }))
        .launch();
}