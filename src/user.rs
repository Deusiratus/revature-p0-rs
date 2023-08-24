use chrono::prelude::*;

pub struct User {
    id: usize,
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: NaiveDate,
    pub joined_date: DateTime<Local>,
    pub age: i32,
}


impl User {
    pub fn new(
        username: String,
        password: String,
        email: String,
        first_name: String,
        last_name: String,
        birthday: NaiveDate,
        joined_date: DateTime<Local>,
        age: i32
    ) -> Self {
        Self {
            id: 0,
            username,
            password,
            email,
            first_name,
            last_name,
            birthday,
            joined_date,
            age
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.first_name.trim().is_empty() || self.first_name.len() > 50 { return false; }
        if self.last_name.trim().is_empty() || self.last_name.len() > 50 { return false; }
        if self.email.trim().is_empty() || self.email.len() > 255 { return false; }
        if self.password.trim().is_empty() { return false; }
        if self.username.trim().is_empty() || self.username.len() > 50 { return false; }
        
        return self.age >= 0 && self.age <= 200;
    }
}