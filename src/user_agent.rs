use std::string::ToString;
use rocket::request::FromRequest;
use rocket::{Request, Outcome};
use rocket::outcome::Outcome::{Success, Failure};
use rocket::http::Status;

pub struct UserAgent {
    pub user_agent: String
}

#[derive(Debug)]
pub enum UserAgentError {
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