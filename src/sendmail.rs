use crate::types;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};
use std::env;

#[tokio::main]
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
        "ATSBOT: A New Comment Has Been Posted",
        Some("A New Comment Has Been Posted!".to_string()),
        Some("ATSBOT: A New Comment Has Been Posted".to_string()),
    );

    let html1 = format!(
        "
        <div><p>{}</p><p>{}</p><p>{}</p></div>
        <a href='https://atstest.xyz/accept/{}'><button>Accept</button></a>
        <a href='https://atstest.xyz/reject/{}'><button>Reject</button></a>
        ",
        com.email, com.comment, com.rating, com.comid, com.comid,
    );
    message.html_part = Some(html1.to_string());

    message.push_recipient(Recipient::new("porthose.cjsmo.cjsmo@gmail.com"));

    // Finally send the message using the `Client`
    let response = client.send(message).await;

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    println!("{:?}", response);

    Ok(())
}

#[tokio::main]
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
        "ATSBOT: A New Comment Has Been Posted",
        Some("ATSBOT: A New Comment Has Been Posted!".to_string()),
        Some("ATSBOT: A New Comment Has Been Posted".to_string()),
    );

    let html1 = format!(
        "
        <div><p>{}</p><p>{}</p><p>{}</p><p>{}</p><p>{}</p><p>{}</p></div>
        <a href='https://atstest.xyz/accept/{}'><button>Accept</button></a>
        <a href='https://atstest.xyz/reject/{}'><button>Reject</button></a>
        ",
        esti.name, esti.address, esti.city, esti.phone, esti.email, esti.comment, esti.estid, esti.estid,
    );

    message.html_part = Some(html1.to_string());
    message.push_recipient(Recipient::new("porthose.cjsmo.cjsmo@gmail.com"));

    // Finally send the message using the `Client`
    let response = client.send(message).await;

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    println!("{:?}", response);

    Ok(())
}
