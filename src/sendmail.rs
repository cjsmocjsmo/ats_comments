use crate::types;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};
use std::env;
use log::info;

pub async fn send_com_mail(
    com: types::FullComment,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let api_key = env::var("COMSERV_API_KEY").unwrap();
    let sec_key = env::var("COMSERV_SEC_KEY").unwrap();
    let client = Client::new(SendAPIVersion::V3, api_key.as_str(), sec_key.as_str());

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        "porthose.cjsmo.cjsmo@gmail.com",
        "ATSBOT",
        Some("ATSBOT: New Comment".to_string()),
        Some("ATSBOT: New Comment".to_string()),
    );

    let html1 = format!(
        "
        <div><p>Email: {}</p><p>Comment: {}</p><p>Date: {}</p></div>
        <a href='https://atstest.xyz/accept/{}'>
        <button style='color:white;background-color:green;padding:5px;'>Accept</button></a>
        <a href='https://atstest.xyz/reject/{}'>
        <button style='color:white;background-color:red;padding:5px;'>Reject</button></a>
        ",
        com.email, com.comment, com.rating, com.comid, com.comid,
    );
    message.html_part = Some(html1.to_string());

    info!("Com Mail Message: {:?}", message);

    message.push_recipient(Recipient::new("porthose.cjsmo.cjsmo@gmail.com"));

    // Finally send the message using the `Client`
    let _response = client.send(message).await;

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    // info!("Com Mail Response: {:?}", response);

    Ok(())
}

// #[tokio::main]
pub async fn send_esti_mail(
    esti: types::Estimate,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let client = Client::new(SendAPIVersion::V3, "public_key", "private_key");

    // Create your a `Message` instance with the minimum required values
    let sender = "porthose.cjsmo.cjsmo@gmail.com";
    let mut message = Message::new(
        &sender,
        "ATSBOT",
        Some("ATSBOT: New Comment".to_string()),
        Some("ATSBOT: New Comment".to_string()),
    );

    let html1 = format!(
        "
        <div>
            <p>{}</p><p>{}</p>
            <p>{}</p><p>{}</p>
            <p>{}</p><p>{}</p>
        </div>
        <a href='https://atstest.xyz/completed/{}'>
        <button style='color:white;background-color:blue;padding:8px;'>Accept</button></a>
        ",
        esti.name, esti.address, esti.city, esti.phone, esti.email, esti.comment, esti.estid,
    );

    message.html_part = Some(html1.to_string());

    info!("Esti Mail Message: {:?}", message);
    
    message.push_recipient(Recipient::new("porthose.cjsmo.cjsmo@gmail.com"));

    // Finally send the message using the `Client`
    let _response = client.send(message);

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    // println!("{:?}", response);

    Ok(())
}
