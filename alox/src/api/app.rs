use crate::{
    api::{
        error::{
            ApiResult,
            ApiResultExt
        }
    },
    util::{
        jwt::JwtClaims
    },
    model::{
        permission::{
            Permission,
            PermissionType
        },
        app::{
            App,
            AppFlat,
            AppFull
        }
    },
    repo::{
        permission::PermissionRepo,
        app::AppRepo
    }
};

use std::{
    path::PathBuf
};

use serde::{
    Deserialize
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

#[derive(Deserialize)]
pub struct AppCreateBody {
    pub ident: String,
    pub name: String,
    pub description: Option<String>,
    pub http_enabled: bool,
    pub https_enabled: bool,
    pub https_redir: bool,
    pub root_dir: Option<PathBuf>
}

#[post("/apps")]
pub async fn create_action(jwt_claims: JwtClaims, app_repo: AppRepo, perm_repo: PermissionRepo) -> ApiResult {
    let user_key = jwt_claims.user.key.as_ref().unwrap();
    
    ApiResult::error(503, "Not implemented")
}