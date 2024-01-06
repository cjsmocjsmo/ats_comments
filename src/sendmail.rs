use crate::types;
use log::info;
use std::process::Command;

pub fn comment_sendmail(com_info: types::Comment) {
    // let msgid = format!("-msgid {}", com_info.comid);
    // let email = format!("-email {}", com_info.email);
    // let comment = format!("-comment '{}'", com_info.comment);
    // println!("msgid: {}", msgid);
    // println!("email: {}", email);
    // println!("comment: {}", comment);
    let comment_str = format!("'{}'", com_info.comment);

    let output = Command::new("/usr/share/sendmail/sendmail/sendmail")
        .arg("-etype")
        .arg("com")
        .arg("-msgid")
        .arg(com_info.comid)
        .arg("-email")
        .arg(com_info.email)
        .arg("-comment")
        .arg(comment_str)
        .output()
        .expect("Failed to execute script");

    println!("Script output: {}", String::from_utf8_lossy(&output.stdout));
}

pub fn mail_test() {
    let output = Command::new("/usr/share/sendmail/sendmail/sendmail")
        .arg("-etype")
        .arg("com")
        .arg("-msgid")
        .arg("fuckmerunning".to_string())
        .output()
        .expect("Failed to execute script");

    println!("Script output: {}", String::from_utf8_lossy(&output.stdout));
    info!("Script output: {}", String::from_utf8_lossy(&output.stdout));
}

pub fn estimate_sendmail(esti_info: types::Estimate) {
    let msgid = format!("-estid {}", esti_info.estid);
    let name = format!("-name {}", esti_info.name);
    let address = format!("-address {}", esti_info.email);
    let city = format!("-city {}", esti_info.city);
    let phone = format!("-phone {}", esti_info.phone);
    let email = format!("-email {}", esti_info.email);
    let comment = format!("-comment '{}'", esti_info.comment);
    let intake = format!("-intake {}", esti_info.intake);
    let reqdate = format!("-reqdate {}", esti_info.reqdate);

    let output = Command::new("/usr/share/sendmail/sendmail/sendmail")
        .arg(msgid)
        .arg(name)
        .arg(address)
        .arg(city)
        .arg(phone)
        .arg(email)
        .arg(comment)
        .arg(intake)
        .arg(reqdate)
        .output()
        .expect("Failed to execute script");

    println!("Script output: {}", String::from_utf8_lossy(&output.stdout));
}
