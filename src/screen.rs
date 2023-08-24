use crate::{db::DBClient, console, user::User};

pub struct Screen {
    db_client: DBClient,
    screen_type: ScreenType,
    rendered: bool,
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
        }
    }

    pub fn render(&mut self) {
        if self.rendered {
            return;
        }

        match self.screen_type {
            ScreenType::Welcome => Self::show_welcome_screen(),
            ScreenType::Register => Self::show_register_screen(self),
            _ => println!("Some other screen"),
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

        let user = User::new(username, password, email, first_name, last_name, birthday, joined_date, age);

        if !user.is_valid() {
            println!("invalid data supplied");
            self.navigate(ScreenType::Register);
        }

        self.db_client.register_user(&user);

        self.navigate(ScreenType::Login);
    }
}