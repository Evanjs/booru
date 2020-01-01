#[derive(Serialize, Deserialize, Debug)]
pub struct ServerError {
    pub success: bool,
    pub message: String,
    pub backtrace: Vec<String>,
}
