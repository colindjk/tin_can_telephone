/// Database implementation for initializing, updating, and reading from
/// database tables.

use stanza::Stanza;

use std::io;

use tokio_core::reactor::{Core};

use futures::Future;

use my::{
    OptsBuilder,
    Pool,
    Params
};

fn users() -> String { "users".to_string() }
fn user_interactions() -> String { "user_interactions".to_string() }
fn group_memberships() -> String { "group_memberships".to_string() }
fn private_messages() -> String { "private_messages".to_string() }
fn group_messages() -> String { "group_messages".to_string() }

pub struct Database {
    core: Core,
    pool: Pool,
}

impl Database {

    /// Init's a connection to the DB, returning a result of whether the
    /// operation ran successfully or not.
    pub fn init_db<S: Into<String>>(db: S) -> Result<Database, io::Error> {
        let core = Core::new()?;
        let handle = &core.handle();
        let mut opts = OptsBuilder::new();
        opts.ip_or_hostname("localhost")
            .tcp_port(3306)
            .user(Some("root"))
            .pass(Some("Winter13!"))
            .db_name(Some(db));

        let db = Database { core: core, pool: Pool::new(opts, handle) };

        Ok(db)
    }

    /// Records the Stanza into the db, performing different actions depending
    /// on the value of the enum fields.
    /// 
    pub fn record_stanza(&mut self, stanza: Stanza) {

        let params = self.record_stanza_params(stanza.clone()).unwrap();
        //let conn = self.pool.get_conn(); // TODO: pass a conn into record stanza

        match stanza {
            Stanza::Message { .. } => {
                self.record_private_message(params);
            },
            Stanza::GroupMessage { .. } => {
                self.record_group_message(params);
            },
            Stanza::Register { .. } => {
                self.register_user(params);
            },
            Stanza::RegisterGroup { .. } => {
                self.register_group(params);
            },
            Stanza::LoginCredentials { .. } => {
                self.record_event(params);
            },

            Stanza::Request { .. } => panic!("No mess"),
            Stanza::Response { .. } => panic!("No mess"),
            _ => panic!("No mess"),

        };
    }

    /// Spawns a thread to record a private message.
    #[inline]
    fn record_private_message(&mut self, params: Params) {
        let task = self.pool.get_conn().and_then(|conn| {
            conn.prep_exec(r"INSERT INTO :table (to, from, msg) \
                 VALUES (:to, :from, :msg)", params)
        });
        self.core.run(task).unwrap();
    }

    /// Spawns a thread to record a private message.
    #[inline]
    fn record_group_message(&mut self, params: Params) {
        let task = self.pool.get_conn().and_then(|conn| {
            conn.prep_exec(r"INSERT INTO :table (group_id, from, msg) \
                 VALUES (:to, :from, :msg)", params)
        });
        self.core.run(task).unwrap();
    }

    /// Adds a user to a database.
    #[inline]
    fn register_user(&mut self, params: Params) {
        let task = self.pool.get_conn().and_then(|conn| {
            conn.prep_exec(r"INSERT INTO :table (user_id, password) \
                 VALUES (:user_id, :password)", params)
        });
        self.core.run(task).unwrap();
    }

    /// Spawns the register_user.
    #[inline]
    fn register_group(&mut self, params: Params) {
        let task = self.pool.get_conn().and_then(|conn| {
            conn.prep_exec(r"INSERT INTO :table (group_id, admin_id) \
                 VALUES (:group_id, :admin_id)", params)
        });
        self.core.run(task).unwrap();
    }

    /// Spawns a thread to record a private message.
    #[inline]
    fn record_event(&mut self, params: Params) {
        let task = self.pool.get_conn().and_then(|conn| {
            conn.prep_exec(r"INSERT INTO :table (event, user_id) \
                 VALUES (:event, :user_id)", params)
        });
        self.core.run(task).unwrap();
    }

    /// Function to record the stanza type, and plop that type's info into it's
    /// respective table.
    pub fn record_stanza_params(&mut self, stanza: Stanza) -> Result<Params, ()> {
        let params = match stanza {

            Stanza::Message { to, from, msg, } => { params!{
                "table" => private_messages(),
                "to" => to, "from" => from, "msg" => msg
            } },
            Stanza::GroupMessage { to, from, msg, .. } => { params!{
                "table" => group_messages(),
                "to" => to, "from" => from, "msg" => msg
            } },

            // For recording data, this moves from to / from standard.
            Stanza::Register { user, psw, } => { params!{
                "table" => users(),
                "user_id" => user, "password" => psw,
            } },
            Stanza::RegisterGroup { group, admin } => { params!{
                "table" => group_memberships(),
                "group_id" => group, "admin_id" => admin,
            } },
            Stanza::LoginCredentials { user, psw } => { params!{
                "table" => user_interactions(),
                "event" => "login", "user_id" => user, "using_pass" => psw.is_some()
            } },

            Stanza::Request { .. } => return Err(()),
            Stanza::Response { .. } => return Err(()),
            _ => return Err(()),

        }.into();
        Ok(params)
    }

}


