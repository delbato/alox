use actix_web::{
    get,
    put,
    post,
    delete,
    web::{
        HttpRequest,
        HttpResponse,
    },
    http::{
        StatusCode
    }
};

#[put("/users")]
pub async fn login_action(request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Not implemented")
}

pub async fn list_action(request: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body("
            {
                \"success\":true
            }
        ")
}