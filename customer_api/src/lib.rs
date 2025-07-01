#![allow(unused)]

pub mod database {
    pub mod mongo_db;
}

pub mod utils {
    pub mod load_settings;
    pub mod password_utils;
}

pub mod models {
    pub mod address;
    pub mod contact;
    pub mod customer;
    pub mod messages;
}
pub mod services {
    pub mod certification;
    pub mod customer;
    pub mod messaging;
}

pub mod handlers {
    pub mod customer;
}

pub mod routes {
    pub mod customer;
}
