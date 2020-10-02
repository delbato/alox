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

#[get("/apps/{app_key}")]
pub async fn get_action(app_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[put("/apps/{app_key}")]
pub async fn update_action(app_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[delete("/apps/{app_key}")]
pub async fn delete_action(app_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[post("/apps")]
pub async fn create_action() -> ApiResult {
    ApiResult::error(503, "Not implemented")
}