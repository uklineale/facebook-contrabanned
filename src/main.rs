#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;

mod proxyset;
mod user_agent;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use rocket::State;
use rocket_contrib::json::Json;

use std::string::String;
use crate::proxyset::ProxySetRequest;
use crate::user_agent::UserAgent;
use rocket::response::Redirect;
use std::option::Option;
use std::option::Option::Some;

//TODO use facebook API to automate creation of honeypots (requestBin forks)
//TODO setup alarming on honeypots
//TODO filter on a wider set of keywords

const SITE_URL: &str = "https://facebook-contrabanned.herokuapp.com/";

struct RedirectMap{
    redirect_map: Arc<Mutex<HashMap<String, (String, String)>>>,
}

#[get("/<content_id>")]
fn get_content(content_id: String, redirects: State<RedirectMap>, user_agent: UserAgent) -> Option<Redirect> {
    let redirect_map = redirects.redirect_map.lock().unwrap();

    redirect_map.get(&content_id)
        .map(|(real_url, fake_url)| {
            if user_agent.user_agent.to_lowercase().contains("facebook") {
                Redirect::to(fake_url.clone())
            } else {
                Redirect::to(real_url.clone())
            }
        })
}

// TODO hook up to database
#[post("/", format = "application/json", data = "<urls>")]
fn create_redirect(urls: Json<ProxySetRequest>, redirects: State<RedirectMap>) -> String {
    let mut redirect_map = redirects.redirect_map.lock().unwrap();
    let proxy_set = urls.0.convert();

    redirect_map.insert(proxy_set.id.clone(), (proxy_set.real_url, proxy_set.fake_url));
    format!("{}{}", SITE_URL, proxy_set.id)
}

fn main() {
    rocket::ignite()
        .manage(RedirectMap {redirect_map: Arc::new(Mutex::new(HashMap::new()))})
        .mount("/", routes![get_content, create_redirect])
        .launch();
}
