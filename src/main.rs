use lettre::{ message::header::ContentType, Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

fn main() {
    // println!("Hello, world!");
    let email = Message::builder()
        .to("Gmail <adityamali2003@gmail.com>".parse().unwrap())
        .from("Aditya <adityamali2003@icloud.com>".parse().unwrap())
        .subject("Rust Email Test")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("This is a test message sent using Rust and Lettre"))
        .unwrap();

    let creds = Credentials::new("emailId Here".to_string(), "Password Here".to_string());
    
    let client = SmtpTransport::starttls_relay("smtp.mail.me.com") // currently using the icloud smtp server config
        .unwrap()
        .port(587)
        .credentials(creds)
        .build();

    match client.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = send_mail();
//         assert_eq!(result);
//     }
// }
