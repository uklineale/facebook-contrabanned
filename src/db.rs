use postgres::{Connection, TlsMode};
use postgres_openssl::OpenSsl;
use std::env::var;


const GET_QUERY: &str = "SELECT real_url, fake_url FROM proxy_sets WHERE id=$1";
const POST_QUERY: &str = "INSERT INTO proxy_sets (id, real_url, fake_url) VALUES ($1, $2, $3)";


pub fn get_proxy_set(id: String) -> (String, String) {
    let conn_url = var("DATABASE_URL")
        .expect("Database url could not be found.");
    let mut handshake = OpenSsl::new().unwrap();
    handshake.danger_disable_hostname_verification(true);
    let conn = Connection::connect(conn_url,
                                   TlsMode::Prefer(&handshake))
        .expect("Could not connect to database.");

    let mut real = "".to_string();
    let mut fake = "".to_string();
    
    for row in &conn.query(GET_QUERY, &[&id]).unwrap() {
        real = row.get("real_url");
        fake = row.get("fake_url");
    }

    return (real, fake);
}

//pub fn create_proxy_set((real_url: String, fake_url: String)) {
//    format!("nope");
//}

