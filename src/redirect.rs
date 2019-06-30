#derive[Serialize, Deserialize]
pub struct Redirect {
    id: String,
    real_url: String,
    fake_url: String
}