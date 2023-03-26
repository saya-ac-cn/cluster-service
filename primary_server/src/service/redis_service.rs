use std::sync::{Arc,Mutex};
use std::time::Duration;

use crate::error::{Error, Result};
use log::error;
use redis::aio::Connection;
use redis::{AsyncCommands, Commands, RedisResult};
///Redis Cache service
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        println!("[primary_server] conncect redis ({})...", url);
        let client = redis::Client::open(url).unwrap();
        println!("[primary_server] conncect redis success!");
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(crate::error::Error::from(err));
        }
        return Ok(conn.unwrap());
    }


    pub async fn exists(&self, k: &str) -> Result<bool> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        let result:RedisResult<Option<bool>> = conn.exists(&k).await;
        return match result {
            Ok(v) => Ok(v.unwrap()),
            Err(e) => Err(Error::from(format!(
                "RedisService exists({}) fail:{}",
                k,
                e.to_string()
            ))),
        };
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let k = k.to_string();
        let v = v.to_string();
        return self.set_string_ex(&k, &v, None).await;
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        let result: RedisResult<Option<String>> =
            redis::cmd("GET").arg(&[&k]).query_async(&mut conn).await;
        return match result {
            Ok(v) => Ok(v.unwrap_or_default()),
            Err(e) => Err(Error::from(format!(
                "RedisService get_string({}) fail:{}",
                k,
                e.to_string()
            ))),
        };
    }

    ///set_string Automatically expire
    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        let k = k.to_string();
        let v = v.to_string();
        let mut conn = self.get_conn().await?;
        return if ex.is_none() {
            match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        } else {
            match redis::cmd("SET")
                .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
                .query_async(&mut conn)
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        };
    }

    ///set_string Automatically expire
    pub async fn ttl(&self, k: &str) -> Result<i64> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!(
                "RedisService ttl fail:{}",
                e.to_string()
            ))),
        };
    }
}
