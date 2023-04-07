
use redis::{Client, RedisResult};

pub fn set_replication() ->  RedisResult<String> {
    let client = Client::open("redis://127.0.0.1:6380")?;
    let mut con = client.get_connection()?;
    
    let set_slave:String = redis::cmd("slaveof")
        .arg("127.0.0.1")
        .arg("6379")
        .query(&mut con).unwrap();

    let read_only:String = redis::cmd("CONFIG")
        .arg("SET")
        .arg("slave-read-only")
        .arg("yes")
        .query(&mut con)?;
    
    let result = format!("output set_slave => {:?}, slave_read_only => {:?}", set_slave,read_only);
    Ok(result)
}

#[allow(dead_code)]
pub fn stop_replication(redis_url : &str) ->  RedisResult<String> {
    let client = Client::open(redis_url)?;
    let mut con = client.get_connection()?;
    
    let res:String = redis::cmd("slaveof")
        .arg("no")
        .arg("one")
        .query(&mut con).unwrap();

    Ok(res)
}

#[allow(dead_code)]
pub fn set_read_only() -> RedisResult<String> {
    let client = Client::open("redis://127.0.0.1:6381")?;
    let mut con = client.get_connection()?;
    let res:String = redis::cmd("CONFIG")
        .arg("SET")
        .arg("slave-read-only")
        .arg("yes")
        .query(&mut con)?;
    Ok(res)
}

#[allow(dead_code)]
pub fn failover() -> RedisResult<String> {
    let client = Client::open("redis://127.0.0.1:6379")?;
    let mut con = client.get_connection()?;
    
    let res:String = redis::cmd("FAILOVER").query(&mut con).unwrap();
    Ok(res)
}


pub fn set_password() -> RedisResult<String> {
    let client = Client::open("redis://127.0.0.1:6381")?;
    let mut con = client.get_connection()?;
    
    let res:String = redis::cmd("CONFIG")
        .arg("SET")
        .arg("requirepass")
        .arg("password")
        .query(&mut con).unwrap();
    Ok(res)
}

use redis::{Commands};
pub fn start_recovery() -> RedisResult<()> {
    // Connect to the source Redis instance
    let client_src = redis::Client::open("redis://:password@127.0.0.1:6381/")?;
    let mut conn_src = client_src.get_connection()?;

    // get keys
    let keys: Vec<String> = conn_src.keys("*")?;
    println!("keys=> {:?}",keys);

    for key in keys {
        let _: () = redis::cmd("MIGRATE")
        .arg("127.0.0.1")
        .arg("6379")
        .arg("")
        .arg(0)
        .arg(5000)
        .arg("COPY")
        .arg("REPLACE")
        .arg("KEYS").arg(key)
        .arg("AUTH2").arg("").arg("password")
        .arg("AUTH").arg("").arg("")
        .query(&mut conn_src)?;
    }


    Ok(())
}

