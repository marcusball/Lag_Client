extern crate lag_client;
extern crate term;

use lag_client::Client;
use lag_client::frame::Message;
use lag_client::state::{Position, ClientState};
//use std::net::SocketAddr;
//use std::thread;

fn main() {
    let addr = "127.0.0.1:6969".parse().unwrap();

    println!("Connecting!");
    if let Ok(mut client) = Client::connect(&addr){
        // let mut test_client = ClientState::new(1);
        // test_client.position = Position(1,2,3);
        // let god = Message::new_client_update_message(&test_client);
        // client.send_message(&god);

        let mut test_counter = 0;

        loop {
            let messages = client.pop_received_messages();
            let mut t = term::stdout().unwrap();
            if let Some(messages) = messages{
                for message in messages{
                    for _ in 1..3{
                        //t.carriage_return().unwrap();
                        t.cursor_up().unwrap();
                        t.delete_line().unwrap();
                    }
                    println!("Received message: {:?}", message);
                }
            }


            if test_counter % 1000000 == 0{
                let mut position = client.get_position();
                position.0 += 1;
                position.1 += 2;
                position.2 += 3;

                //println!("Updated position!");
                client.set_position(position);
            }

            test_counter += 1;

            // let message = client.read();
            // if let Ok(message) = message{
            //     println!("Received a message!");
            // }
            if !client.is_connected(){
                println!("Client disconnected!");
                break;
            }
        }
    }
}
