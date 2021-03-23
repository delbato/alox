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
            App
        }
    },
    repo::{
        permission::PermissionRepo,
        app::AppRepo
    }
};

use std::{
    path::PathBuf,
    ops::Deref
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
    pub enabled: Option<bool>,
    pub root_dir: Option<PathBuf>
}

impl Into<App> for &AppCreateBody {
    fn into(self) -> App {
        let enabled = if let Some(enabled) = self.enabled {
            enabled
        } else { false };

        let root_dir = if let Some(root_dir) = self.root_dir.clone() {
            root_dir
        } else {
            PathBuf::from("/var/apps").join(&self.ident)
        };
        App {
            key: None,
            ident: self.ident.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            enabled,
            root_dir
        }
    }
}

#[post("/apps")]
pub async fn create_action(jwt_claims: JwtClaims, app_create_body: Json<AppCreateBody>, app_repo: AppRepo, perm_repo: PermissionRepo) -> ApiResult {
    let user_key = jwt_claims.user.key.as_ref().unwrap();
    if !jwt_claims.user.is_admin {
        return ApiResult::error(401, "Not authorized");
    }

    let mut app: App = app_create_body.deref().into();

    app = app_repo.insert(app).await?;

    let perm = Permission {
        permission_type: PermissionType::Owner,
        key_user: user_key.clone(),
        key_app: app.key.as_ref().cloned().unwrap(),
        key: None
    };

    perm_repo.insert(perm).await?;
    
    ApiResult::success(app)
}

#[post("/apps/{app_key}/site")]
pub async fn set_site_action(jwt_claims: JwtClaims, app_repo: AppRepo, perm_repo: PermissionRepo) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}