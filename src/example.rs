// Examples for understanding concepts will be located here.

use std::error::Error;

use rustful::{ Server, Handler, Context, Response, TreeRouter };

struct Salutation(&'static str);

impl Handler for Salutation {
    /// Remember, 'variables' as in path / route variables.
    fn handle_request(&self, context: Context, response: Response) {
        if let Some(smash_bro) = context.variables.get("character") {
            // use the value 'smash_bro', taken out of optional variables
            response.send(format!("{} {}!", self.0, smash_bro));
        } else {
            response.send(self.0)
        }
    }
}

/// A server running a router interface example.
fn smash_bros_server() -> Server {
    let router = insert_routes! { 
        TreeRouter::new() => {
            // Lets define some GET requests!
            "select" => {
                Get: Salutation("Choose a character!"),
                ":character" => Get: Salutation("You picked:")
            }
            "result" => {
                Get: Salutation("GAME!"),
                ":character" => Get: Salutation("The winner is,")
            }
        }
    };

    Server {
        // Give the server a closure to use for "handling" events.
        handlers: my_router,
        host: 6767.into(),

        // Okay we don't care about any of Servers other fields
        ..Server::default()
    }
}
