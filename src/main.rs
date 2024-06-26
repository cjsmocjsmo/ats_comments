use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
// use env_logger;
use log::info;
use std::env;
use std::io::Read;
use std::fs::File;
// use dotenv::dotenv;
use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslMethod},
};

pub mod accounts;
pub mod db;
// pub mod envvars;
pub mod functions;
pub mod sendmail;
pub mod server;
pub mod types;

#[tokio::main]
// #[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("/usr/share/ats_comments/ats_comments/atscomments.env").ok();
 
    env_logger::init();
    // let _vars = envvars::set_env_vars();

    let db_check = functions::db_file_checks();
    info!("db_check result: {:?}", db_check);
    // if db_check == 6 {
    //     let _create_tables = db::create_tables();
    // }
    let static_path = env::var("COMSERV_STATIC").unwrap();
    let uploads_path = env::var("COMSERV_UPLOADS").unwrap();
    // let socket = functions::gen_server_addr();

    let key_file_path = env::var("COMSERV_KEY_PEM").unwrap();
    let cert_file_path = env::var("COMSERV_CERT_PEM").unwrap();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key(&load_encrypted_private_key(key_file_path))
        .unwrap();

    builder
        .set_certificate_chain_file(cert_file_path).unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(server::test)
            .service(server::com_upload)
            .service(server::all_comments)
            .service(server::add_estimate)
            .service(server::all_estimates)
            .service(server::accept_comment)
            .service(server::reject_comment)
            .service(server::esti_complete)
            .service(server::backup_file)
            .service(fs::Files::new("/static", static_path.clone()).show_files_listing())
            .service(fs::Files::new("/uploads", uploads_path.clone()).show_files_listing())
    })
    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await

}

fn load_encrypted_private_key(key_path: String) -> PKey<Private> {
    let mut file = File::open(key_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    PKey::private_key_from_pem_passphrase(&buffer, b"password").unwrap()
}