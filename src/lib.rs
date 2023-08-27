use db::DBClient;
use postgres::{Client, NoTls};

use screen::{Screen, ScreenType};

mod account;
mod console;
mod db;
mod screen;
mod user;

pub struct App {
    screen: Option<Screen>,
    running: bool,
}

impl App {
    /// Constructs an App instance.
    ///
    /// # Panics
    /// This function will panic when the database connection fails for any reasn.
    pub fn new() -> Self {
        let client: Client =
            match Client::connect("host=localhost user=postgres password=password", NoTls) {
                Ok(client) => client,
                Err(e) => panic!("unable to construct database client: {}", e),
            };
        let screen = Some(Screen::new(DBClient::new(client)));

        Self {
            screen,
            running: true,
        }
    }

    pub fn run(&mut self) {
        loop {
            let screen = self.screen.as_mut().unwrap();
            screen.render();
            let selection = console::get_input(">", "Pleas enter a valid whole number.");

            match selection {
                1 => screen.navigate(ScreenType::Login),
                2 => screen.navigate(ScreenType::Register),
                3 => self.running = false,
                _ => println!("The choice you entered was invalid"),
            }

            if !self.running {
                drop(self.screen.take());
                break;
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
