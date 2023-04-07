#![allow(warnings)]
mod save_data;
mod get_data;
mod utils;
mod replicate;
mod hashing;

use actix_cors::Cors;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::{get, App, HttpRequest, HttpServer};
use std::time;
use std::error::Error;
use serde_json::json;
use std::thread;
use std::sync::mpsc;

#[get("/index.html")]
async fn index(_req: HttpRequest) -> &'static str {
    "<p>Hello World!</p>"
}


#[get("/sign_up")]
async fn sign_up(req: HttpRequest) -> &'static str {

    println!("Sign up process started ");
    check_hash();

    let mut query_string = req.query_string().to_owned();
    query_string = query_string.replace("%22", "\"");

    let user: save_data::UserData = serde_json::from_str(&query_string.to_string()).unwrap();

    let resp = save_data::save_new_data(&user);
    println!("{:?} resp = ",resp);

    save_new_hash();
    "Sign up successful"
}

#[get("/sign_in")]
async fn sign_in(req: HttpRequest) -> impl Responder  {
    check_hash();
    let mut query_string = req.query_string().to_owned();
    query_string = query_string.replace("%22", "\"");

    println!("input str = {}", query_string);

    let user: utils::UserLoginData = serde_json::from_str(&query_string.to_string()).unwrap();
    let username = user.username;
    let password = user.password;
    let resp = get_data::get_user_data_by_key(&username);
    let user_data = resp.map(|data| data.clone()).and_then(|data| Ok(data));

    let user_username = user_data.as_ref().map(|data| data.username.clone()).unwrap_or_default();
    let user_password = user_data.as_ref().map(|data| data.password.clone()).unwrap_or_default();
    let user_chats = user_data.as_ref().map(|data| data.chats.clone()).unwrap_or_default();

    let mut status_code = 200;
    let mut message = "Something went wrong".to_string();
    if user_username == username && user_password == password {
        let message_set = format!("Success,{}", user_chats.clone().to_string());
        message = message_set;
    } else {
        message = "Invalid Username or Password".to_string();
        status_code = 300;
    }

    let mut response = utils::Response::new();
    response.set_status(status_code);
    response.set_message(&message);
    HttpResponse::Ok().json(response)
}

#[get("/add_user_chat")]
async fn add_user_chat(req: HttpRequest) -> impl Responder  {
    println!("add new user chat");
    check_hash();

    let mut query_string = req.query_string().to_owned();
    query_string = query_string.replace("%22", "\"");

    let user: utils::UsernameAdd = serde_json::from_str(&query_string.to_string()).unwrap();
    let username = user.username;
    let user = user.user;

    let resp = save_data::add_new_chat(user,username);
    let response_string = resp.as_ref().map(|data| data.clone()).unwrap_or_default();

    let status_code = 200;

    let mut response = utils::Response::new();
    response.set_status(status_code);
    response.set_message(&response_string);

    save_new_hash();

    HttpResponse::Ok().json(response)
}



#[get("/get_user_chat_list")]
async fn get_user_chat_list(req: HttpRequest) -> impl Responder  {

    println!("get user chat list");
    check_hash();

    let mut query_string = req.query_string().to_owned();
    query_string = query_string.replace("%22", "\"");

    let user:utils::UserToken = serde_json::from_str(&query_string.to_string()).unwrap();
    let username = user.username;
    let is_active_user = user.is_active_user;
    let is_valid_user = user.is_valid_user;

    let resp = get_data::get_user_data_by_key(&username);
    let user_data = resp.map(|data| data.clone()).and_then(|data| Ok(data));

    let _user_username = user_data.as_ref().map(|data| data.username.clone()).unwrap_or_default();
    let user_chats = user_data.as_ref().map(|data| data.chats.clone()).unwrap_or_default();

    let mut status_code = 200;
    let mut message = "Something went wrong".to_string();
    if is_active_user && is_valid_user {
        let message_set = format!("Success,{}", user_chats.clone().to_string());
        message = message_set;
    } else {
        message = "Invalid Username or Password".to_string();
        status_code = 300;
    }

    let mut response = utils::Response::new();
    response.set_status(status_code);
    response.set_message(&message);
    HttpResponse::Ok().json(response)
}


#[get("/get_messages")]
async fn get_messages(req: HttpRequest) -> impl Responder {

    println!("get messages");
    check_hash();

    let query_string = req.query_string().to_owned().replace("%22", "\"");
    let user: utils::UserToken = serde_json::from_str(&query_string).unwrap();
    let username = user.username;
    let is_active_user = user.is_active_user;
    let is_valid_user = user.is_valid_user;

    let (tx, rx) = mpsc::channel();

    // Spawn a thread to get user data
    thread::spawn(move || {

        let mut status_code = 200;
        let mut message = "Something went wrong".to_string();
        if is_active_user && is_valid_user {
            let resp = get_data::get_messages(&username);
            let response_string = resp.as_ref().map(|data| data.clone()).unwrap_or_default();
        
            message = response_string;
        } else {
            status_code = 300;
        }

        let mut response = utils::Response::new();
        response.set_status(status_code);
        response.set_message(&message);
        tx.send(response).unwrap();
    });

    save_new_hash();
    let response = rx.recv().unwrap();
    HttpResponse::Ok().json(response)
}


