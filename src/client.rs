mod form_data;
mod hashcash;
mod maze;

use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{env};

use serde_json::{Value};
use crate::form_data::PublicPlayer;


fn main() {

    let mut user_name = get_username();
    let mut leader: String = "".to_string();
    let stream = TcpStream::connect("localhost:7878");

// loop
    match stream {
        Ok(mut stream) => {
            //Connect to server
            send_message(&mut stream, "\"Hello\"".to_string());

            //Exec
            loop {
                let size = len_stream(&mut stream);
                if size !=0 {
                    let buff: form_data::MessageResponse = return_value(&mut stream, size);
                    match &buff {
                        form_data::MessageResponse::Welcome(res) => welcome(&mut stream, res, user_name.clone()),
                        form_data::MessageResponse::SubscribeResult(res) => subscribe_result(res),
                        form_data::MessageResponse::Challenge(res) => {
                            let res = select_challenge(res, &leader);
                            send_message(&mut stream, res.to_string());
                        },
                        form_data::MessageResponse::PublicLeaderBoard(res) => {
                            leader = leader_board(res, user_name.clone());
                        },
                        form_data::MessageResponse::EndOfGame(res) => {
                            println!("Close connection...");
                            println!("{}", res);
                            break;
                        },
                        form_data::MessageResponse::RoundSummary(value) => round_summary(value)
                    }
                }

                // let ten_millis = time::Duration::from_millis(100);
                // thread::sleep(ten_millis);
            }

            //close connection
            drop(stream);
        }

        Err(err) => panic!("Cannot connect: {err}")
    }
}


//-------- Message Traitment --------
fn welcome(stream: &mut TcpStream, res: &Value, user_name: String) {
    println!("here welcome, version: {}", res["version"]);
    send_message(stream, "{\"Subscribe\":{\"name\":\"".to_owned() + &user_name +"\"}}");
}

fn subscribe_result(res: &form_data::SubscribeResult) {
    match &res {
        form_data::SubscribeResult::Ok => println!("Subscribed !"),
        form_data::SubscribeResult::Error(val) => println!("Message :  {}", val),
        form_data::SubscribeResult::Err(val) => println!("Message : {}", val)
    }
}

fn select_challenge(challenge: &form_data::Challenge, user_to_target: &str) -> String {
    let mut result: String = "".to_string();
    match &challenge {
        form_data::Challenge::MD5HashCash(res) => {
            let temp = hashcash::hashcash(&res);
            // println!("{:?}",temp);
            result = "{\"ChallengeResult\":{\"answer\":{\"MD5HashCash\":{\"seed\":".to_string() + &temp.seed.to_string() + ",\"hashcode\":\"" + &temp.hashcode + "\"}},\"next_target\":\""+ user_to_target + "\"}}";
            println!("{}", result);

        },
        // form_data::Challenge::MonstrousMaze(res) => {
        //     // let temp = maze::start(&res);
        //     result = "{\"ChallengeResult\":{\"answer\":{\"MonstrousMaze\":{\"path\":".to_string() + "&temp.seed.to_string()" + "\"}},\"next_target\":\""+ user_to_target + "\"}}";
        //     println!("{}", result);
        // }
    };
    result
}

fn leader_board(res: &Vec<PublicPlayer>, me: String) -> String {
    println!("Leaderboard : ");
    let mut userToTarget = "";
    for player in res {
        println!("Player : {}", player.name);
        if me != player.name { userToTarget = &player.name }
    }
    return userToTarget.to_string();
}

fn round_summary(res : &Value) {
    println!("-----------------");
    println!("Round summary : ");
    println!("{}", res);
    println!("-----------------");
}

//-------- End Message Traitment --------


//-------- TCP Traitment --------

fn return_value(stream: &mut TcpStream, size: u32) -> form_data::MessageResponse {
    let mut data = Vec::new();
    data.resize(size.try_into().unwrap(), 0);
    match stream.read(&mut data) {
        Ok(_) => {
            // attempt!{{
                let v: form_data::MessageResponse = serde_json::from_str(from_utf8(&data).unwrap()).unwrap();
                return v;
            //     } catch(e) {
            //     println!("Erreur");
            // }}
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
    return serde_json::from_str(from_utf8((&"").as_ref()).unwrap()).unwrap();
}

fn len_stream(stream: &mut TcpStream) -> u32 {
    let mut number = [0 as u8; 4];
    match stream.read_exact(&mut number) {
        Ok(_) => {
            let s = u32::from_be_bytes(number);
            return s;
        },
        Err(e) => {
            println!("Failed to get length: {}", e);
            drop(stream);
        }
    }
    return 0;
}

fn send_message(stream: &mut TcpStream, message: String) {
    println!("{}", message.len() as u32);
    let n : u32 = message.len() as u32;
    stream.write_all(&n.to_be_bytes()).unwrap(); //.unwrap()
    stream.write_all(&message.as_bytes()).unwrap(); //.unwrap()
}
//------- End TCP Traitment --------

fn get_username() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", args[1].to_string());
        return args[1].to_string();
    }
    return "fred".to_string();
}