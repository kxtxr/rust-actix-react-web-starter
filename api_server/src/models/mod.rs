use diesel::{r2d2::ConnectionManager, MysqlConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub mod blog;
pub mod invitation;
pub mod user;
