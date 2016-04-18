extern crate lag_client;
use lag_client::Client;
use lag_client::frame::Message;
use lag_client::state::ClientState;
//use std::net::SocketAddr;
//use std::thread;

fn main() {
    let addr = "127.0.0.1:6969".parse().unwrap();

    println!("Connecting!");
    if let Ok(mut client) = Client::connect(&addr){
        let mut test_client = ClientState::new(1);
        test_client.position = (1,2,3);
        let god = Message::new_client_update_message(&test_client);
        client.send_message(&god);

        loop {
            let messages = client.pop_received_messages();
            if let Some(messages) = messages{
                for message in messages{
                    println!("Received message: {:?}", message);
                }
            }
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
