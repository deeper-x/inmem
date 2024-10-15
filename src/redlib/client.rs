use redis::{Client, Cmd, Commands, Connection, RedisError};

pub struct RedisClient {
    conn: Connection,
}

use crate::settings::params::Redis;

impl RedisClient {
    pub fn new() -> Result<RedisClient, RedisError> {
        let rs: Redis = Redis::new();

        let client: Client = redis::Client::open(rs.uri)?;

        let conn: Connection = client.get_connection()?;

        Ok(RedisClient { conn })
    }

    pub fn set(self, k: &str, v: &str) -> Result<(), redis::RedisError> {
        let mut cmd: Cmd = redis::cmd("SET");
        let res = cmd.arg(k).arg(v);
        let mut conn = self.conn;

        let out: Result<(), redis::RedisError> = res.exec(&mut conn);

        out
    }

    pub fn get(&mut self, k: &str) -> Result<String, redis::RedisError> {
        self.conn.get(k)
    }
}
