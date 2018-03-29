#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod db;
pub mod schema;
pub mod user;

use user::User;
use dotenv::dotenv;
use rocket::Outcome;
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::{Redirect, content::Html};
use rocket::http::{Cookie, Cookies};
use std::env;

#[get("/login")]
fn login_redirect(_user: User) -> &'static str {
    "You are already logged in"
}

#[get("/login", rank = 2)]
fn login_form() -> Html<&'static str> {
    Html(
        r#"
    <form action="/login" method="post">
    Username:<br><input type="text" name="username">
    <input type="submit"
    </form>
    "#,
    )
}

#[derive(FromForm)]
pub struct LoginData {
    pub username: String,
}

#[post("/login", data = "<login_data>")]
fn login_action(
    login_data: Form<LoginData>,
    mut cookies: Cookies,
    conn: db::Conn,
) -> Result<Redirect, &'static str> {
    let username = &login_data.get().username;

    match User::find_with_username(&username, &conn) {
        Some(user) => {
            cookies.add_private(Cookie::new("username", user.username));
            Ok(Redirect::to("/user"))
        }
        None => Err("Not Found"),
    }
}

#[get("/logout")]
fn logout_action(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("username"));
    Redirect::to("/login")
}

fn get_user_from_request<'a, 'r>(request: &'a Request<'r>, conn: db::Conn) -> Option<User> {
    let username: String = request
        .cookies()
        .get_private("username")?
        .value()
        .parse()
        .ok()?;
    User::find_with_username(&username, &conn)
}

struct AdminUser(User);
impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let conn = request.guard::<db::Conn>()?;

        let user = get_user_from_request(request, conn);

        if let Some(user @ User { is_admin: true, .. }) = user {
            Outcome::Success(AdminUser(user))
        } else {
            Outcome::Forward(())
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let conn = request.guard::<db::Conn>()?;

        let user = get_user_from_request(request, conn);

        if let Some(user) = user {
            Outcome::Success(user)
        } else {
            Outcome::Forward(())
        }
    }
}

#[get("/user")]
fn admin_index(user: AdminUser) -> String {
    format!("Hey {}, you are an admin", user.0.username)
}

#[get("/user", rank = 2)]
fn user_index(user: User) -> String {
    format!("Sorry {}, you are not an admin", user.username)
}

#[get("/user", rank = 3)]
fn index() -> &'static str {
    "You are not logged in"
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::init_pool(&database_url);

    rocket::ignite()
        .manage(pool)
        .mount(
            "/",
            routes![
                login_redirect,
                login_form,
                login_action,
                logout_action,
                admin_index,
                user_index,
                index
            ],
        )
        .launch();
}
