use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct BucketRequest {
    pub route_name: String,
    pub identifiers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket {
    pub request: BucketRequest,
    pub limit: i32,
    pub remaining: i32,
    pub reset: i32,
}

impl Bucket {
    fn new(request: BucketRequest, limit: i32, remaining: i32, reset: i32) -> Bucket {
        Bucket {
            request,
            limit,
            remaining,
            reset,
        }
    }
}
