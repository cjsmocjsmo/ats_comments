use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use env_logger;
use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

pub mod accounts;
pub mod db;
pub mod envvars;
pub mod functions;
// pub mod sendmail;
pub mod server;
pub mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let _vars = envvars::set_env_vars();

    let db_check = functions::db_file_checks();
    info!("db_check result: {:?}", db_check);
    if db_check == 6 {
        let _create_tables = db::create_tables();
    }

    let uploads_path = env::var("COMSERV_UPLOADS").unwrap();
    let socket = functions::gen_server_addr();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            "/etc/letsencrypt/live/atstest.xyz/privkey.pem",
            SslFiletype::PEM,
        )
        .unwrap();
    builder
        .set_certificate_chain_file("/etc/letsencrypt/live/atstest.xyz/fullchain.pem")
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(server::test)
            .service(server::add_comment)
            .service(server::all_comments)
            .service(server::add_estimate)
            .service(server::all_estimates)
            .service(server::accept_comment)
            .service(server::reject_comment)
            .service(server::esti_complete)
            .service(fs::Files::new("/uploads", uploads_path.clone()).show_files_listing())
    })
    .bind_openssl(socket, builder)?
    .run()
    .await
}
