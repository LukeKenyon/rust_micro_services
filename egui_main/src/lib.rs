use serde::{Deserialize, Serialize};

pub mod services {
    pub mod authentication;
}

pub mod screens {
    pub mod home;
    pub mod login;
    pub mod settings;
}

#[derive(PartialEq)]
pub enum ContentScreens {
    Login,
    Settings,
    Home,
}
#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    LoginSuccess(String), // token
    LoginFailed(String),
}
