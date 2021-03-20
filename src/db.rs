use crate::schema::users;
use diesel::{sqlite::SqliteConnection, Connection, QueryDsl, RunQueryDsl};
use dotenv::dotenv;

#[derive(Insertable, Queryable)]
pub struct User {
    pub name: String,
    pub emotion: String,
}
impl From<UsersRecord> for User {
    fn from(record: UsersRecord) -> Self {
        User {
            name: record.name,
            emotion: record.emotion,
        }
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct UsersRecord {
    pub id: i32,
    pub name: String,
    pub emotion: String,
}

pub struct UsersDb {
    connection: SqliteConnection,
}
impl Default for UsersDb {
    fn default() -> Self {
        dotenv().ok();

        let database_url = "sample.db";
        let connection = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Self { connection }
    }
}
impl UsersDb {
    pub fn find_user(&self, user_name: &str) -> Option<UsersRecord> {
        use crate::diesel::ExpressionMethods;
        use crate::schema::users::dsl::*;
        users
            .filter(name.eq(user_name))
            .first::<UsersRecord>(&self.connection)
            .ok()
    }
    pub fn register_user(&self, user: &User) {
        diesel::insert_into(users::table)
            .values(user)
            .execute(&self.connection)
            .expect("Error saving new post");
    }
    pub fn update_user(&self, user: &User) {
        use crate::diesel::ExpressionMethods;
        use crate::schema::users::dsl::*;
        diesel::update(users.filter(name.eq(&user.name.clone())))
            .set(emotion.eq(user.emotion.clone()))
            .execute(&self.connection)
            .expect("Error updating user emotion");
    }
}
