extern crate rust_test_task;

#[cfg(test)]
mod dataset;
#[cfg(test)]
mod utils;

use std::env;

use rocket::http::{ContentType, Header, Status};
use rocket::local::Client;

use rust_test_task::rocket;

#[test]
fn test_invalid_url() {
    let mut form = utils::Form::new();
    form.add_text("image[]", "https://github.com/ChugunovRoman");

    let client = Client::new(rocket()).unwrap();
    let response = client
        .post("/api/v1/images/upload")
        .header(Header::new(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary),
        ))
        .body(form.get())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_invalid_base64() {
    let mut form = utils::Form::new();
    form.add_text("image[]", dataset::get_invalid_png_base64());
    form.add_text("image[]", dataset::get_invalid_jpg_base64());

    let client = Client::new(rocket()).unwrap();
    let response = client
        .post("/api/v1/images/upload")
        .header(Header::new(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary),
        ))
        .body(form.get())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_unsupported_binary() {
    let cur_dir = env::current_dir().unwrap();
    let mut form = utils::Form::new();

    form.add_file(
        "image[]",
        "image/webp",
        cur_dir.join("tests/dataset/webp.webp").to_str().unwrap(),
    );

    let client = Client::new(rocket()).unwrap();
    let response = client
        .post("/api/v1/images/upload")
        .header(Header::new(
            "Content-Type",
            format!("multipart/form-data; boundary={}", form.boundary),
        ))
        .body(form.get())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}
