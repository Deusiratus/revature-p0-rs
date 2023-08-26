use postgres::Client;

use crate::{account::Account, user::User};

pub struct DBClient {
    client: Client,
}

impl DBClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn register_user(&mut self, user: &User) {
        match self.client.execute(
            "insert into revature_p0.users (username, password, email, 
                first_name, last_name, birthday, joined_date, \"age\")
                values ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &user.username,
                &user.password,
                &user.email,
                &user.first_name,
                &user.last_name,
                &user.birthday,
                &user.joined_date,
                &user.age,
            ],
        ) {
            Ok(_) => println!("successfully entered user into the database!"),
            Err(e) => eprintln!("error: {}", e),
        }
    }

    pub fn find_by_username_and_password(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<User, &'static str> {
        match self.client.query(
            "select * from revature_p0.users where username = $1 and password = $2", 
            &[&username, &password]
        ) {
            Ok(rs) => match rs.iter().next() {
                Some(row) => {
                    return Ok(User::new(
                        row.get(0),
                        row.get(1),
                        row.get(2),
                        row.get(3),
                        row.get(4),
                        row.get(5),
                        row.get(6),
                        row.get(7),
                        row.get(8),
                    ));
                }
                None => return Err("No user found with supplied credentials"),
            },
            Err(e) => {
                eprintln!("Problem with database: {e}");
                return Err("There was a problem");
            }
        }
    }

    pub fn accounts_by_user_id(&mut self, id: i32) -> Result<Vec<Account>, String> {
        let query = "select 
                ac.account_id, ac.account_name, ac.balance
                from revature_p0.account ac
                join revature_p0.user_account_map uam
                on uam.account_id = ac.account_id
                join revature_p0.users usr
                on usr.user_id = uam.user_id
                where usr.user_id = $1";
        
        let result = match self.client.query(query, &[&id]) {
            Ok(vec) => vec,
            Err(e) => return Err(format!("{e}"))
        };

        let accounts = result
            .iter()
            .map(|row| Account{ id: row.get(0), name: row.get(1), balance: row.get(2) })
            .collect::<Vec<Account>>();

        Ok(accounts)
    }

    pub fn open_account(
        &mut self, 
        name: &str, 
        starting_balance: f64,
        user_id: i32,
    ) -> Result<Account, &'static str> {
        let query = match self.client.query(
            "insert into revature_p0.account(account_name, balance) values($1, $2) returning account_id", 
            &[&name, &starting_balance]
        ) {
            Ok(row) => row,
            Err(_) => return Err("Something went wrong in the database")
        };

        let account = match query.iter().next() {
            Some(row) => Account { id: row.get(0), name: name.to_string(), balance: starting_balance },
            None => return Err("No results returned"),
        };

        let _link_query = match self.client.execute(
            "insert into revature_p0.user_account_map(user_id, account_id) values ($1, $2)", 
            &[&user_id, &account.id]
        ) {
            Ok(_) => (),
            Err(_) => return Err("failed to link account")
        };

        Ok(account)
    }

    pub fn save_account(&mut self, id: i32, blalance: f64) -> Result<(), String> {
        let _query = match self.client.execute(
            "update revature_p0.account set balance = $1 where account_id = $2", 
            &[&blalance, &id],
        ) {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
                return Err(format!("{e}"))
            }
        };

        Ok(())
    }

    pub fn add_balance(&mut self, balance: f64, recipient: i32) -> Result<(), String> {
        let query = "update revature_p0.account set balance = balance + $1 where account_id = $2";

        let _query = match self.client.execute(query, &[&balance, &recipient]) {
            Ok(_) => (),
            Err(e) => return Err(format!("{e}"))
        };

        Ok(())
    }

    pub fn account_exists(&mut self, id: i32) -> bool {
        let query = "select count(*) from revature_p0.account where account_id = $1";

        let result = match self.client.query_one(query, &[&id]) {
            Ok(row) => row,
            Err(e) => {
                eprintln!("Problem with fetching acocunt: {e}");
                return false;
            }
        };
        
        result.get::<usize, i64>(0) != 0
    }
}
