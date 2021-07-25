use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_header(header: &str) -> Option<BasicAuth> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();

        if split_vec.len() != 2 {
            return None;
        }

        if split_vec[0] != "Basic" {
            return None;
        }

        Self::from_base64(split_vec[1])
    }

    fn from_base64(base64_str: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_str).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split_vec = decoded_str.split(":").collect::<Vec<_>>();

        if split_vec.len() != 2 {
            return None;
        }

        let (username, password) = (split_vec[0], split_vec[1]);

        if username != "wutong" || password != "123456" {
            return None;
        }

        Some(BasicAuth {
            username: username.to_string(),
            password: password.to_string(),
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    // Basic Auth header
    // Authorization: Basic base64
    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");

        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_header(header_auth) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
