use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use std::env;
use env_logger;
use log::info;


pub mod accounts;
pub mod envvars;
pub mod functions;
pub mod server;
pub mod types;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    let _vars = envvars::set_env_vars();
    
    let db_check = functions::db_file_checks();
    info!("db_check result: {:?}", db_check); // Log the result of db_check
    
    let uploads_path = env::var("COMSERV_UPLOADS_PATH").unwrap();
    let socket = functions::gen_server_addr();
    

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
            .service(server::add_estimate)
            // .service(functions::delete_all)
            // .service(functions::delete_single)
            .service(fs::Files::new("/uploads", uploads_path.clone()).show_files_listing())
    })
    .bind(socket)?
    .run()
    .await
}
