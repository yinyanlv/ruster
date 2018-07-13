use actix::*;
use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager };
// use dotenv;

// pub type DBPool = Pool<ConnectionManager<PgConnection>>;

// pub fn init_pool() -> DBPool {
//     let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let manager = ConnectionManager::<PgConnection>::new(db_url);
//     Pool::builder().build(manager).expect("Failed to create pool.")
// }

pub struct ConnDsl(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}