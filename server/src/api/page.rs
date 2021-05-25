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

#[get("/{page_key}")]
pub async fn get_action(page_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[put("/{page_key}")]
pub async fn update_action(page_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[delete("/{page_key}")]
pub async fn delete_action(page_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[post("")]
pub async fn create_action() -> ApiResult {
    ApiResult::error(503, "Not implemented")
}