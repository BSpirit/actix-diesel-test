use crate::schema::users;
use diesel::prelude::*;


#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn create(&self, connection: &SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(self)
            .execute(connection)
    }
}

impl User {
    pub fn get(id: &i32, connection: &SqliteConnection) -> Result<User, diesel::result::Error> {
        users::table.find(id).first(connection)
    }

    pub fn delete(id: &i32, connection: &SqliteConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl;
        diesel::delete(dsl::users.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_user: &NewUser, connection: &SqliteConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl;
        diesel::update(dsl::users.find(id))
            .set(new_user)
            .execute(connection)?;
        Ok(())
    }

    pub fn get_all(conn: &SqliteConnection) -> Result<Vec<User>, diesel::result::Error> {
        users::table.load::<User>(conn)
    }
}
