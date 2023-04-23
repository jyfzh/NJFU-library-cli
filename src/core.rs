use crate::cli::Action;
use crate::cli::Action::*;
use crate::context;
use log::*;

pub fn handle_action(action: Action) {
    debug!("Action: {:?}", action);
    // Perform the action.
    match action {
        Query { name, site } => {
            let context = context::Context::new();
            if name.is_some() {
                match context.query_by_name(name.unwrap()) {
                    Some(result) => println!("{}", result),
                    None => println!("No such student in library."),
                }
            } else if site.is_some() {
                match context.query_by_site(site.unwrap()) {
                    Some(result) => println!("{}", result),
                    None => println!("No student owner the site."),
                }
            }
        }

        Login { username, password } => {
            let context = context::Context::new();
            match context.login(username, password) {
                Some(result) => println!("{}", result),
                None => println!("Login failed."),
            }
        }

        Status {} => {
            let context = context::Context::new();
            match context.status() {
                Some(result) => println!("{}", result),
                None => println!("no data."),
            }
        }
    };
}
