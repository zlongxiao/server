use serde::{Deserialize, Serialize};


#[derive(Debug,Clone,Default,Deserialize, Serialize)]
pub struct  MinerList{
    pub id:Option<u64>,
    pub name:String,
    pub ip:String,
    pub state:u32,
    pub update_time:String,
    pub create_time:Option<String>,
}