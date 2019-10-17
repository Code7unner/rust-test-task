#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate curl;

use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct Image {
    links: Vec<String>,
}

#[post("/image", format = "json", data = "<image>")]
fn image_handler(image: Json<Image>) -> JsonValue {
    json!({
        "result": image.links
    })
}

fn main() {
    rocket::ignite().mount("/", routes![image_handler]).launch();
}
