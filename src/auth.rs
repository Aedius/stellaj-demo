use reqwest::header::HeaderMap;
use rocket::http::Status;
use rocket::response::{self, Redirect, Responder, Response};
use rocket::serde::json::serde_json::from_str;
use rocket::serde::Deserialize;
use urlencoding::encode;

use rocket::request::Request;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Tokens {
    access_token: String,
    expires_in: usize,
    refresh_expires_in: usize,
    refresh_token: String,
    token_type: String,
    id_token: String,
    // not-before-policy: usize,
    session_state: String,
    scope: String,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Tokens {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(Status::Ok)
            .raw_header("Content-Type", "text/html; charset=UTF-8")
            .raw_header("Authorization", format!("Bearer {}", self.access_token))
            .ok()
    }
}

#[get("/login")]
pub fn login() -> Redirect {
    let host = "http://localhost:8080";
    let realm = "horfimbor";
    let client_id = "horfimbor-demo";
    let redirect_uri = encode("http://localhost:8000/auth/login-token");

    Redirect::to(format!("{}/auth/realms/{}/protocol/openid-connect/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid", host, realm, client_id, redirect_uri))
}

#[get("/login-token?<session_state>&<code>")]
pub async fn login_token(session_state: &str, code: &str) -> Tokens {
    let _ = session_state;

    let host = "http://localhost:8080";
    let realm = "horfimbor";
    let client_id = "horfimbor-demo";
    let client_secret = "8gvRWJEwjtEV1RFpQCfybThiPgeZjl6y";
    let redirect_uri = encode("http://localhost:8000/auth/login-token");

    let mut header = HeaderMap::new();
    header.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "{}/auth/realms/{}/protocol/openid-connect/token",
            host, realm
        ))
        .body(format!(
            "grant_type=authorization_code&code={}&client_id={}&client_secret={}&redirect_uri={}",
            code, client_id, client_secret, redirect_uri
        ))
        .headers(header)
        .send()
        .await
        .unwrap();

    let text = res.text().await.unwrap();

    let tokens: Tokens = from_str(&text).unwrap();

    tokens
}

#[get("/welcome")]
pub fn welcome() -> String {
    "welcome".to_string()
}
