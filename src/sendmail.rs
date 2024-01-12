use crate::types;
use log::info;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};
use std::env;

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
        <div>
            <p>{}</p>
            <p>{}</p>
            <p>{}</p>
        </div>
        <a href='https://atstest.xyz/accept/{}'>
        <button class='acceptBtn'>Accept</button></a>
        <a href='https://atstest.xyz/reject/{}'>
        <button class='rejectBtn'>Reject</button></a>
        <style>
            p {{
                font-size: 12px;
                font-weight: bold;
            }}
            button {{
                border-radius: 5px;
                border: none;
                font-size: 16px;
                font-weight: bold;
                margin: 8px;
            }}
            .acceptBtn {{
                color: white;
                background-color: green;
                padding: 5px;
            }}
            .rejectBtn {{
                color: white;
                background-color: red;
                padding: 5px;
            }}
        ",
        com.email, com.rating, com.comment, com.comid, com.comid,
    );
    message.html_part = Some(html1.to_string());

    info!("Com Mail Message: {:?}", message);

    message.push_recipient(Recipient::new("tflowerpower1313@gmail.com"));

    // Finally send the message using the `Client`
    let _response = client.send(message).await;

    Ok(())
}

// #[tokio::main]
pub async fn send_esti_mail(
    esti: types::Estimate,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let api_key = env::var("COMSERV_API_KEY").unwrap();
    let sec_key = env::var("COMSERV_SEC_KEY").unwrap();
    let client = Client::new(SendAPIVersion::V3, api_key.as_str(), &sec_key.as_str());

    // Create your a `Message` instance with the minimum required values
    // let sender = "porthose.cjsmo.cjsmo@gmail.com";
    let mut message = Message::new(
        "porthose.cjsmo.cjsmo@gmail.com",
        "ATSBOT",
        Some("ATSBOT: New Estimate Request".to_string()),
        Some("ATSBOT: New Estimate Request".to_string()),
    );

    let html1 = format!(
        "
        <div>
            <p>{}</p>
            <p>{}</p>
            <p>{}</p>
            <p>{}</p>
            <p>{}</p>
            <p>{}</p>
        </div>
        <a href='https://atstest.xyz/completed/{}'>
        <button class='comBtn'>Completed</button></a>
        <style>
            p {{
                font-size: 12px;
                font-weight: bold;
            }}
            button {{
                border-radius: 5px;
                border: none;
                font-size: 16px;
                font-weight: bold;
                margin: 8px;
            }}
            .comBtn {{
                color: white;
                background-color: green;
                padding: 5px;
            }}
        </style>
        ",
        esti.name, esti.phone, esti.email, esti.address, esti.city, esti.comment, esti.estid,
    );

    message.html_part = Some(html1.to_string());

    info!("Esti Mail Message: {:?}", message);

    message.push_recipient(Recipient::new("tflowerpower1313@gmail.com"));

    // Finally send the message using the `Client`
    let _response = client.send(message).await;

    Ok(())
}
