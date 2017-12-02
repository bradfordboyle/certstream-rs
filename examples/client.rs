extern crate certstream;
extern crate serde_json;

use serde_json::Value;

fn main() {
    let rx = certstream::cert_stream_event_stream(false);

    for received in rx {
        match received["message_type"] {
            Value::String(ref s) => {
                println!("Message type -> {:?}", s);
            }
            _ => (),
        }
    }
}
