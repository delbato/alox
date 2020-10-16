extern crate alox;

use std::{
    result::Result as StdResult,
    error::Error,
};

use alox::{
    util::{
        jwt::{
            JwtManager,
            JwtClaims
        }
    },
    model::{
        user::{
            UserFlat,
            UserNoPw
        }
    }
};

type Result<T> = StdResult<T, Box<dyn Error>>;


#[test]
fn test_misc_jwt_generate() -> Result<()> {
    let mut user = UserFlat::new();
    user.key = Some(String::from("0"));
    user.username = String::from("wrckn");
    user.email = String::from("wrckn");
    user.is_admin = true;
    let manager = JwtManager::new("some-secret");
    let token = manager.generate_token(JwtClaims::from(user));
    println!("Token: {}", token);
    Ok(())
}
