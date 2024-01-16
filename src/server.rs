use crate::accounts;
use crate::sendmail;
use crate::types;
use actix_web::{get, web, HttpResponse, Responder, web::Redirect};
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
        let acctidz: String = row.get(1).unwrap();
        let comidz: String = row.get(2).unwrap();
        let namez: String = row.get(3).unwrap();
        let emailz: String = row.get(4).unwrap();
        let commentz: String = row.get(5).unwrap();
        let ratingz: String = row.get(6).unwrap();
        let datez: String = row.get(7).unwrap();
        let acceptedz: String = row.get(8).unwrap();
        let rejectedz: String = row.get(9).unwrap();

        let comment = types::FullComment {
            acctid: acctidz.to_string(),
            comid: comidz.to_string(),
            name: namez.to_string(),
            email: emailz.to_string(),
            comment: commentz.to_string(),
            rating: ratingz.to_string(),
            date: datez.to_string(),
            accepted: acceptedz.to_string(),
            rejected: rejectedz.to_string(),
        };
        info!("Comment: {:?}", comment);
        comment_vec.push(comment);



        
    }

    HttpResponse::Ok().json(comment_vec)
}

#[get("/addcom/{name}/{email}/{comment}/{rating}")]
pub async fn add_comment(f: web::Path<(String, String, String, String)>) -> impl Responder {
    let (name, email, comment, rating) = f.into_inner();
    let comidz = accounts::create_hash(comment.clone());
    info!("name: {:#?}", name);
    info!("email: {:#?}", email);
    info!("comment: {:#?}", comment);
    info!("rating: {:#?}", rating);
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        let acct_info = accounts::account_info_from_email(email.clone());
        let acctid = &acct_info[0].acctid;
        let datae = &acct_info[0].date;
        let commet = types::FullComment {
            acctid: acctid.to_string(),
            comid: comidz.clone(),
            name: name.clone(),
            email: email.clone(),
            comment: comment.clone(),
            rating: rating.clone(),
            date: datae.to_string(),
            accepted: "None".to_string(),
            rejected: "None".to_string(),
        };
        info!("has_account Comment: {:#?}", commet);
        let com_serv_comments_db =
            env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
        let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
        conn.execute(
            "INSERT INTO comments (acctid, comid, name, email, comment, date, accepted, rejected) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&commet.acctid, &commet.comid, &commet.name, &commet.email, &commet.comment, &commet.date, &commet.accepted, &commet.rejected],
        )
        .expect("unable to insert comment");

        let mailz = sendmail::send_com_mail(commet).await;
        match mailz {
            Ok(_) => info!("Mail Sent"),
            Err(e) => info!("Mail Error: {:?}", e),
        };
    } else {
        let acct_info = accounts::create_account(name.clone(), email.clone());
        let acctid = &acct_info.acctid;
        let datae = &acct_info.date;
        let fullcomment = types::FullComment {
            acctid: acctid.to_string(),
            comid: comidz.clone(),
            name: name.clone(),
            email: email.clone(),
            comment: comment.clone(),
            rating: rating.clone(),
            date: datae.to_string(),
            accepted: "None".to_string(),
            rejected: "None".to_string(),
        };
        info!("create_account Comment: {:#?}", fullcomment);
        let com_serv_comments_db =
            env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
        let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
        conn.execute(
            "INSERT INTO comments (acctid, comid, name, email, comment, rating, date, accepted, rejected) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[&fullcomment.acctid, &fullcomment.comid, &fullcomment.name, &fullcomment.email, &fullcomment.comment, &rating, &fullcomment.date, &fullcomment.accepted, &fullcomment.rejected],
        )
        .expect("unable to insert comment");

        let mailz = sendmail::send_com_mail(fullcomment).await;
        match mailz {
            Ok(_) => info!("Mail Sent"),
            Err(e) => info!("Mail Error: {:?}", e),
        };
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
            estid: row.get(2).unwrap(),
            name: row.get(3).unwrap(),
            address: row.get(4).unwrap(),
            city: row.get(5).unwrap(),
            phone: row.get(6).unwrap(),
            email: row.get(7).unwrap(),
            comment: row.get(8).unwrap(),
            intake: row.get(9).unwrap(),
            reqdate: row.get(10).unwrap(),
            completed: row.get(11).unwrap(),
        };
        info!("Estimate: {:?}", estimate);
        estimate_vec.push(estimate);
    }

    HttpResponse::Ok().json(estimate_vec)
}

