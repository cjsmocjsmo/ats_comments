use crate::accounts;
use crate::types;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::prelude::*;
use log::info;
use std::env;

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Rusic Web Server is running!")
}

#[get("/comment/{name}/{email}/{comment}")]
pub async fn add_comment(f: web::Path<(String, String, String)>) -> impl Responder {
    let (name, email, comment) = f.into_inner();
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        let acct_info = accounts::account_info(email.clone());
        let acctid = &acct_info[0].acctid;
        let datae = &acct_info[0].date;
        let commet = types::Comment {
            acctid: acctid.to_string(),
            comment: comment.clone(),
            date: datae.to_string(),
        };
        info!("has_account Comment: {:#?}", commet);
        let com_serv_comments_db =
            env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
        let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
        conn.execute(
            "INSERT INTO comments (acctid, comment, date) VALUES (?1, ?2, ?3)",
            &[&commet.acctid, &commet.comment, &commet.date],
        )
        .expect("unable to insert comment");
    } else {
        let acct_info = accounts::create_account(name.clone(), email.clone());
        let acctid = &acct_info.acctid;
        let datae = &acct_info.date;
        let comment = types::Comment {
            acctid: acctid.to_string(),
            comment: comment.clone(),
            date: datae.to_string(),
        };
        info!("create_account Comment: {:#?}", comment);
        let com_serv_comments_db =
            env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
        let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
        conn.execute(
            "INSERT INTO comments (acctid, comment, date) VALUES (?1, ?2, ?3)",
            &[&comment.acctid, &comment.comment, &comment.date],
        )
        .expect("unable to insert comment");
    };

    HttpResponse::Ok().body("Comment inserted into db")
}

#[get("/estimate/{name}/{address}/{city}/{state}/{phone}/{email}/{intake}/{reqdate}")]
pub async fn add_estimate(
    f: web::Path<(String, String, String, String, String, String, String)>,
) -> impl Responder {
    let (name, address, city, state, phone, email, reqdate) = f.into_inner();
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        let acct_info = accounts::account_info(email.clone());
        let acctid = &acct_info[0].acctid;
        let today = Local::now().format("%Y-%m-%d").to_string();
        let estimate = types::Estimate {
            acctid: acctid.to_string(),
            name: name.clone(),
            address: address.clone(),
            city: city.clone(),
            state: state.clone(),
            phone: phone.clone(),
            email: email.clone(),
            intake: today.clone(),
            reqdate: reqdate.clone(),
        };
        info!("has_account Estimate: {:#?}", estimate);
        let com_serv_estimates_db =
            env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
        let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
        conn.execute(
            "INSERT INTO estimates (acctid, name, address, city, state, phone, email, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[&estimate.acctid, &estimate.name, &estimate.address, &estimate.city, &estimate.state, &estimate.phone, &estimate.email, &estimate.intake, &estimate.reqdate],
        )
        .expect("unable to insert estimate");
    } else {
        let acct_info = accounts::create_account(name.clone(), email.clone());
        let acctid = &acct_info.acctid;
        let today = Local::now().format("%Y-%m-%d").to_string();
        let estimate = types::Estimate {
            acctid: acctid.to_string(),
            name: name.clone(),
            address: address.clone(),
            city: city.clone(),
            state: state.clone(),
            phone: phone.clone(),
            email: email.clone(),
            intake: today.clone(),
            reqdate: reqdate.clone(),
        };
        info!("create_account Estimate: {:#?}", estimate);
        let com_serv_estimates_db =
            env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
        let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
        conn.execute(
            "INSERT INTO estimates (acctid, name, address, city, state, phone, email, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[&estimate.acctid, &estimate.name, &estimate.address, &estimate.city, &estimate.state, &estimate.phone, &estimate.email, &estimate.intake, &estimate.reqdate],
        ).expect("unable to insert estimate");
    };

    HttpResponse::Ok().body("Estimate inserted into db")
}
