use eyre::{WrapErr, Result, eyre};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct NYTResponse {
    pub id: usize,
    pub solution: String,
    pub print_date: String,
    pub days_since_launch: usize,
    pub editor: String
}

pub fn get_daily_wordle() -> Result<NYTResponse> {
    let now = time::OffsetDateTime::now_local().wrap_err("could not determine local timezone")?;
    let url = format!("https://www.nytimes.com/svc/wordle/v2/{}.json", now.date());

    let response = reqwest::blocking::get(url)?;

    if response.status() == 200 {
        let json: NYTResponse = response.json().wrap_err("invalid json from nytimes.com")?;
        Ok(json)
    } else {
        Err(eyre!("bad response from nytimes.com"))
    }
}
