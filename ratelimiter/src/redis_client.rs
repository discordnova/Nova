use std::error::Error;

use redis::{Client, Commands, Connection};

use crate::bucket::Bucket;

pub struct RedisClient {
    conn: Connection,
}

impl RedisClient {
    /// Create a new RedisClient which connects to a given node
    fn new(addr: &str) -> RedisClient {
        let client = Client::open(addr).unwrap();
        let conn = client.get_connection().unwrap();
        RedisClient { conn }
    }

    /// Get a bucket using its hash and return it
    fn get_bucket(&mut self, hash: &str) -> Result<Bucket, Box<dyn Error>> {
        let stored_data: String = self
            .conn
            .get(format!("nova:ratelimiter:buckets:{}", hash))?;

        return Ok(serde_json::from_str(stored_data.as_str())?);
    }

    /// Check if a bucket is in the Redis store
    fn has_bucket(&mut self, hash: &str) -> Result<bool, Box<dyn Error>> {
        match self
            .conn
            .exists(format!("nova:ratelimiter:buckets:{}", hash))
        {
            Err(e) => Err(e.into()),
            Ok(o) => Ok(o),
        }
    }

    /// Create a bucket in the Redis store
    fn create_bucket(&mut self, hash: &str, bucket: &Bucket) -> Result<(), Box<dyn Error>> {
        let data: String = serde_json::to_string(&bucket)?;
        self.conn
            .set(format!("nova:ratelimiter:buckets:{}", hash), data)?;
        Ok(())
    }

    /// Delete a bucket using its hash
    fn delete_bucket(&mut self, hash: &str) -> Result<(), Box<dyn Error>> {
        self.conn
            .del(format!("nova:ratelimiter:buckets:{}", hash))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use crate::bucket::{Bucket, BucketRequest};

    // Need a Redis to test right now
    #[test]
    fn test_redis_client() {
        let mut client = crate::redis_client::RedisClient::new("redis://127.0.0.1:6379/");
        let bc = Bucket {
            limit: 50,
            remaining: 50,
            reset: 0,
            request: BucketRequest {
                route_name: String::from("/hello_world"),
                identifiers: vec![String::from("joe"), String::from("mama")],
            },
        };

        let hash = "test";

        assert_eq!(client.has_bucket(hash).unwrap(), false);

        match client.create_bucket(hash, &bc) {
            Err(e) => panic!("{:?}", &e),
            Ok(_) => {}
        };
        assert_eq!(client.has_bucket(hash).unwrap(), true);

        let bck = match client.get_bucket(hash) {
            Err(e) => panic!("{:?}", &e),
            Ok(b) => b,
        };
        assert_eq!(bck.limit, bc.limit);

        match client.delete_bucket(hash) {
            Err(e) => panic!("{:?}", &e),
            Ok(_) => {}
        };
        assert_eq!(client.has_bucket(hash).unwrap(), false);
    }
}
