extern crate rusqlite;

use rusqlite::{Connection, Result};
use crate::MinerList;

struct Count{
    count:u32
}

pub fn connect() -> Result<()> {
    let conn = Connection::open("proxy.db")?;

    conn.execute(
        "create table if not exists clients (
             id integer primary key AUTOINCREMENT,
             name varchar (50) unique,ip varchar (50),state integer,update_time datetime default (datetime('now', 'localtime')),create_time datetime default (datetime('now', 'localtime')) ,power float 
         )",
        [],
    )?;

    Ok(())
}

impl MinerList {
    pub fn new() -> Self{
        MinerList { name: "name".to_string(), ip: "ip".to_string(), state:0, update_time: "2022-02-02".to_string(), ..Default::default()  }
    }

    pub fn insert(miner: MinerList) -> Result<bool,rusqlite::Error> {
        let conn = Connection::open("proxy.db")?;

       conn.execute(
            "INSERT INTO clients (name,ip,state,update_time) values (?,?,?,?)",
            [
                miner.name,
                miner.ip,
                miner.state.to_string(),
                miner.update_time,
            ],
        )?;
        
        Ok(true)
    }

    pub fn find() -> Result<Vec<MinerList>> {
        let conn = Connection::open("proxy.db")?;
        let mut stmt = conn.prepare("SELECT * from clients;")?;

        let miners = stmt.query_map([], |row| {
            Ok(MinerList {
                id: row.get(0)?,
                name: row.get(1)?,
                ip: row.get(2)?,
                state: row.get(3)?,
                update_time: row.get(4)?,
                create_time: row.get(5)?,
            })
        })?;
        let mut result=vec![];
        for miner in miners {
            println!("Found miner {:?}", miner);
            result.push(miner?);
        }

        Ok(result)
    }

    pub fn delete(id:u32)->Result<bool,rusqlite::Error> {
        let conn = Connection::open("proxy.db")?;
        conn.execute(
            "delete from clients where id=?;",
            [
                id
            ],
        )?;
        Ok(true)
    }

    pub fn count()->Result<u32,rusqlite::Error> {
        let conn = Connection::open("proxy.db")?;
        let mut stmt = conn.prepare("select count(1) as count from clients;")?;
        let counts = stmt.query_map([], |row| {
            Ok(Count {
                count:row.get(0)?,
            })
        })?;
        let mut result=0;
        for count in counts {
           result = count?.count;
        }
        Ok(result)
    }

    pub fn update(id:u32,state:u32)->Result<bool,rusqlite::Error> {
        let conn = Connection::open("proxy.db")?;
        conn.execute(
            "update clients set state=?,update_time=now() where id=?",
            [
                state,
                id
            ],
        )?;
        Ok(true)
    }
}
