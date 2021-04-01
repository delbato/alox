use crate::{
    model::{
        user::{
            UserFlat,
            UserNoPw
        }
    }
};

use serde::{
    Serialize,
    Deserialize
};
use jwt::{
    Header,
    encode,
    decode,
    Validation,
    EncodingKey,
    DecodingKey
};
use chrono::{
    DateTime,
    Utc,
    Duration
};
use actix_web::{
    FromRequest,
    HttpRequest,
    dev::Payload,
    web::Data
};
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub user: UserNoPw
}

impl From<UserFlat> for JwtClaims {
    fn from(user: UserFlat) -> Self {
        let exp: DateTime<Utc> = Utc::now() + Duration::days(30);
        Self {
            sub: user.username.clone(),
            aud: String::from("alox"),
            exp: exp.timestamp() as usize,
            user: user.into()
        }
    }
}

impl FromRequest for JwtClaims {
    type Error = ();
    type Config = ();
    type Future = impl Future<Output = Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt_manager = req.app_data::<Data<JwtManager>>()
            .cloned()
            .unwrap();
        let auth_header = req.headers().get("Authorization").unwrap();
        let auth_string = auth_header.to_str().unwrap();
        let auth_split: Vec<&str> = auth_string.split(" ").collect();
        let token = String::from(auth_split[1]);
        async move {
            jwt_manager.validate_token(&token)
                .ok_or(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub username: String,
}

#[derive(Clone)]
pub struct JwtManager {
    encoding_key: EncodingKey,
    secret: String
}

impl JwtManager {
    pub fn new(secret: &str) -> Self {
        let secret_bytes = secret.as_bytes();
        Self {
            secret: String::from(secret),
            encoding_key: EncodingKey::from_secret(secret_bytes)
        }
    }

    pub fn generate_token(&self, claims: JwtClaims) -> String {
        let header = Header::default();
        encode(&header, &claims, &self.encoding_key).expect("Couldnt create JWT token!")
    }

    pub fn validate_token(&self, token: &str) -> Option<JwtClaims> {
        let validation = Validation::default();
        let decoding_key = DecodingKey::from_secret(self.secret.as_bytes());
        let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
            .ok()?;
        Some(token_data.claims)
    }
}