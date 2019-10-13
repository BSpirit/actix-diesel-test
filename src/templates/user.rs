use askama::Template;
use crate::models::user::User;
use crate::models::product::Product;


#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    pub user: User,
    pub products: Vec<Product>,
}

#[derive(Template)]
#[template(path = "add_user.html")]
pub struct AddUserTemplate {
    pub created: bool,
}