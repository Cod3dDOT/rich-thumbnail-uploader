use serde::Deserialize;

#[derive(Deserialize)]
pub struct ImgurResponse {
    pub data: ImgurData,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct ImgurData {
    pub link: String,
}
