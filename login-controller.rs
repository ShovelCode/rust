#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::Form;
use rocket::response::Redirect;

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login", data = "<login_form>")]
fn login(login_form: Form<LoginForm>) -> Redirect {
    let username = login_form.username.clone();
    let password = login_form.password.clone();

    // Perform authentication logic here
    if authenticate(&username, &password) {
        // Successful login, redirect to a protected page
        Redirect::to("/dashboard")
    } else {
        // Invalid credentials, redirect back to the login page
        Redirect::to("/login")
    }
}

fn authenticate(username: &str, password: &str) -> bool {
    // Perform actual authentication check against a database or other storage
    // Return true if the credentials are valid, false otherwise
    // This is a placeholder implementation, you should replace it with your own authentication logic
    username == "admin" && password == "password"
}

fn main() {
    rocket::ignite().mount("/", routes![login]).launch();
}
