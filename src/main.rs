#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod proxyset;
mod user_agent;

use std::env;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use rocket::{State, Data};
use rocket_contrib::json::Json;

use std::string::String;
use crate::proxyset::{ProxySet, ProxySetRequest};
use crate::user_agent::UserAgent;
use rocket::response::Redirect;
use rocket::Request;
use rocket::request;
use rocket::Outcome;
use rocket::request::FromRequest;
use std::option::Option;
use std::option::Option::Some;
use rocket::data::{FromData, Transform, FromDataSimple};
use rocket::outcome::Outcome::{Success, Failure};

const FAKE_CONTENT: &str = "Hello, Facebook bot!";
const REAL_CONTENT: &str = "Hello, regular person!";
const SITE_URL: &str = "https://facebook-contrabanned.herokuapp.com/";

struct RedirectMap{
    redirect_map: Arc<Mutex<HashMap<String, (String, String)>>>,
}

#[get("/<content_id>")]
fn get_content(content_id: String, redirects: State<RedirectMap>, user_agent: UserAgent) -> Redirect {
    let redirect_map = redirects.redirect_map.lock().unwrap();
    let (real_url, fake_url) = redirect_map.get(&content_id).unwrap();

    // Can use a 307 (::temporary()) to pass POST data, be a real proxy
    if (user_agent.user_agent.to_lowercase().contains("facebook")) {
        Redirect::to(fake_url.clone())
    } else {
        Redirect::to(real_url.clone())
    }
}

// TODO hook up to database
#[post("/", format = "application/json", data = "<urls>")]
fn create_redirect(urls: Json<ProxySetRequest>, redirects: State<RedirectMap>) -> String {

    let mut redirect_map = redirects.redirect_map.lock().unwrap();

    let proxy_set = urls.0.convert();

    redirect_map.insert(proxy_set.id.clone(), (proxy_set.real_url, proxy_set.fake_url));
    format!("Your redirect is stored at {}{}", SITE_URL, proxy_set.id)
}

fn main() {
    rocket::ignite()
        .manage(RedirectMap {redirect_map: Arc::new(Mutex::new(HashMap::new()))})
        .mount("/", routes![get_content, create_redirect])
        .launch();
}
