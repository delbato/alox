use crate::{
    model::Model
};

use noria::{
    DataType
};

macro_rules! get_string {
    ($row:expr) => {
        match $row {
            DataType::Text(c_str) => {
                let raw_str = c_str.to_str().map_err(|_| ())?;
                String::from(raw_str)
            },
            _ => return Err(())
        }
    };
}

macro_rules! get_bool {
    ($row:expr) => {
        match $row {
            DataType::UnsignedInt(uint) => if *uint == 0 { false } else { true },
            _ => return Err(())
        }
    };
}

macro_rules! get_i64 {
    ($row:expr) => {
        match $row {
            DataType::BigInt(int) => *int,
            _ => return Err(())
        }
    };
}

pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
    pub is_admin: bool
}

impl User {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            username: String::new(),
            email: String::new(),
            password: String::new(),
            password_salt: String::new(),
            is_admin: false,
        }
    }
}

impl Model for User {
    fn into_row(self) -> Vec<DataType> {
        vec![
            self.id.into(),
            self.username.into(),
            self.email.into(),
            self.password.into(),
            self.password_salt.into(),
            DataType::UnsignedInt(self.is_admin as u32)
        ]
    }

    fn from_row(row: &[DataType]) -> Result<Self, ()> {
        if 6 != row.len() {
            return Err(());
        }
        Ok(
            Self {
                id: get_i64!(&row[0]),
                username: get_string!(&row[1]),
                email: get_string!(&row[2]),
                password: get_string!(&row[3]),
                password_salt: get_string!(&row[4]),
                is_admin: get_bool!(&row[5]),
            }
        )
    }
}