#[get("/addesti/{name}/{address}/{city}/{phone}/{email}/{comment}/{reqdate}")]
pub async fn add_estimate(
    f: web::Path<(String, String, String, String, String, String, String)>,
) -> impl Responder {
    let (name, address, city, phone, email, comment, reqdate) = f.into_inner();
    let eid = name.clone() + &address + &city + &phone + &email;
    let estidz = accounts::create_hash(eid.clone());
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        let acct_info = accounts::account_info_from_email(email.clone());
        let acctid = &acct_info[0].acctid;
        let today = Local::now().format("%Y-%m-%d").to_string();
        let estimate = types::Estimate {
            acctid: acctid.to_string(),
            estid: estidz.clone(),
            name: name.clone(),
            address: address.clone(),
            city: city.clone(),
            phone: phone.clone(),
            email: email.clone(),
            comment: comment.clone(),
            intake: today.clone(),
            reqdate: reqdate.clone(),
            completed: "No".to_string(),
        };
        info!("has_account Estimate: {:#?}", estimate);
        let com_serv_estimates_db =
            env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
        let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
        conn.execute(
            "INSERT INTO estimates (acctid, estid, name, address, city, phone, email, comment, intake, reqdate, completed) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            &[&estimate.acctid, &estimate.estid, &estimate.name, &estimate.address, &estimate.city, &estimate.phone, &estimate.email, &estimate.comment, &estimate.intake, &estimate.reqdate, &estimate.completed],
        )
        .expect("unable to insert estimate");

        let mailz = sendmail::send_esti_mail(estimate).await;
        match mailz {
            Ok(_) => info!("Esti Mail Sent"),
            Err(e) => info!("Mail Error: {:?}", e),
        };
    } else {
        let acct_info = accounts::create_account(name.clone(), email.clone());
        let acctid = &acct_info.acctid;
        let today = Local::now().format("%Y-%m-%d").to_string();
        let estimate = types::Estimate {
            acctid: acctid.to_string(),
            estid: estidz.clone(),
            name: name.clone(),
            address: address.clone(),
            city: city.clone(),
            phone: phone.clone(),
            email: email.clone(),
            comment: comment.clone(),
            intake: today.clone(),
            reqdate: reqdate.clone(),
            completed: "No".to_string(),
        };
        info!("create_account Estimate: {:#?}", estimate);
        let com_serv_estimates_db =
            env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
        let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
        conn.execute(
            "INSERT INTO estimates (acctid, estid, name, address, city, phone, email, comment, intake, reqdate, completed) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            &[&estimate.acctid, &estimate.estid, &estimate.name, &estimate.address, &estimate.city, &estimate.phone, &estimate.email, &estimate.comment, &estimate.intake, &estimate.reqdate, &estimate.completed],
        ).expect("unable to insert estimate");

        let mailz = sendmail::send_esti_mail(estimate).await;
        match mailz {
            Ok(_) => info!("Esti Mail Sent"),
            Err(e) => info!("Mail Error: {:?}", e),
        };
    };

    HttpResponse::Ok().body("\nEstimate inserted into db\n")
}

#[get("/accept/{msgid}")]
pub async fn accept_comment(id: web::Path<String>) -> impl Responder {
    let msgid = id.into_inner();
    let todays_date = Local::now().format("%Y-%m-%d").to_string();
    info!("msgid: {:#?}", msgid);
    info!("todays_date: {:#?}", todays_date);
    let com_serv_comments_db =
        env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
    conn.execute(
        "UPDATE comments SET accepted = ?1 WHERE comid = ?2",
        &[&todays_date, &msgid],
    )
    .expect("unable to update comment");

    Redirect::to("https://alpha-tree.biz/accepted").permanent();

    HttpResponse::Ok().body("\nComment Updated\n")
}

#[get("/reject/{msgid}")]
pub async fn reject_comment(id: web::Path<String>) -> impl Responder {
    let msgid = id.into_inner();
    let todays_date = Local::now().format("%Y-%m-%d").to_string();
    info!("msgid: {:#?}", msgid);
    info!("todays_date: {:#?}", todays_date);
    let com_serv_comments_db =
        env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
    conn.execute(
        "UPDATE comments SET rejected = ?1 WHERE comid = ?2",
        &[&todays_date, &msgid],
    )
    .expect("unable to update comment");

    Redirect::to("https://alpha-tree.biz/rejected").permanent();

    HttpResponse::Ok().body("\nComment Rejected\n")
}

#[get("/completed/{msgid}")]
pub async fn esti_complete(id: web::Path<String>) -> impl Responder {
    let msgid = id.into_inner();
    let todays_date = Local::now().format("%Y-%m-%d").to_string();
    let com_serv_estimates_db =
        env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
    let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
    conn.execute(
        "UPDATE estimates SET completed = ?1 WHERE estid = ?2",
        &[&todays_date, &msgid],
    )
    .expect("unable to update estimate");

    Redirect::to("https://alpha-tree.biz/completed").permanent();

    HttpResponse::Ok().body("\nEstimate Completed\n")
}
