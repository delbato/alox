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

#[get("/{content_key}")]
pub async fn get_action(content_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[put("/{content_key}")]
pub async fn update_action(content_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[delete("/{content_key}")]
pub async fn delete_action(content_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[post("")]
pub async fn create_action() -> ApiResult {
    ApiResult::error(503, "Not implemented")
}