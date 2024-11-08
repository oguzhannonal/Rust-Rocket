#[macro_use]
extern crate rocket;

use rand::Rng;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::sync::Mutex;

mod models;
use models::{Child, LoginForm, TableRow};

pub struct AppState {
    logged_in: Mutex<bool>,
}

#[get("/")]
fn index(state: &State<AppState>) -> Template {
    let is_logged_in = *state.logged_in.lock().unwrap();
    if !is_logged_in {
        return Template::render("login", context! {});
    }

    let data = generate_random_data();
    Template::render("table", context! { rows: data })
}

#[post("/login", data = "<form>")]
fn login(form: Form<LoginForm>, state: &State<AppState>) -> Redirect {
    if form.username == "admin" && form.password == "password" {
        *state.logged_in.lock().unwrap() = true;
        Redirect::to("/")
    } else {
        Redirect::to("/")
    }
}

#[get("/logout")]
fn logout(state: &State<AppState>) -> Redirect {
    *state.logged_in.lock().unwrap() = false;
    Redirect::to("/")
}

#[get("/api/data")]
fn get_random_data() -> rocket::serde::json::Json<Vec<TableRow>> {
    let data = generate_random_data();
    rocket::serde::json::Json(data)
}

fn generate_random_data() -> Vec<TableRow> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    for i in 0..1000 {
        let num_children = rng.gen_range(0..6);
        let children: Vec<Child> = (0..num_children)
            .map(|j| Child {
                id: j,
                column1: format!("Child {} Data {}", j, rng.gen_range(0..1000)),
                column2: format!("Child {} Data {}", j, rng.gen_range(0..1000)),
                column3: format!("Child {} Data {}", j, rng.gen_range(0..1000)),
                column4: format!("Child {} Data {}", j, rng.gen_range(0..1000)),
                column5: format!("Child {} Data {}", j, rng.gen_range(0..1000)),
                column6: format!("Child {} Data {}", j, rng.gen_range(0..1000)),
            })
            .collect();

        data.push(TableRow {
            id: i,
            column1: format!("Data {}", rng.gen_range(0..1000)),
            column2: format!("Data {}", rng.gen_range(0..1000)),
            column3: format!("Data {}", rng.gen_range(0..1000)),
            column4: format!("Data {}", rng.gen_range(0..1000)),
            column5: format!("Data {}", rng.gen_range(0..1000)),
            column6: format!("Data {}", rng.gen_range(0..1000)),
            children,
        });
    }
    data
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config::figment()
        .merge(("port", 8080))
        .merge(("address", "127.0.0.1"));

    rocket::custom(config)
        .mount("/", routes![index, login, logout, get_random_data])
        .manage(AppState {
            logged_in: Mutex::new(false),
        })
        .attach(Template::fairing())
}
