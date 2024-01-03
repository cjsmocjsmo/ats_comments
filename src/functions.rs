// use actix_web::{get, web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
// use walkdir::WalkDir;
// use env_logger;
use log::info;
// use crate::types;

pub fn gen_server_addr() -> SocketAddr {
    let raw_addr = env::var("COMSERV_RAW_HTTP").expect("COMSERV_RAW_HTTP not set");
    let compare_v4_addr = Ipv4Addr::from_str(&raw_addr).unwrap();
    let port: u16 = env::var("COMSERV_PORT")
        .expect("COMSERV_PORT not set")
        .parse()
        .unwrap();
    let socket = SocketAddr::new(IpAddr::V4(compare_v4_addr), port);
    info!("Server Address: {:?}", socket);

    socket
}

pub fn db_file_checks() -> bool {
    // let mut ac_dir = 0;
    // let comserv_base_path = env::var("COMSERV_BASE_PATH").expect("COMSERV_BASE_PATH not set");
    // // check if comserv_base_path exists if not create it
    // if !std::path::Path::new(&comserv_base_path).exists() {
    //     std::fs::create_dir(&comserv_base_path).expect("Unable to create base path");
    //     info!("Created base path: {:?}", &comserv_base_path);
    //     ac_dir += 1;
    // } else {
    //     info!("Base path exists: {:?}", &comserv_base_path);
    // }
    let mut db_dir = 0;
    let comserv_db_path = env::var("COMSERV_DB_PATH").expect("COMSERV_DB_PATH not set");
    // check if comserv_db_path exists if not create it
    if !std::path::Path::new(&comserv_db_path).exists() {
        std::fs::create_dir(&comserv_db_path).expect("Unable to create db path");
        info!("Created db path: {:?}", &comserv_db_path);
        db_dir += 1;
    } else {
        info!("Db path exists: {:?}", &comserv_db_path);
    }
    let mut acct = 0;
    let comserv_acct_db_file = env::var("COMSERV_ACCT_DB").expect("COMSERV_ACCT_DB not set");
    // check if comserv_acct_db_file exists if not create it
    if !std::path::Path::new(&comserv_acct_db_file).exists() {
        std::fs::File::create(&comserv_acct_db_file).expect("Unable to create accouts db file");
        info!("Created accounts db file: {:?}", &comserv_acct_db_file);
        acct += 1;
    } else {
        info!("Accounts db file exists: {:?}", &comserv_acct_db_file);
    }
    let mut auth = 0;
    let comserv_auth_db = env::var("COMSERV_AUTH_DB").expect("COMSERV_AUTH_DB not set");
    // check if comserv_auth_db exists if not create it
    if !std::path::Path::new(&comserv_auth_db).exists() {
        std::fs::File::create(&comserv_auth_db).expect("Unable to create the auth db file");
        info!("Created auth db file: {:?}", &comserv_auth_db);
        auth += 1;
    } else {
        info!("Auth db file exists: {:?}", &comserv_auth_db);
    }
    let mut comments = 0;
    let comserv_comments_db = env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    // check if comserv_comments_db exists if not create it
    if !std::path::Path::new(&comserv_comments_db).exists() {
        std::fs::File::create(&comserv_comments_db).expect("Unable to create the comments db file");
        info!("Created comments db file: {:?}", &comserv_comments_db);
        comments += 1;
    } else {
        info!("Comments db file exists: {:?}", &comserv_comments_db);
    }
    let mut estimates = 0;
    let comserv_esti_db = env::var("COMSERV_ESTIMATEs_DB").expect("COMSERV_ESTIMATEs_DB not set");
    // check if comserv_esti_db exists if not create it
    if !std::path::Path::new(&comserv_esti_db).exists() {
        std::fs::File::create(&comserv_esti_db).expect("Unable to create the estimates db file");
        info!("Created estimates db file: {:?}", &comserv_esti_db);
        estimates += 1;
    } else {
        info!("Estimates db file exists: {:?}", &comserv_esti_db);
    }

    let total = db_dir + acct + auth + comments + estimates;
    if total == 0 || total == 6 {
        return true;
    } else {
        return false;
    }
}

