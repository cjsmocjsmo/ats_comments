use crate::accounts;
use crate::types;
use actix_web::{get, web, HttpResponse, Responder};
use std::env;

// use serde::{Deserialize, Serialize};
use log::info;

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Rusic Web Server is running!")
}

#[get("/comment/{name}/{email}/{comment}")]
pub async fn add_comment(f: web::Path<(String, String, String)>) -> impl Responder {
    let (name, email, comment) = f.into_inner();
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        // get account info and insert comment
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

// #[get("/jsonblob")]
// pub async fn jsonblob() -> impl Responder {
//     let dup_info = get_25_files();

//     HttpResponse::Ok().json(dup_info)
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct TransDupsEntry {
//     pub jsonfilename: String,
//     pub filename: String,
//     pub httpfilename: String,
//     pub duplicates: Vec<DupStruct>,
// }
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct DupStruct {
//     pub strdups: String,
//     pub httpdups: String,
// }
// fn get_25_files() -> Vec<TransDupsEntry> {
//     let json_path = env::var("COMSERV_JSON_PATH").unwrap();
//     let pagination = env::var("COMSERV_PAGINATION")
//         .unwrap()
//         .parse::<usize>()
//         .unwrap();
//     let mut files = Vec::new();

//     for entry in WalkDir::new(json_path) {
//         let entry = entry.unwrap();
//         if entry.file_type().is_file() {
//             let file_path = entry.path().to_str().unwrap().to_owned();
//             let file_contents = std::fs::read_to_string(file_path).unwrap();
//             let img_hash_struct: TransDupsEntry = serde_json::from_str(&file_contents).unwrap();
//             files.push(img_hash_struct);

//             if files.len() == pagination {
//                 break;
//             }
//         }
//     }

//     files
// }

// #[get("/delete_all/{filename}")]
// pub async fn delete_all(f: web::Path<String>) -> impl Responder {
//     let prefix = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ToRemove/";
//     let fname = f.into_inner();
//     let filename = format!("{}{}", prefix, fname);
//     println!("Filename: \n\t{}", filename);
//     //open filename read it's contents and delete all files
//     let file_contents = std::fs::read_to_string(&filename).unwrap();
//     let img_hash_struct: TransDupsEntry = serde_json::from_str(&file_contents).unwrap();
//     for dup in img_hash_struct.duplicates {
//         let prefix2 = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/";
//         let file_to_delete = format!("{}{}", prefix2, dup.strdups);
//         println!("File to delete: \n\t{}", file_to_delete);
//         std::fs::remove_file(file_to_delete).unwrap();
//     }
//     std::fs::remove_file(filename).unwrap();

//     HttpResponse::Ok().body("All Deleted!")
// }

// #[get("/delete_single/{filename}")]
// pub async fn delete_single(f: web::Path<String>) -> impl Responder {
//     let prefix = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/";
//     let fname = f.into_inner();
//     let filename = format!("{}{}", prefix, fname);
//     std::fs::remove_file(&filename).unwrap();

//     HttpResponse::Ok().body("Single File Deleted!")
// }
