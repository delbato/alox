use actix_web::{
    web::{
        HttpRequest,
        HttpResponse,
    },
    http::{
        StatusCode
    }
};

pub async fn list_action(request: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body("
            {
                \"success\":true
            }
        ")
}