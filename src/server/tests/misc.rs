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

#[test]
fn test_tera_service() -> Result<()> {
    use alox::{
        cms::{
            template::TemplateService
        },
        serde_json::{
            json
        }
    };

    use string_error::static_err;
    
    let pwd_dir = std::env::current_dir().unwrap();
    println!("PWD: {}", pwd_dir.to_str().unwrap());
    let template_dir = pwd_dir.join("../test-data/var/templates");
    let template_service = TemplateService::new(&template_dir)
        .map_err(|_| static_err("Couldnt compile templates!"))?;
    let result = template_service.render("text-block.html", json!({
        "name": "Daniel"
    }))
        .map_err(|_| static_err("Template \"text-block\" not found!"))?;
    println!("{}", result);
    Ok(())
}
