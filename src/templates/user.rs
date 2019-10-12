use askama::Template;
use crate::models::user::User;


#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<User>,
}