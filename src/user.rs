use chrono::prelude::*;

#[derive(Debug)]
pub struct User {
    id: i32,
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
        id: i32,
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
            id,
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

    pub fn is_valid(&self) -> Result<(), &'static str> {

        if self.first_name.trim().is_empty() || self.first_name.len() > 50 { 
            return Err("First name cannot be empty or more than 50 characters"); 
        }

        if self.last_name.trim().is_empty() || self.last_name.len() > 50 { 
            return Err("Last name cannot be empty or more than 50 characters"); 
        }

        if self.email.trim().is_empty() || self.email.len() > 255 { 
            return Err("Email cannot be empty or more than 255 characters"); 
        }

        if self.password.trim().is_empty() { 
            return Err("Password cannot be empty"); 
        }

        if self.username.trim().is_empty() || self.username.len() > 50 { 
            return Err("Username cannot be empty or more that 50 characters"); 
        }
        
        if self.age < 0 || self.age > 200 {
            return Err("Your age is likely not negative or over 200");
        }

        Ok(())
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}