use redis::RedisError;

extern crate redis;
use redis::Commands;
use std::env;

fn connect_redis() -> redis::Connection{
    // Connect to redis
    let client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
    client.get_connection().unwrap()
}

pub fn whitelist_token(token: &str, user_id: &str) -> Result<(), RedisError> {
    let mut con = connect_redis();

    // Insert value
    let test : Result<(), RedisError> = con.set(token, user_id);
    test
}

pub fn get_whitelist_token(token: &str) -> Result<String, RedisError> {
    let mut con = connect_redis();
    
    // Get value
    let test:Result<String, redis::RedisError> = con.get(token);
    test
}

pub fn unwhitelist_token(token: &str) -> Result<(), RedisError> {
    let mut con = connect_redis();
    
    // Get value
    let test:Result<(), redis::RedisError> = con.del(token);
    test
}