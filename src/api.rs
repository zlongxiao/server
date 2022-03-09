extern crate chrono;
use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use std::time::SystemTime;


use warp::{reply, Filter, Rejection, Reply};

use crate::MinerList;
type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Deserialize, Debug)]
pub struct AddRequest {
    pub name: String,
    pub ip: String,
    pub state: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteRequest{
    pub id:u32
}


#[derive(Deserialize, Debug)]
pub struct UpdateRequest{
    id:u32,
    state:u32
}




pub async fn api() {
    let add_items = warp::post()
        .and(warp::path("api"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(add_list);

    let get_items = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path::end())
        .and_then(find_list);
    
    let get_counts = warp::get()
        .and(warp::path("api"))
        .and(warp::path("counts"))
        .and(warp::path::end())
        .and_then(get_count);

    let delete_items = warp::post()
        .and(warp::path("api"))
        .and(warp::path("del"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(delete_miner);

        let update_items = warp::post()
        .and(warp::path("api"))
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(update_miner);

    let routes = add_items.or(get_items)
    .or(get_counts)
    .or(update_items)
    .or(delete_items);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn find_list() -> WebResult<impl Reply> {
    let list = MinerList::find().unwrap();
    Ok(reply::json(&list))
}

async fn add_list(body: AddRequest) -> WebResult<impl Reply> {
    let mut miner = MinerList::new();
    miner.name = body.name;
    miner.ip = body.ip;
    miner.state = body.state.parse::<u32>().unwrap();
    let system_time = SystemTime::now();
    let datetime: DateTime<Local> = system_time.into();
    miner.update_time = datetime.format("%Y-%m-%d %T").to_string();
    let result = MinerList::insert(miner).unwrap();

    if result {
        return Ok(format!("add success"));
    }
    Ok(format!("add fail"))
}

async fn get_count()-> WebResult<impl Reply> {
    let count = MinerList::count().unwrap();
    Ok(count.to_string())
}

async fn delete_miner(body:DeleteRequest)-> WebResult<impl Reply>{
    MinerList::delete(body.id).unwrap();
    Ok(format!("delete success"))
}

async fn update_miner(body:UpdateRequest)->WebResult<impl Reply>{
    MinerList::update(body.id,body.state).unwrap();
    Ok(format!("update success"))
}
