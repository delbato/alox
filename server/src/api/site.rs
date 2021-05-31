use crate::{
    api::{
        error::{
            ApiResult,
            ApiResultExt
        }
    },
    model::{
        site::SiteFlat
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

#[get("/{site_key}")]
pub async fn get_action(site_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[put("/{site_key}")]
pub async fn update_action(site_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[delete("/{site_key}")]
pub async fn delete_action(site_key: Path<String>) -> ApiResult {
    ApiResult::error(503, "Not implemented")
}

#[derive(Debug, Deserialize)]
struct SiteCreateBody {
    pub enabled: bool,
    pub base_url: String,
    pub host: String,
    pub name: String,
    pub description: Option<String>
}

impl Into<SiteFlat> for Json<SiteCreateBody> {
    fn into(self) -> SiteFlat {
        SiteFlat {
            key: None,
            enabled: self.enabled,
            base_url: self.base_url.clone(),
            host: self.host.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            key_root_page: None
        }
    }
}

#[post("")]
pub async fn create_action(site_create_body: Json<SiteCreateBody>) -> ApiResult {
    let site: SiteFlat = site_create_body.into();

    

    ApiResult::error(503, "Not implemented")
}