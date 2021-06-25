use std::collections::HashMap;

use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, CookieJar};
use rocket::form::Form;

use rocket_dyn_templates::Template;


#[derive(FromForm)]
struct Login<'r> {
    username: &'r str,
    password: &'r str
}

#[derive(Debug)]
struct Principal(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Principal {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Principal, Self::Error> {
        request.cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(Principal)
            .or_forward(())
    }
}

#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/session", $crate::session:: $($t)*))
}

pub use session_uri as uri;

use crate::users::User;
use crate::{Context, DbConn};

#[get("/")]
fn index(user: Principal) -> Template {
    let mut template_context = HashMap::new();
    template_context.insert("username", user.0);
    Template::render("session", &template_context)
}

#[get("/", rank = 2)]
fn no_auth_index() -> Redirect {
    Redirect::to(uri!(login_page))
}

#[get("/login")]
fn login(_user: Principal) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/login", rank = 2)]
async fn login_page(flash: Option<FlashMessage<'_>>, conn: DbConn) -> Template {
    let flash = flash.map(FlashMessage::into_inner);
    Template::render("login", Context::raw(&conn, flash).await)
}

#[post("/login", data = "<login>")]
async fn post_login(jar: &CookieJar<'_>, login: Form<Login<'_>>, conn: DbConn) -> Result<Redirect, Flash<Redirect>> {
    //if login.username == "Sergio" && login.password == "password" {
   let successful = User::exists(login.username.to_string(), login.password.to_string(), &conn).await;

    if successful {
        jar.add_private(Cookie::new("username", login.username.to_string()));
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username/password."))
    }
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
    jar.remove_private(Cookie::named("username"));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, no_auth_index, login, login_page, post_login, logout]
}
