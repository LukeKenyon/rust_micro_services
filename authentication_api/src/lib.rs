#![allow(unused)]

pub mod database {
    pub mod mongo_db;
}

pub mod utils {
    pub mod load_settings;
    pub mod password_utils;
}

pub mod models {
    pub mod authentication;
    pub mod user;
}

pub mod services {
    pub mod certification;
    pub mod user;
}

pub mod handlers {

    pub mod authentication;
    pub mod user;
}

pub mod routes {

    pub mod authentication;
    pub mod user;
}
