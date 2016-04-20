extern crate lag_client;
extern crate term;
extern crate rand;
extern crate colored;

use lag_client::Client;
use lag_client::frame::Message;
use lag_client::state::{Position, ClientState, GameState};
use std::iter;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};
use colored::*;
use std::ops::Deref;

//use std::net::SocketAddr;
//use std::thread;

struct GameGrid{
    /// The dimensions of the game board, (width, height)
    dimensions: (u32, u32),

    /// Positions of all players
    state: GameState
}

impl GameGrid{
    fn new(dimensions: (u32, u32)) -> GameGrid{
        GameGrid{
            dimensions: dimensions,
            state: GameState::new()
        }
    }

    fn draw(&self, client_id: u32){
        let mut t = term::stdout().unwrap();
        for _ in 0..(self.dimensions.1 + 2){
            //t.carriage_return().unwrap();
            t.cursor_up().unwrap();
            t.delete_line().unwrap();
        }

        let mut clients = self.get_sorted_clients();
        clients.reverse();

        //println!("{:?}", clients);

        // draw upper border
        println!("╔{}╗", iter::repeat("═").take(self.dimensions.0 as usize).collect::<String>());

        let mut top_client : Option<&ClientState> = None;
        for y in 0..self.dimensions.1{
            let row = (0..self.dimensions.0 as usize).map(|x|{
                if top_client.is_none() { top_client = clients.pop(); }

                if let Some(client) = top_client{
                    if client.position.1 == y as i32{
                        match client.position.0.cmp(&(x as i32)){
                            Ordering::Equal => {
                                top_client = None;
                                if client.id == client_id{
                                    return "+";
                                } else { return "*"; }
                            },
                            Ordering::Less => {
                                top_client = None;
                            },
                            _ => {}
                        }
                    }
                    else if client.position.1 < y as i32{
                        top_client = None;
                    }
                }

                return " ";
            }).collect::<String>();

            println!("║{}║", row);
        }
        println!("╚{}╝", iter::repeat("═").take(self.dimensions.0 as usize).collect::<String>());
    }

    fn get_sorted_clients(&self) -> Vec<&ClientState>{
        let mut clients = self.state.clients.values().collect::<Vec<&ClientState>>();
        clients.sort_by(|a,b|
                match a.position.1.cmp(&b.position.1){
                    Ordering::Equal => {
                        return a.position.0.cmp(&b.position.0);
                    },
                    o => { return o; }
                }
            );
        return clients;
    }
}

fn get_next_position(client: &Client, game: &GameGrid) -> Option<Position>{
    if let Some(position) = client.get_position(){
        let mut options = Vec::with_capacity(4);

        // If this client can move to the left
        if position.0 > 0 { options.push(Position(-1,0,0)); }
        // If this client can move to the right
        if position.0 < (game.dimensions.0 as i32 - 1) { options.push(Position(1,0,0)); }
        // if this client can move upward
        if position.1 > 0 { options.push(Position(0,-1,0)); }
        // if this client can move downward
        if position.1 < (game.dimensions.1 as i32 - 1) { options.push(Position(0,1,0)); }
        // the client can always stay in current position
        options.push(Position(0,0,0));


        let mut rng = rand::thread_rng();
        let shift = options[rng.gen_range::<usize>(0,options.len() - 1)];
        return Some(position + shift);
    }
    return None;
}

fn main() {
    let addr = "127.0.0.1:6969".parse().unwrap();

    println!("Connecting!");
    if let Ok(mut client) = Client::connect(&addr){
        let mut game_board = GameGrid::new((10,10));

        let mut test_counter = 0;

        println!("Waiting to authenticate with server...");
        loop{
            if client.is_authenticated(){
                println!("Received server authorization!");
                break;
            }
        }

        let client_id = client.get_id().unwrap();

        game_board.draw(client_id);

        loop {
            let messages = client.pop_received_messages();

            if let Some(messages) = messages{
                for message in messages{
                    match message{
                        Message::GameStateUpdate(game_state) => {
                            game_board.state.update_from_vec(&game_state);
                        },
                        m => {
                            println!("Received message: {:?}", m);
                        }
                    }
                }

                game_board.draw(client_id);
            }


            if test_counter % 1000000 == 0{
                // let mut position = client.get_position();
                // position.0 += 1;
                // position.1 += 2;
                // position.2 += 3;
                if let Some(position) = get_next_position(&client, &game_board){

                    //println!("Updated position!");
                    client.set_position(position);
                }
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

        client.disconnect();
    }
}
