use crate::{
    api::{
        error::{
            ApiResult,
            ApiResultExt
        }
    },
    model::{
        perm::{
            Permission,
            PermissionType
        }
    },
    repo::{
        perm::PermissionRepo
    },
    util::{
        jwt::JwtClaims
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
use serde::Deserialize;

#[get("/{perm_key}")]
pub async fn get_action(perm_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[get("/by-user/{user_key}")]
pub async fn get_by_user_action(user_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[put("/{perm_key}")]
pub async fn update_action(perm_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[delete("/{perm_key}")]
pub async fn delete_action(perm_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[derive(Deserialize)]
struct PermissionCreateBody {
    pub key_site: String,
    pub key_user: String,
    pub perm_type: PermissionType
}

#[post("")]
pub async fn create_action(perm_create_body: Json<PermissionCreateBody>, jwt_claims: JwtClaims, perm_repo: PermissionRepo) -> ApiResult {
    let user_key = jwt_claims.user.key.as_ref().unwrap();
    let perm = perm_repo.find_by_user_and_site(user_key, &perm_create_body.key_site).await?;
    ApiResult::error(503, "Not implemented")
}