#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod proxyset;

use std::env;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use rocket::{State, Data};
use rocket_contrib::json::Json;

use std::string::String;
use crate::proxyset::{ProxySet, ProxySetRequest};
use rocket::response::Redirect;
use rocket::Request;
use rocket::request;
use rocket::Outcome;
use rocket::request::FromRequest;
use std::option::Option;
use std::option::Option::Some;
use rocket::data::{FromData, Transform, FromDataSimple};
use rocket::http::Status;
use rocket::outcome::Outcome::{Success, Failure};

const FAKE_CONTENT: &str = "Hello, Facebook bot!";
const REAL_CONTENT: &str = "Hello, regular person!";
const SITE_URL: &str = "https://facebook-contrabanned.herokuapp.com/";

struct RedirectMap{
    redirect_map: Arc<Mutex<HashMap<String, (String, String)>>>,
}

struct UserAgent {
    user_agent: String
}

#[derive(Debug)]
enum UserAgentError {
    MultipleUserAgents
}

impl <'a, 'r> FromRequest<'a, 'r> for UserAgent {
    type Error = UserAgentError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let user_agent: Vec<_> = request.headers().get("User-Agent").collect();
        match user_agent.len() {
            0 => Success(UserAgent{
                user_agent: "N/A".to_string()
            }),
            1 => Success(UserAgent{
                user_agent: user_agent[0].to_string()
            }),
            _ => Failure((Status::BadRequest, UserAgentError::MultipleUserAgents))
        }
    }
}

#[get("/<content_id>")]
fn get_content(content_id: String, redirects: State<RedirectMap>, user_agent: UserAgent) -> Redirect {
    let redirect_map = redirects.redirect_map.lock().unwrap();
    let urls = redirect_map.get(&content_id).unwrap();

    println!("User Agent is {:?}", user_agent.user_agent);
    // Can use a 307 (::temporary()) to pass POST data, be a real proxy
    Redirect::to(urls.0.clone())
}

// TODO import your own Redirect class
// TODO Get POST body data
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
