extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};

mod cache {
    pub trait AbstractCache {
        fn new(addr: &str) -> Self;
        // fn connect(&self, addr: &str) -> Cache;
        fn set(&self, key: &str, value: &str);
        fn get(&self, key: &str) -> redis::RedisResult<()>;
        fn del(&self, key: &str);
    }

    //pub struct Cache { connection: Option<redis::Connection> }
    pub struct Cache { connection: redis::Connection }

    impl AbstractCache for Cache {
        fn new(addr: &str) -> Cache {
            let client = redis::Client::open(addr).unwrap();
            let connection = client.get_connection().unwrap();
            Cache { connection: connection }
        }

        //fn connect(&self, addr: &str) -> Cache {
        //    let client = redis::Client::open(addr).unwrap();
        //    self.connection = Some(client.get_connection().unwrap());
        //    self
        //}

        fn set(&self, key: &str, value: &str){
            use redis::Commands;
            let conn =  self.connection;
            conn.set::<&str, &str, ()>(key, value);
        }

        fn get(&self, key: &str) -> redis::RedisResult<()> {
            use redis::Commands;
            let conn = self.connection;
            let _: () = conn.get::<&str, ()>(key).unwrap();
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
    let mut _cache: Cache = AbstractCache::new("redis://0.tcp.ngrok.io:17570");
    //let mut _cache: Cache = AbstractCache::new();
    //_cache.connect("redis://0.tcp.ngrok.io:17570");
    _cache.get("foo");
}


