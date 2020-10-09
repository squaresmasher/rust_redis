extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};

mod cache {
    pub trait AbstractCache {
        fn new() -> Self;
        fn connect(&self, addr: &str);
        fn set(&self, key: &str, value: &str);
        fn get(&self, key: &str) -> redis::RedisResult<()>;
        fn del(&self, key: &str);
    }

    pub struct Cache { connection: Option<redis::Connection> }

    impl AbstractCache for Cache {
        fn new() -> Cache {
            Cache {connection: None}
        }

        fn connect(&self, addr: &str) {
            let client = redis::Client::open(addr).unwrap();
            self.connection = Some(client.get_connection().unwrap());
        }

        fn set(&self, key: &str, value: &str){
            use redis::Commands;
            let mut conn = self.connection.unwrap();
            let _: () = conn.set(key, value).unwrap();

        }

        fn get(&self, key: &str) -> redis::RedisResult<()> {
            use redis::Commands;
            let mut conn = self.connection.as_ref().unwrap();
            let _: redis::RedisResult<()> = *conn.get(key);
            Ok(())
        }

        fn del(&self, key: &str) {

        }
    }
}


fn test_redis() -> RedisResult<()> {
    let client = Client::open("redis://0.tcp.ngrok.io:17570").unwrap();
    let mut con = client.get_connection().unwrap();
    let _ : () = con.set("test_rust", 69420).unwrap();
    let test_rust: i32 = con.get("test_rust").unwrap();
    println!("test_rust: {}", test_rust);
    Ok(())
}

fn main() {
    use cache::{Cache, AbstractCache}; 
    let mut _cache: Cache = AbstractCache::new();
    _cache.connect("redis://0.tcp.ngrok.io:17570");
    _cache.get("foo");
}