use std::env;

use log::info;

pub fn set_env_vars() {
    let base_path = env::var("COMSERV_BASE_PATH");
    if base_path.is_err() {
        env::set_var("COMSERV_BASE_PATH", "/usr/share/ats-comments");
    };
    let comserv_db_path = env::var("COMSERV_DB_PATH");
    if comserv_db_path.is_err() {
        env::set_var("COMSERV_DB_PATH", "/usr/share/ats-comments/db");
    };
    let comserv_uploads_path = env::var("COMSERV_UPLOADS_PATH");
    if comserv_uploads_path.is_err() {
        env::set_var(
            "COMSERV_UPLOADS_PATH",
            "/usr/share/ats-comments/uploads",
        );
    };
    let comserv_acct_db = env::var("COMSERV_ACCT_DB");
    if comserv_acct_db.is_err() {
        env::set_var("COMSERV_ACCT_DB", "/usr/share/ats-comments/db/accounts.db");
    };
    let comserv_auth_db = env::var("COMSERV_AUTH_DB");
    if comserv_auth_db.is_err() {
        env::set_var("COMSERV_AUTH_DB", "/usr/share/ats-comments/db/auth.db");
    };
    let comserv_comments_db = env::var("COMSERV_COMMENTS_DB");
    if comserv_comments_db.is_err() {
        env::set_var(
            "COMSERV_COMMENTS_DB",
            "/usr/share/ats-comments/db/comments.db",
        );
    };
    let comserv_esti_db = env::var("COMSERV_ESTIMATEs_DB");
    if comserv_esti_db.is_err() {
        env::set_var(
            "COMSERV_ESTIMATEs_DB",
            "/usr/share/ats-comments/db/estimates.db",
        );
    };
    let comserv_raw_http = env::var("COMSERV_RAW_HTTP");
    if comserv_raw_http.is_err() {
        env::set_var("COMSERV_RAW_HTTP", "192.168.0.91");
    };
    let comserv_http = env::var("COMSERV_HTTP_ADDR");
    if comserv_http.is_err() {
        env::set_var("COMSERV_HTTP_ADDR", "http://192.168.0.91");
    };
    let comserv_port = env::var("COMSERV_PORT");
    if comserv_port.is_err() {
        env::set_var("COMSERV_PORT", "8181");
    };
    info!("Environment variables set")
}
