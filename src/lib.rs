extern crate serde_json;
extern crate websocket;

use serde_json::Value;
use websocket::{ClientBuilder, OwnedMessage};

use std::sync::mpsc::{self, Receiver};
use std::thread;

const CONNECTION: &'static str = "wss://certstream.calidog.io";

pub fn cert_stream_event_stream(skip: bool) -> Receiver<Value> {
    println!("Connecting to {}", CONNECTION);

    let mut client = ClientBuilder::new(CONNECTION).unwrap().connect_secure(None).unwrap();

    println!("Successfully connected");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || for message in client.incoming_messages() {
                      if let OwnedMessage::Text(s) = message.unwrap() {
                          let v: Value = serde_json::from_str(&s).unwrap();

                          let message_type = &v["message_type"].clone();
                          if skip && message_type.as_str() == Some("heartbeat") {
                              continue;
                          }
                          tx.send(v).unwrap();
                      } else {
                          println!("Received other type");
                      }
                  });

    rx
}
