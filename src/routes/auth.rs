use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginSchema {
    pub login: String,
    pub password: String,
    pub undelete: Option<bool>,
    pub captcha_key: Option<String>,
    pub login_source: Option<String>,
    pub gift_code_sku_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterSchema {
    pub username: String,
    pub passowrd: Option<String>,
    pub consent: bool,
    pub email: Option<String>,
    pub fingerprint: Option<String>,
    pub invite: Option<String>,
    pub date_of_birth: Option<String>,
    pub gift_code_sku_id: Option<String>,
    pub captcha_key: Option<String>,
    pub promotional_email_opt_in: Option<bool>,
}

#[post("/login", format = "json", data = "<body>")]
pub fn auth_login(body: Json<LoginSchema>) -> &'static str {
    "Hello, world!"
}

#[post("/register", format = "json", data = "<body>")]
pub fn auth_register(body: Json<RegisterSchema>) -> &'static str {
    "Hello, world!"
}
