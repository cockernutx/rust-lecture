use manganis::Asset;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use gloo::console::__macro::JsValue;
use gloo::utils::window;

#[derive(Debug, Clone)]
pub enum GetAssetError {
    RequestError(String),
    FsError(String)
}

impl Display for GetAssetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(why) => write!(f, "{why}"),
            Self::FsError(why) => write!(f, "{why}"),

        }
    }
}
impl Error for GetAssetError {}

impl From<reqwest::Error> for GetAssetError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value.to_string())
    }
}

impl From<JsValue> for GetAssetError {
    fn from(value: JsValue) -> Self {
        Self::RequestError(value.as_string().unwrap())
    }
}

impl From<std::io::Error> for GetAssetError {
    fn from(value: std::io::Error) -> Self {
        Self::FsError(value.to_string())
    }
}

pub async fn get_asset(asset: &Asset) -> Result<String, GetAssetError> {
    if cfg!(feature = "web") {
        let url = format!("{}{}", window().location().origin()?, asset.to_string());
        let req = reqwest::get(url).await?;
        let text = req.text().await?;
        Ok(text)
    } else if cfg!(feature = "desktop") {
        let path = asset.local;
        let text = fs::read_to_string(path)?;
        Ok(text)
    } else {
        panic!("platform unsupported")
    }
}
