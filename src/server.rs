use crate::accounts;
use crate::types;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::prelude::*;
use log::info;
use std::env;

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("\nATS Comments Server up and running!\n")
}

#[get("/allcom")]
pub async fn all_comments() -> impl Responder {
    let com_serv_comments_db =
        env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM comments").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut comment_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let comment = types::Comment {
            acctid: row.get(1).unwrap(),
            comment: row.get(2).unwrap(),
            date: row.get(3).unwrap(),
        };
        info!("Comment: {:?}", comment);
        comment_vec.push(comment);
    }

    let mut fullcomvec = Vec::new();
    for com in comment_vec {
        let acctidz = &com.acctid;
        let acct_info = accounts::account_info_from_acctid(acctidz.to_string());
        let name = &acct_info[0].name;
        let email = &acct_info[0].email;
        let date = com.date;
        let full_comment = types::FullComment {
            acctid: acctidz.to_string(),
            name: name.to_string(),
            email: email.to_string(),
            comment: com.comment,
            date: date,
        };
        info!("Full Comment: {:#?}", full_comment);
        fullcomvec.push(full_comment);
    }

    let fullcomvec_str = serde_json::to_string(&fullcomvec).unwrap();

    HttpResponse::Ok().body(format!("{:#?}", fullcomvec_str))
}

#[get("/addcom/{name}/{email}/{comment}")]
pub async fn add_comment(f: web::Path<(String, String, String)>) -> impl Responder {
    let (name, email, comment) = f.into_inner();
    info!("name: {:#?}", name);
    info!("email: {:#?}", email);
    info!("comment: {:#?}", comment);
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        let acct_info = accounts::account_info_from_email(email.clone());
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

    HttpResponse::Ok().body("Comment inserted into db\n")
}

#[get("/allesti")]
pub async fn all_estimates() -> impl Responder {
    let com_serv_estimates_db =
        env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
    let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM estimates").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut estimate_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let estimate = types::Estimate {
            acctid: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            address: row.get(3).unwrap(),
            city: row.get(4).unwrap(),
            phone: row.get(5).unwrap(),
            email: row.get(6).unwrap(),
            comment: row.get(7).unwrap(),
            intake: row.get(8).unwrap(),
            reqdate: row.get(9).unwrap(),
        };
        info!("Estimate: {:?}", estimate);
        estimate_vec.push(estimate);
    }

    let fullestvec_str = serde_json::to_string(&estimate_vec).unwrap();

    HttpResponse::Ok().body(format!("{:#?}", fullestvec_str))
}

#[get("/addesti/{name}/{address}/{city}/{phone}/{email}/{comment}/{reqdate}")]
pub async fn add_estimate(
    f: web::Path<(String, String, String, String, String, String, String)>,
) -> impl Responder {
    let (name, address, city, phone, email, comment, reqdate) = f.into_inner();
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        let acct_info = accounts::account_info_from_email(email.clone());
        let acctid = &acct_info[0].acctid;
        let today = Local::now().format("%Y-%m-%d").to_string();
        let estimate = types::Estimate {
            acctid: acctid.to_string(),
            name: name.clone(),
            address: address.clone(),
            city: city.clone(),
            phone: phone.clone(),
            email: email.clone(),
            comment: comment.clone(),
            intake: today.clone(),
            reqdate: reqdate.clone(),
        };
        info!("has_account Estimate: {:#?}", estimate);
        let com_serv_estimates_db =
            env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
        let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
        conn.execute(
            "INSERT INTO estimates (acctid, name, address, city, phone, email, comment, intake, reqdate) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[&estimate.acctid, &estimate.name, &estimate.address, &estimate.city, &estimate.phone, &estimate.email, &estimate.comment, &estimate.intake, &estimate.reqdate],
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
            phone: phone.clone(),
            email: email.clone(),
            comment: comment.clone(),
            intake: today.clone(),
            reqdate: reqdate.clone(),
        };
        info!("create_account Estimate: {:#?}", estimate);
        let com_serv_estimates_db =
            env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
        let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
        conn.execute(
            "INSERT INTO estimates (acctid, name, address, city, phone, email, comment, intake, reqdate) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[&estimate.acctid, &estimate.name, &estimate.address, &estimate.city, &estimate.phone, &estimate.email, &estimate.comment, &estimate.intake, &estimate.reqdate],
        ).expect("unable to insert estimate");
    };

    HttpResponse::Ok().body("\nEstimate inserted into db\n")
}
