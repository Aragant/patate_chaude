use serde_json::{Value};
use serde::{Serialize, Deserialize};
use crate::hashcash;
// use crate::maze;

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageResponse {
    Welcome(Value),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    RoundSummary(Value),
    EndOfGame(Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Error(Value),
    Err(Value)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(hashcash::MD5HashCashInput),
    // MonstrousMaze(maze::MonstrousMazeInput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f32,
}
