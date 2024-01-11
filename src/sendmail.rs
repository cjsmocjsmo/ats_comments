use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

use crate::types;

#[tokio::main]
async fn send_com_mail(com: types::FullComment) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let client = Client::new(
        SendAPIVersion::V3,
        "public_key",
        "private_key",
    );

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        "porthose.cjsmo.cjsmo@gmail.com",
        "atsbot: ",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
    );

    message.push_recipient(Recipient::new("receiver@company.com"));

    // Finally send the message using the `Client`
    let response = client.send(message).await;

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    println!("{:?}", response);

    Ok(())
}


#[tokio::main]
async fn send_esti_mail(est: types::Estimate) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let client = Client::new(
        SendAPIVersion::V3,
        "public_key",
        "private_key",
    );

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        "mailjet_sender@company.com",
        "Mailjet Rust",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
    );

    message.push_recipient(Recipient::new("receiver@company.com"));

    // Finally send the message using the `Client`
    let response = client.send(message).await;

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    println!("{:?}", response);

    Ok(())
}