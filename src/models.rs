use crate::schema::users;
use diesel::prelude::*;
use std::vec::Vec;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn create(&self, conn: &SqliteConnection) -> usize {
        diesel::insert_into(users::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new user")
    }
}

impl User {
    pub fn list(conn: &SqliteConnection) -> Vec<User> {
        users::table.load::<User>(conn)
                    .expect("Error loading users")
    }
}

