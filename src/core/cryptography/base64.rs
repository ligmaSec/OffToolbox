#[derive(serde::Serialize, serde::Deserialize)]
pub enum Base64Mode {
    Encode,
    Decode,
}


impl Default for Base64Mode {
    fn default() -> Self {
        Base64Mode::Encode
    }
}


