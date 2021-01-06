extern crate websocket;

use std::str;
use std::thread;
use websocket::sync::Server;
use websocket::message::{OwnedMessage};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    c: i32,
}


///
/// Gets the current unix timestamp of the server
/// Returns - i64 - The current unix timestamp
///
fn get_timestamp() -> i64{

    //Gets the current unix timestamp of the server
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => return n.as_secs() as i64,
        Err(_e) =>return 0
    }

}

///
/// Creates a JSON string containing the message count and the current timestamp
/// Param - c - i32 - The message count
/// Returns - OwnedMessage object - an OwnedMessage object holding a JSON string containing the message count and the current timestamp
///
fn get_event(c: i32) -> OwnedMessage {

    //create an event array for the time that message "c" is received by the server
    let event = json!({
        "c": c,
        "ts": get_timestamp()
    });

    // convert the json to a string
    let event_string = event.to_string();

    // return OwnedMessage Text object, with the event string as it's payload
    return OwnedMessage::Text(event_string);
}

///
/// Send a connected client an event JSON string
/// Param ws - websocket::sender::Writer The client connection the outgoing message is for
/// Param - c - i32 - The message count
/// Returns - websocket::sender::Writer - The client connection the outgoing message is for ( needs
///                                       to be returned back to the serve function )
///
fn notify(mut ws: websocket::sender::Writer<std::net::TcpStream>, c: i32 ) -> websocket::sender::Writer<std::net::TcpStream> {
    let message = get_event(c);

    //send the given connection the event timestamp for message "c"
    ws.send_message(&message).unwrap();
    return ws
}

///
/// Starts the websocket server listening on the given address and port
/// handles any incoming requests to the server
///
fn main() {
    let server = Server::bind("0.0.0.0:8080").unwrap();
    println!("Running");

    for request in server.filter_map(Result::ok) {

        // Called once per incoming connection
        // Handles events like when a new client connects
        // and when the server receives a message from the client.
        // Spawn a new thread for each connection.
        thread::spawn(|| {

            // get the client of the incoming connection
            let client = request.accept().unwrap();
            let ip = client.peer_addr().unwrap();
            let (mut receiver, mut sender) = client.split().unwrap();

            println!("Connection from {}", ip);

            // send newly connected client initial timestamp
            sender = notify(sender, 0);

            //continuously listen for incoming messages
            for message in receiver.incoming_messages() {
                let message = message.unwrap();


                match message {
                    // process any incoming Text messages
                    OwnedMessage::Text(message_text) => {

                        // decode incoming message into a struct
                        let request: Request = serde_json::from_str(&message_text).unwrap();

                        // notify client with event for message with count "c"
                        sender = notify(sender, request.c)
                    }
                    //ignore all other kinds of message ( Ping, Close, etc )
                    _ => {
                        continue;
                    }
                }
            }
        });
    }

}