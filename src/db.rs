/// Database implementation for initializing, updating, and reading from
/// database tables.

use stanza;

use std::io;

use tokio_core::reactor::{Core};

use my::prelude;
use my::{
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
        let pool = Pool::new(
            "mysql://root:Winter13!@localhost:3307", &core.handle());

        let db = Database { core: core, pool: pool };

        Ok(db)
    }

    // thing
    //pub fn 

}


