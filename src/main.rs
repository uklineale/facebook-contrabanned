#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate uuid;

use std::env;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use rocket::State;
use rocket::response::Redirect;
use uuid::Uuid;

const FAKE_CONTENT: &str = "Hello, Facebook bot!";
const REAL_CONTENT: &str = "Hello, regular person!";
const SITE_URL: &str = "https://facebook-contrabanned.herokuapp.com/";

struct RedirectMap{
    redirect_map: Arc<Mutex<HashMap<String, (String, String)>>>,
}

#[get("/<content_id>")]
fn get_content(content_id: String, redirects: State<RedirectMap>) -> String {
    let redirect_map = redirects.redirect_map.lock().unwrap();
    let urls = redirect_map.get(&content_id).unwrap();
    format!("Your redirect URL are:\n{}\n{}", urls.0, urls.1)
    // Can use a 307 (::temporary()) to pass POST data, be a real proxy
//    Redirect::to(target_url)
}

// TODO import your own Redirect class
// TODO Get POST body data
// TODO hook up to database
#[post("/", format = "application/json", data = "<urls>")]
fn create_redirect(urls: Json<Redirect>, redirects: State<RedirectMap>) -> String {

    let mut redirect_map = redirects.redirect_map.lock().unwrap();

    let redirect_id = Uuid::new_v4().to_string();
    redirect_map.insert(redirect_id, (real_redirect_url, fake_redirect_url));
    format!("Your redirect is stored at {}{}", SITE_URL, redirect_id)
}

fn main() {
    rocket::ignite()
        .manage(RedirectMap {redirect_map: Arc::new(Mutex::new(HashMap::new()))})
        .mount("/", routes![get_content, create_redirect])
        .launch();
}
