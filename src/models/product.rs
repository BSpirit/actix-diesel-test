use crate::schema::products;
use crate::models::user::User;
use diesel::prelude::*;


#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub user_id: Option<i32>
}

#[derive(Insertable, AsChangeset)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub user_id: Option<i32>
}

impl<'a> NewProduct<'a> {
    pub fn create(&self, connection: &SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(products::table)
            .values(self)
            .execute(connection)
    }
}

impl Product {
    pub fn get(id: &i32, connection: &SqliteConnection) -> Result<Product, diesel::result::Error> {
        products::table.find(id).first(connection)
    }

    pub fn delete(id: &i32, connection: &SqliteConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::products::dsl;
        diesel::delete(dsl::products.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_product: &NewProduct, connection: &SqliteConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::products::dsl;
        diesel::update(dsl::products.find(id))
            .set(new_product)
            .execute(connection)?;
        Ok(())
    }

    pub fn get_all(conn: &SqliteConnection) -> Result<Vec<Product>, diesel::result::Error> {
        products::table.load::<Product>(conn)
    }
}