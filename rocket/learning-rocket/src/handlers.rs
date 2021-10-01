use std::path::{Path, PathBuf};

use rocket::{
    data::ToByteUnit,
    form::Form,
    fs::{NamedFile, TempFile},
    get,
    http::{ContentType, Cookie, CookieJar, Status},
    post,
    request::{FromRequest, Outcome, Request},
    serde::{json::Json, Deserialize},
    State,
};

use super::config;

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}

// 动态参数 /hello/wutong/12/true
#[get("/hello/<name>/<age>/<cool>")]
pub fn dynamic_paths(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

// /path/1/2/3/asdasdasd
#[get("/path/<path..>")]
pub fn path_buf(path: std::path::PathBuf) -> String {
    let s = path.to_str();

    if let Some(s) = s {
        s.to_string()
    } else {
        "Error".to_string()
    }
}

// 演示 handler 中读取配置
#[get("/config")]
pub fn get_config(c: &State<config::Config>) -> String {
    c.custom.aaa.clone()
}

pub struct AdminUser {
    pub name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let admin = request.query_value::<bool>("admin");
        if let Some(admin) = admin {
            match admin {
                Ok(admin) => {
                    if admin {
                        Outcome::Success(AdminUser {
                            name: "Is Admin User".to_string(),
                        })
                    } else {
                        Outcome::Success(AdminUser {
                            name: "Is Common User".to_string(),
                        })
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    Outcome::Failure((Status::InternalServerError, ()))
                }
            }
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

// /guard             => 403
// /guard?admin=123   => 500
// /guard?admin=false => 200, Is Common User
// /guard?admin=true  => 200, Is Common User
#[get("/guard")]
pub fn get_request_guard(admin: AdminUser) -> String {
    admin.name.clone()
}

// 设置读取 cookie
#[get("/cookie")]
pub fn test_cookies(cookies: &CookieJar<'_>) -> String {
    let placeholder = "+1";
    let num = cookies.get("number").map(|c| c.value());
    let s = format!(
        "{} {}",
        if let Some(n) = num { n } else { placeholder },
        placeholder
    );
    cookies.add(Cookie::new("number", s));

    if let Some(num) = num {
        num.to_string()
    } else {
        placeholder.to_string()
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserData {
    name: String,
    age: u16,
}

// format: 'json' 只允许 content-type=application/json 的请求
#[post("/json", /* format = "json", */ data = "<user>")]
pub fn body_json(user: Json<UserData>) -> String {
    format!("{} {}", user.name, user.age)
}

#[derive(Debug, FromForm)]
pub struct UserFormData {
    name: String,
    age: u16,
}

// content-type = x-www-form-urlencoded
#[post("/form", data = "<user>")]
pub fn body_form(user: Form<UserFormData>) -> String {
    format!("{} {}", user.name, user.age)
}

#[derive(Debug, FromForm)]
pub struct UploadForm<'r> {
    #[field(validate = ext(ContentType::JPEG))]
    #[field(validate = len(..32.mebibytes()))]
    image: TempFile<'r>,
    file_name: String,
}

#[post("/upload-with-tempfile", data = "<data>")]
pub async fn upload_with_tempfile(mut data: Form<UploadForm<'_>>) -> std::io::Result<String> {
    let file_name = data.file_name.clone();
    let file_path = &Path::new("upload/").join(file_name);
    data.image.persist_to(file_path).await?;
    Ok(file_path.to_str().unwrap_or_default().to_string())
}

#[get("/file/<file..>")]
pub async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}
