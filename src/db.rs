use postgres::Client;

use crate::user::User;

pub struct DBClient {
    client: Client,
}

impl DBClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn register_user(&mut self, user: &User) {
        let query = "insert into revature_p0.users (username, password, email, 
            first_name, last_name, birthday, joined_date, \"age\")
            values ($1, $2, $3, $4, $5, $6, $7, $8)";
        
        match self.client.execute(query, &[
            &user.username,
            &user.password,
            &user.email,
            &user.first_name,
            &user.last_name,
            &user.birthday,
            &user.joined_date,
            &user.age
        ]) {
            Ok(_) => println!("successfully entered user into the database!"),
            Err(e) => println!("error: {}", e),
        }
    }
}

