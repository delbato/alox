use crate::{
    api::{
        error::{
            ApiResult,
            ApiResultExt
        }
    }
};

use actix_web::{
    web::{
        Json,
        Path
    },
    get,
    post,
    put,
    delete
};

#[get("/buckets/{bucket_key}")]
pub async fn get_action(bucket_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[post("/buckets/{bucket_key/query")]
pub async fn query_action(bucket_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[put("/buckets/{bucket_key}")]
pub async fn update_action(bucket_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[delete("/buckets/{bucket_key}")]
pub async fn delete_action(bucket_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[post("/buckets")]
pub async fn create_action() -> ApiResult {
    ApiResult::error(503, "Not implemented")
}