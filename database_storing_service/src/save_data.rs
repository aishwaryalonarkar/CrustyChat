extern crate redis;

use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};
use redis::Client;
use serde_json::{json};

use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    name: String,
    phone: String,
    password: String,
    email: String,
    hash: String,
    username: String,
    chats: String
}


pub fn save_new_data(data : &UserData) -> RedisResult<String> {
    // Save to master instance
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut conn = client.get_connection()?;
    let serialized = serde_json::to_string(&data).unwrap();

    let key_set = ["user_data:",&data.username.to_string()];
    let key = key_set.concat();
    let master_resp: String = conn.set(key, serialized)?;
    

    // Save to backup instance
    let client = redis::Client::open("redis://:password@127.0.0.1:6381/")?;
    let mut conn = client.get_connection()?;
    let serialized = serde_json::to_string(&data).unwrap();

    let key_set = ["user_data:",&data.username.to_string()];
    let key = key_set.concat();
    let backup_resp: String = conn.set(key, serialized)?;
    

    let result = format!("save_new_data master_resp => {:?}, backup_resp => {:?}", master_resp,backup_resp);
    Ok(result)
}


#[allow(unused_variables)]
pub fn add_new_chat(user:String,username:String) -> RedisResult<String> {
    println!("add new data = {} , {} ",user,username);
    let key_set = ["user_data:",&username];
    let key = key_set.concat();
    
    let client = Client::open("redis://127.0.0.1:6379/")?;
    
    let mut conn = client.get_connection()?;
    let result: Option<String> = conn.get(key.clone())?;
    // conn.close();

    let mut username_exists = false;
    let mut username_data: Option<String> = None;

    match result.clone() {
        value => {
            if value!=None {
                username_exists = true;
                username_data = value;
            }
        },
        // _panic => {
        //     println!("panic -==========  ");
        // },
    }
    
    if username_exists==true {
        let username_key = &key;

        let key_set = ["user_data:",&user];
        let key = key_set.concat();
        let user_key = key.clone();
        let mut conn = client.get_connection()?;
        let result: Option<String> = conn.get(key)?;
        let mut user_data: Option<String> = None;
        match result.clone() {
            value => {
                user_data = value;
            },
            // _panic => {
            // },
        }   
        let username_data: utils::UserData = serde_json::from_str(&username_data.expect("REASON").to_string()).unwrap();
        let user_data: utils::UserData = serde_json::from_str(&user_data.expect("REASON").to_string()).unwrap();


        // update in user data
        let chat_data = user_data.chats;
        let mut chat_set = [chat_data.clone(),",".to_string(),username.clone()];
        if chat_data == "" {
            chat_set = ["".to_string(),"".to_string(),username.clone()];
        }
        let chat_data = chat_set.concat();

        let value: String = conn.get(user_key.clone()).unwrap();
        let mut object: serde_json::Value = serde_json::from_str(&value).unwrap();
        let object_obj = object.as_object_mut().unwrap();
        object_obj.insert("chats".to_string(), json!(chat_data));
        let updated_value = json!(object);
    
        // Store the updated JSON object in Redis
        let master_resp :String = conn.set(user_key.clone(), updated_value.to_string()).unwrap();

        let client = Client::open("redis://:password@127.0.0.1:6381/")?;
        let mut conn = client.get_connection()?;
        // let result: Option<String> = conn.get(username_key)?;
        let backup_resp :String = conn.set(user_key, updated_value.to_string()).unwrap();

        let result = format!("save_new_data master_resp => {:?}, backup_resp => {:?}", master_resp,backup_resp);
        println!("add new chat user => {:?} ",result);

        // conn.close();


        // update in username data
        let client = Client::open("redis://127.0.0.1:6379/")?;
    
        let mut conn = client.get_connection()?;

        // println!("username data = {:?} \n user data = {:?} ", username_data,user_data);
        let chat_data = username_data.chats;
        println!("Previous chats = {:?} ",chat_data);
        let mut chat_set = [chat_data.clone(),",".to_string(),user.clone()];
        if chat_data == "" {
            chat_set = ["".to_string(),"".to_string(),user];
        }
        let chat_data = chat_set.concat();
        println!("New chats = {:?} ",chat_data);

        let value: String = conn.get(username_key.clone()).unwrap();
        let mut object: serde_json::Value = serde_json::from_str(&value).unwrap();
        let object_obj = object.as_object_mut().unwrap();
        object_obj.insert("chats".to_string(), json!(chat_data));
        let updated_value = json!(object);
    
        // Store the updated JSON object in Redis
        let master_resp :String = conn.set(username_key, updated_value.to_string()).unwrap();

        let client = Client::open("redis://:password@127.0.0.1:6381/")?;
        let mut conn = client.get_connection()?;
        let backup_resp :String = conn.set(username_key, updated_value.to_string()).unwrap();

        let result = format!("save_new_data master_resp => {:?}, backup_resp => {:?}", master_resp,backup_resp);

        println!("add new chat username => {:?} ",result);
        // conn.close();
    }
    let mut response = "Something went wrong";
    if username_exists {
        response = "Success";
    } 
    Ok(response.to_string())

}

