
extern crate sha2;
extern crate hex;

use sha2::{Sha256, Digest};
use redis::{RedisResult};
use redis::Commands;
use std::fs::{File};
use std::io::{Write};
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::{to_string_pretty};
use std::fs;
use chrono::Local;

use crate::utils;




lazy_static! {
    static ref FILE_MUTEX: Mutex<()> = Mutex::new(());
}

pub fn compute_hash(redis_url : &str) -> RedisResult<String> {

    let current_time = Local::now();
    println!("The current time is: {}", current_time);

    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;
    
    let mut hasher = Sha256::new();
    
    let mut cursor: i64 = 0;
    loop {
        let (new_cursor, keys): (i64, Vec<String>) = redis::cmd("SCAN")
            .arg(cursor)
            .arg("MATCH")
            .arg("*")
            .arg("COUNT")
            .arg(1000)
            .query(&mut con)?;
        
        for key in keys {
            let value: String = con.get(&key)?;
            hasher.update(value.as_bytes());
        }
        
        cursor = new_cursor;
        if cursor == 0 {
            break;
        }
    }
    
    let result = hasher.finalize();

    let hash_string = hex::encode(result);

    println!("Hash {:?}", hash_string);
    
    let current_time = Local::now();
    println!("The current time is: {}", current_time);
    Ok(hash_string)
}


pub fn get_saved_master_hash() ->  Result<String,  std::io::Error> {

    let _lock = FILE_MUTEX.lock().unwrap();

    let contents = fs::read_to_string("src/hash.json")?;

    drop(_lock);
    Ok(contents)
}


pub fn modify_hash(hash_data : utils::HashData) -> Result<(), Box<dyn std::error::Error>> {

    let _lock = FILE_MUTEX.lock().unwrap();

    let json_data = to_string_pretty(&hash_data)?;
    let mut file = File::create("src/hash.json")?;
    file.write_all(json_data.as_bytes())?;
    file.flush()?;
    drop(_lock);
    Ok(())
}



