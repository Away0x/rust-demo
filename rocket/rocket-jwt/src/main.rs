use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::{serde_json::json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{post, routes};

const SECRET_KEY: &[u8] = b"secret";

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    sub: String,
    exp: usize,
    company: String,
}

struct Token;

impl Token {
    fn from_auth_header(header: &str) -> Option<Token> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }

        if split_vec[0] != "Bearer" {
            return None;
        }

        Self::from_jwt(split_vec[1])
    }

    fn from_jwt(token_str: &str) -> Option<Token> {
        let va = Validation {
            sub: Some("b@b.com".to_string()),
            ..Validation::default()
        };

        match decode::<Claims>(&token_str, &DecodingKey::from_secret(SECRET_KEY), &va) {
            Ok(c) => {
                println!("validate {:?}", c.claims);
                return Some(Token);
            }
            Err(_) => None,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");

        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_auth_header(header_auth) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[post("/sign")]
async fn get_jwt() -> Value {
    let my_claims = Claims {
        sub: "b@b.com".to_string(),
        company: "Company".to_string(),
        exp: 1000000000000000, // 过期时间
    };

    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET_KEY),
    ) {
        Ok(t) => t,
        Err(_) => panic!("token encode error"),
    };

    json!({ "token": token })
}

#[post("/validate")]
async fn validation_jwt(_auth: Token) -> Value {
    json!("ok")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/jwt", routes![get_jwt, validation_jwt])
        .launch()
        .await?;

    Ok(())
}
