use super::{ExternalClient, Singleton};
use redis::{Client as RedisClient, Commands, Connection, RedisError};

struct Redis {
    conn_str: String,
    instance: RedisClient,
}

pub struct Cache<'a> {
    client: &'a Redis,
}

// impl ExternalClient<RedisClient> for Redis {
//   fn new(host: &str, port: u16, db: &str) -> Self {
//       Redis {
//         connStr: format!("redis://{host}:{port}/{db}"),
//         instance: RedisClient::,
//       }
//   }

//   async fn connect<T, E>(&mut self) -> Result<T, E> {
//       match self.get_instance().get_connection() {
//           Ok(conn) => Ok(conn),
//           Err(e) => Err(e),
//       }
//   }
// }

impl<'a> Cache<'a> {
    fn new(client: &'a Redis) -> Cache<'a> {
        Cache { client }
    }

    fn get_connection(&self) -> Connection {
        self.client.get_instance().get_connection().unwrap()
    }

    fn get(&self, key: &str) -> String {
        let mut conn = self.get_connection();
        conn.get(key).unwrap()
    }

    fn set(&self, key: &str, value: &str, ttl: u64) {
        let mut conn = self.get_connection();
        conn.set_ex::<&str, &str, ()>(key, value, ttl).unwrap();
    }

    fn del(&self, key: &str) {
        let mut conn = self.get_connection();
        conn.del::<&str, ()>(key).unwrap();
    }
}

impl Singleton<RedisClient> for Redis {
    fn get_instance(&self) -> &RedisClient {
        &self.instance
    }
}
