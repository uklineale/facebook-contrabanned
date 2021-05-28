use std::string::String;
use uuid::Uuid;

pub struct ProxySet {
    pub id: String,
    pub real_url: String,
    pub fake_url: String
}

#[derive(Serialize, Deserialize)]
pub struct ProxySetRequest {
    pub real_url: String,
    pub fake_url: String
}

impl ProxySetRequest {
    pub fn convert(self) -> ProxySet {
        ProxySet {
            id: Uuid::new_v4().to_string(),
            real_url: self.real_url,
            fake_url: self.fake_url
        }
    }
}