#[get("/set_replication")]
async fn set_replication() ->  impl Responder {

    let res = replicate::set_replication();
    let res2 = replicate::set_password();
    // let res3 = replicate::failover();
    // let res3 = replicate::stop_replication("redis://127.0.0.1:6381");

    // println!("res of rep = {:?}, res2 = {:?}, res3 = {:?} ",res, res2, res3);
    println!("res of rep = {:?}, {:?} ",res,res2);
    // "Replication set up successfully"
    let mut response = utils::Response::new();
    response.set_status(200);
    response.set_message("Replication set up successfully");
    HttpResponse::Ok().json(response)

}

#[get("/set_read_only")]
async fn set_read_only() ->  impl Responder {

    let res = replicate::set_read_only();
    println!("res of rep = {:?} ",res);
    let mut response = utils::Response::new();
    response.set_status(200);
    response.set_message("Read only set up successfully");
    HttpResponse::Ok().json(response)
}


fn check_hash() {

    println!("Checking hash ...");
    //  get db hash
    let res = hashing::compute_hash("redis://127.0.0.1:6379/");
    let db_hash = res.unwrap();

    // get saved hash 
    let res = hashing::get_saved_master_hash().unwrap();
    let json_data: utils::HashData = serde_json::from_str(&res).unwrap();
    let master_hash = json_data.master_hash;

    // compare the hashes
    if db_hash != master_hash {
        println!("Started recovery ...");
        // mismatched hash. do recovery
        let recovery = replicate::start_recovery();
        println!("recovery status = {:?} ",recovery);
        save_new_hash();
    }

}

fn save_new_hash() {

    println!("Computing and saving new hash ...");
    // get db hash
    let res = hashing::compute_hash("redis://127.0.0.1:6379/");
    let db_hash = res.unwrap();

    // get saved hash 
    let res = hashing::get_saved_master_hash().unwrap();
    let mut json_data: utils::HashData = serde_json::from_str(&res).unwrap();
    json_data.master_hash = db_hash;

    // save this new hash
    let save_hash = hashing::modify_hash(json_data);
    println!("New Hash write => {:?} ", save_hash);

}

#[get("/compute_hash")]
async fn compute_hash() -> impl Responder {
    
    let res = hashing::compute_hash("redis://127.0.0.1:6379/");
    let db_hash = res.unwrap();

    println!("start recovery");
    let res = replicate::start_recovery();
    println!("recovery= {:?} ",res);

    let mut response = utils::Response::new();
    response.set_status(200);
    response.set_message("Hash Computed");
    HttpResponse::Ok().json(response)
}

fn activemq_message_sender(message_details : String) -> Result<String, Box<dyn Error>> {
    
    let chars = ["http://localhost:5000/send_message?",&message_details];
    let url = chars.concat();
    let resp = reqwest::blocking::get(url.clone())?.text()?;
    println!("{:#?}, url-={} ", resp,url.clone());
    Ok(resp)
}

#[get("/send")]
async fn send(req: HttpRequest) -> impl Responder {
    println!("params {:?} ", req.query_string());
    let query_string = req.query_string().to_owned();

    let (tx, rx) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        println!("Sending message to activeMQ..");

        let duration = time::Duration::from_millis(100);

        
        let vec_path: Vec<&str> = query_string.split(",").collect();
        let queue_name = vec_path[0];
        let message = vec_path[1];
        let author : &str = vec_path[2];
        let date_time : &str = vec_path[3];


        
        let chars = ["queue=",queue_name, "&message=", message, "&author=", author, "&date_time=", date_time];
        let message_details = chars.concat();
        println!("{:?} url=> ",message_details);

        match activemq_message_sender(message_details.to_owned()) {
            Ok(resp) => {
                println!("Response: {}", resp);
                tx.send(resp).unwrap(); // send response back to parent thread
            },
            Err(e) => println!("Error: {:?}", e),
        }
        thread::sleep(duration);
    });
    let message_response = rx.recv().unwrap(); // wait for response from child thread
    let response_body = json!({
        "response": message_response,
        "status" : 200
    });

    handle.join().unwrap();
    HttpResponse::Ok().json(response_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let replication = set_replication;

    println!("Listening on 127.0.0.1:8081");
    HttpServer::new(|| {
        let cors = Cors::default()
              .allow_any_header()
              .allow_any_method()
              .allow_any_origin()
              .max_age(3600);

        App::new()
            .wrap(cors)
            .service(index)
            .service(sign_up)
            .service(sign_in)
            .service(add_user_chat)
            .service(get_user_chat_list)
            .service(get_messages) 
            .service(set_replication)
            .service(set_read_only)
            .service(compute_hash)
            .service(send)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await;
    Ok(())
}
