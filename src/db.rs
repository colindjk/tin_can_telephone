/// Database implementation for initializing, updating, and reading from
/// database tables.

use stanza;

use std::io;

use tokio_core::reactor::{Core};

use my::prelude;
use my::{
    OptsBuilder,
    Pool,
};

pub struct Database {
    core: Core,
    pool: Pool,
}

impl Database {

    /// Init's a connection to the DB, returning a result of whether the
    /// operation ran successfully or not.
    pub fn init() -> Result<Database, io::Error> {
        let core = Core::new()?;
        let handle = &core.handle();
        let mut opts = OptsBuilder::new();
        opts.ip_or_hostname("localhost")
            .tcp_port(3306)
            .user(Some("root"))
            .pass(Some("Winter13!"));

        let db = Database { core: core, pool: Pool::new(opts, handle) };

        Ok(db)
    }

    //pub connect() -> 

}


