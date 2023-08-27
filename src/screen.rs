use crate::{db::DBClient, console, user::User, account::Account};

pub struct Screen {
    db_client: DBClient,
    screen_type: ScreenType,
    rendered: bool,
    logged_in_user: Option<User>,
    active_account: Option<Account>,
    user_accounts: Option<Vec<Account>>
}
pub enum ScreenType {
    Account,
    Welcome,
    Dashboard,
    Login,
    Register,
}

impl Screen {
    pub fn new(db_client: DBClient) -> Self {
        Self {
            db_client,
            screen_type: ScreenType::Welcome,
            rendered: false,
            logged_in_user: None,
            active_account: None,
            user_accounts: None,
        }
    }

    pub fn render(&mut self) {
        if self.rendered {
            return;
        }

        match self.screen_type {
            ScreenType::Welcome => Self::show_welcome_screen(),
            ScreenType::Register => Self::show_register_screen(self),
            ScreenType::Login => Self::login_user(self),
            ScreenType::Dashboard => Self::show_dashboard(self),
            ScreenType::Account => Self::show_account_screen(self),
        }

        self.rendered = true;
    }

    pub fn navigate(&mut self, screen_type: ScreenType) {
        self.screen_type = screen_type;
        self.rendered = false;
        self.render();
    }

    fn show_welcome_screen() {
       println!("Welcome to the Bank of Rust!");
       println!("Please choose an option");
       println!("1) Login");
       println!("2) Register");
       println!("3) Exit");
    }

    fn show_register_screen(&mut self) {
        println!("Registration");

        let username = console::get_string("Enter a username:");
        let password = console::get_string("Enter a password: ");
        let email = console::get_string("Enter your email: ");
        let first_name  = console::get_string("Enter first name:");
        let last_name = console::get_string("Enter your last name: ");
        let birthday = console::get_date();
        let joined_date = chrono::offset::Local::now();
        let age: i32 = console::get_input("Enter your age: ", "Please enter a valid whole number");

        let user = User { 
            id: 0, 
            username, 
            password, 
            email, 
            first_name, 
            last_name, 
            birthday, 
            joined_date, 
            age
        };

        if let Err(str) = user.is_valid() {
            println!("{}", str);
            self.navigate(ScreenType::Register);
        }

        self.db_client.register_user(&user);

        self.navigate(ScreenType::Login)
    }

    fn login_user(&mut self) {
        println!("Login screen");
        let username = console::get_string("Enter your username:");
        let password = console::get_string("Enter your password:");

        if let Ok(user) = self.db_client.find_by_username_and_password(&username, &password) {
            println!("Login successful!");
            self.logged_in_user = Some(user);
            self.navigate(ScreenType::Dashboard);
        } else {
            println!("Login failed: username or password was incorrect");
            self.navigate(ScreenType::Login);
        }
    }

    fn show_dashboard(&mut self) {
        let user = match self.logged_in_user {
            Some(ref user) => user,
            // should never happen, as it should not be possible to
            // get to this screen without logging in
            None => {
                println!("Please log in first");
                return self.navigate(ScreenType::Login);
            }
        };

        println!("Dashboard");
        println!("================");
        println!("Welcome, {}!", user.first_name);
        println!("What would you like to do today?");
        println!("1) Access an account");
        println!("2) Open a new account");
        println!("3) Logout");

        let selection = console::get_input(">", "Please enter a valid whole number");

        match selection {
            1 => {
                let accounts = match self.db_client.accounts_by_user_id(user.id()) {
                    Ok(accounts) => accounts,
                    Err(str) => {
                        println!("{}", &str);
                        return self.navigate(ScreenType::Dashboard);
                    }
                };

                if accounts.is_empty() {
                    println!("You have no registered accounts! Please create one first.");
                    self.navigate(ScreenType::Dashboard)
                }

                for acc in &accounts[..] {
                    println!("Account ID: {}", acc.id);
                    println!("Name: {}", acc.name);
                    println!("Balance: ${:.2}", acc.balance);
                    println!("{}", "=".repeat(12));
                }

                self.user_accounts = Some(accounts);

                let selected_account = loop {
                    let name = console::get_string("Enter an account name");

                    let account = self.user_accounts.as_ref().unwrap()
                        .iter()
                        .find(|acc| acc.name == name);

                    match account {
                        Some(account) => break account,
                        None => {
                            println!("You have no account by that name");
                            continue;
                        }
                    }
                };

                self.active_account = Some(selected_account.clone());
                self.navigate(ScreenType::Account)
            }
            2 => {
                let name = console::get_string("Enter an account name");
                let starting_balance = console::get_input("Enter a starting balance", 
                    "Please enter a valid decimal number");
                
                let created_account = self.db_client.open_account(
                    &name, 
                    starting_balance,
                    user.id(),
                );

                let created_account = match created_account {
                    Ok(account) => account,
                    Err(str) => {
                        println!("{}", str);
                        return self.navigate(ScreenType::Dashboard);
                    }
                };

                self.active_account = Some(created_account);
                self.navigate(ScreenType::Account)
            }
            3 => {
                self.navigate(ScreenType::Welcome)
            }
            _ => {
                println!("Invalid choice");
                self.navigate(ScreenType::Dashboard)
            }
        }
    }

    fn show_account_screen(&mut self) {
        let account = self.active_account
            .as_mut()
            .expect("Account should be present");
        println!("{}", account.name);
        println!("=============================");
        println!("Current account: {}", account.name);
        println!("Current Balance: ${:.2}", account.balance);
        println!("Please choose an option");
        println!("1) Deposit funds");
        println!("2) Withdraw funds");
        println!("3) Make a transfer");
        println!("4) Back to dashboard");

        let selection = console::get_input("<", "Please enter a valid whole number");

        match selection {
            1 => {
                let amount: f64 = console::get_input("Enter amount:", "Please enter a valid decimal number");
                account.balance += amount;
                self.db_client.save_account(account.id, account.balance).expect("Problem saving account");
                println!("Your new balance is: ${:.2}", account.balance);
                self.navigate(ScreenType::Account)
            },
            2 => {
                let amount = get_f64_in_bound(account.balance);

                account.balance -= amount;
                self.db_client.save_account(account.id, account.balance).expect("Problem saving account");
                println!("Your new balance is: ${:.2}", account.balance);
                self.navigate(ScreenType::Account)
            },
            3 => {
                let recipient = loop {
                    let id = console::get_input("Enter an account id:", "Please enter a valid whole number");

                    if !self.db_client.account_exists(id) {
                        println!("No account with that id exists");
                        continue;
                    }
                    break id;
                };

                let amount = get_f64_in_bound(account.balance);
                account.balance -= amount;
                self.db_client.save_account(account.id, account.balance).expect("Problem saving account");
                self.db_client.add_balance(amount, recipient).expect("Problem making transfer");
                self.navigate(ScreenType::Account)
            },
            _ => {
                self.navigate(ScreenType::Dashboard)
            }
        }
    }
}

fn get_f64_in_bound(bound: f64) -> f64 {
    loop {
        let num = console::get_input::<f64>("Enter amount:", "Please enter a valid decimal number");
        if num > bound {
            println!("Insufficient funds");
            continue;
        }
        break num;
    }
}