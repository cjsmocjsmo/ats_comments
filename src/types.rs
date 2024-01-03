use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub acctid: String,
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub id: i32,
    pub acctid: String,
    pub token: String,
    pub indate: String,
    pub outdate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub acctid: String,
    pub comment: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Estimate {
    pub id: i32,
    pub acctid: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub date: String,
}