extern crate redis;
use redis::Commands;
use redis::{Client, RedisResult};
use crate::utils;
use std::error::Error;

pub fn get_user_data_by_key(key: &str) -> RedisResult<utils::UserData> {
    println!("get data by key");
    
    let key_set = ["user_data:",key];
    let key = key_set.concat();
    let client = Client::open("redis://127.0.0.1:6379/")?;
    let mut conn = client.get_connection()?;
    let result: Option<String> = conn.get(key)?;
    let user_data: utils::UserData = serde_json::from_str(&result.expect("REASON").to_string()).unwrap();

    Ok(user_data)
}

#[allow(unused_variables)]
pub fn get_messages(queue:&str) -> RedisResult<String> {
    let key_set = ["chat_data:".to_string(),queue.to_string()];
    let key = key_set.concat();
    let client = Client::open("redis://127.0.0.1:6379/")?;
    let mut conn = client.get_connection()?;
    let result: Option<String> = conn.get(key.clone())?;

    let mut res = "".to_string();
    let mut queue_exists = false;
    match result.clone() {
        value => {
            if value!=None && result.clone().expect("REASON").to_string()!="" {
                queue_exists = true;
                res = result.expect("REASON").to_string();
            }
        },
        // _panic => {
        //     println!("is panic");
        // },
    }   

    let amq_messages_resp = activemq_message_receiver(queue.to_string());

    let amq_messages = amq_messages_resp.unwrap();
    let mut amq_message_exists = false;
    if amq_messages!="" {
        amq_message_exists = true;
    }

    let mut chat_set = ["".to_string(),"".to_string(),"".to_string()];
    if amq_message_exists && queue_exists {
        chat_set = [res.clone(),",".to_string(),amq_messages.clone()];
    } else if amq_message_exists && !queue_exists {
        chat_set = [amq_messages.clone(),"".to_string(),"".to_string()];
    } else if !amq_message_exists && queue_exists {
        chat_set = [res.clone(),"".to_string(),"".to_string()];
    } 

    let chat_data = chat_set.concat();
    let master_resp: String = conn.set(key.clone(), chat_data.clone())?;
    
    let client = Client::open("redis://:password@127.0.0.1:6381/")?;
    let mut conn = client.get_connection()?;
    let backup_resp: String = conn.set(key.clone(), chat_data.clone())?;


    let result = format!("save_chat master_resp => {:?}, backup_resp => {:?}", master_resp,backup_resp);
    Ok(chat_data)
}


fn activemq_message_receiver(queue : String) -> Result<String, Box<dyn Error>> {
    
    let chars = ["http://localhost:5000/consume_message?queue=",&queue];
    let url = chars.concat();
    let resp = reqwest::blocking::get(url.clone())?.text()?;
    println!("{:#?}, url-={} ", resp,url.clone());

    let mut res = "".to_string();
    if resp!="" {
        res = resp.to_string();
    }

    Ok(res)
}