use reqwest::header::HeaderMap;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::serde::json::serde_json::from_str;
use rocket::serde::{json::Json, Deserialize, Serialize};
use urlencoding::encode;

use rocket::Route;

pub fn get_route() -> Vec<Route> {
    return routes![login, login_token, welcome, login_token_get];
}

#[derive(Serialize, Deserialize, Debug)]
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

#[get("/login")]
pub fn login() -> Redirect {
    let host = "http://localhost:8080";
    let realm = "horfimbor";
    let client_id = "horfimbor-demo";
    let redirect_uri = encode("http://localhost:8000/auth/login-token");

    Redirect::to(format!("{}/auth/realms/{}/protocol/openid-connect/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid", host, realm, client_id, redirect_uri))
}

#[get("/login-token?<session_state>&<code>")]
pub async fn login_token_get(session_state: &str, code: &str) -> NamedFile {
    let _ = session_state;
    let _ = code;
    NamedFile::open("static/auth/login-token.html")
        .await
        .ok()
        .unwrap()
}

#[post("/login-token?<session_state>&<code>")]
pub async fn login_token(session_state: &str, code: &str) -> Json<Tokens> {
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

    Json(tokens)
}

#[get("/welcome")]
pub fn welcome() -> String {
    "welcome".to_string()
}
