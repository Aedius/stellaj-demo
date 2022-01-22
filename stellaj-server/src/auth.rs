

use jsonwebtokens as jwt;
use jwt::{Algorithm, AlgorithmID, Verifier, raw};
use reqwest::header::HeaderMap;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::serde_json::from_str;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::request::{self, Outcome, Request, FromRequest};
use rocket::{Error, Route};
use serde_json::json;
use serde_json::value::Value;
use urlencoding::encode;


pub fn get_route() -> Vec<Route> {
    return routes![login, login_token, welcome];
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
    #[serde(rename = "not-before-policy")]
    not_before_policy: usize,
    session_state: String,
    scope: String,
}

#[get("/login")]
pub fn login() -> Redirect {
    let host = "http://localhost:8080";
    let realm = "horfimbor";
    let client_id = "horfimbor-demo";
    let redirect_uri = encode("http://localhost:8000/");

    Redirect::to(format!("{}/auth/realms/{}/protocol/openid-connect/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid", host, realm, client_id, redirect_uri))
}

#[post("/login-token?<session_state>&<code>")]
pub async fn login_token(session_state: &str, code: &str) -> Json<Tokens> {
    let _ = session_state;

    let host = "http://localhost:8080";
    let realm = "horfimbor";
    let client_id = "horfimbor-demo";
    let client_secret = "Cg2qEiaojoTJwrRrmMMtsKYlLHw16Mz7";
    let redirect_uri = encode("http://localhost:8000/");

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

    error!("test : {}", text);

    let tokens: Tokens = from_str(&text).unwrap();

    Json(tokens)
}

#[derive(Debug)]
pub struct KeyCloakUser{
    pub username: String
}

#[derive(Debug)]
pub enum KeyCloakError {
    Missing,
    Invalid,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims{
    preferred_username: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for KeyCloakUser {
    type Error = KeyCloakError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::BadRequest, KeyCloakError::Missing)),
            Some(key) if key.len() > 7 => {
                let key = &key[7..key.len()];

                let sig = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5rLHfJBzm3Z+abdmeDNYSvwqWou5TGbyqi4JZJ7PcL90gGI8ZGAIbmpCntSTzB6mtaaccpkYPBv3E7zbEmWxrOf/Y+8Wutmc2V4fDEWivSMbsUwOJBhOaRY0mkaKzrX1b4jgex6Fbox9RiJrr2g1SXgncVBuKhbe1rwFla+LpTMMxK7uvJRiXyOSMFAq51d0qbQVzwz+qF4oQ5nqn2WWuI+ZER+Pog5afVpJOIs1gDKNIMSCMq64q3WwFXqTiJZaYTq6ap2KoqoNy8euwRVhxMzt47j/EdZIYC6iruzAOTWl6JKfsKH8uNTQ1v5BsTXmv7EwKf1bK+LJQEJziwLAqQIDAQAB
-----END PUBLIC KEY-----";

                let mut alg = Algorithm::new_rsa_pem_verifier(AlgorithmID::RS256, sig.as_bytes()).unwrap();
                alg.set_kid("OMilXIowKglmPO1ATFfTPxzP9QwOIFgV-Ya_Ntx4dsw");
                let verifier = Verifier::create()
                    .issuer("http://localhost:8080/auth/realms/horfimbor")
                    .audience("account")
                    .ignore_iat()
                    .build().unwrap();

                let claims: Value = verifier.verify(&key, &alg).unwrap();

                let claims:Claims = serde_json::from_value(claims).unwrap();

                info!("claims : {:?}", claims);

                Outcome::Success(KeyCloakUser{
                    username: claims.preferred_username
                })
            },
            Some(_) => Outcome::Failure((Status::BadRequest, KeyCloakError::Missing)),
        }

    }
}


#[get("/welcome")]
pub fn welcome(user: KeyCloakUser) -> String {
    user.username.to_string()
}
