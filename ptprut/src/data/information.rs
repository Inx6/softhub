use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Info{
    pub data: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apk{
    pub name: String,
    pub url: String,
    pub ic: String,
    pub ty: String,
    pub label: String,
    pub version: String,
    pub package: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config{
    pub ip: String,
    pub port: String,
    pub db_name: String,
    pub db_pwd: String,
    pub db: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Add{
    pub nick: String,
    pub name: String,
    pub pwd: String,
    pub avatar: String,
    pub level: String,
    pub status: i32,
    pub time: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login{
    pub name: String,
    pub pwd: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tonge{
    pub data: String,
    pub size: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ia{
    pub size: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Addr{
    pub city: String,
    pub province: String,
    pub country: String,
    pub isp: String,
}