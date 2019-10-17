extern crate rust_test_task;

#[cfg(test)]
mod dataset;

use std::env;
use std::fs::File;
use std::io::Read;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use rust_test_task::rocket;

#[test]
fn test_invalid_single_bmp_base64() {
    let client = Client::new(rocket()).unwrap();
    let response = client
        .post("/api/v1/images/upload")
        .header(ContentType::Plain)
        .body(dataset::get_invalid_bmp_base64())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_invalid_single_uri() {
    let client = Client::new(rocket()).unwrap();
    let response = client
        .post("/api/v1/images/upload")
        .header(ContentType::Plain)
        .body("https://github.com/ChugunovRoman")
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_unsupported_single_file() {
    let mut body = Vec::new();
    let cur_dir = env::current_dir().unwrap();
    let mut file = File::open(cur_dir.join("tests/dataset/webp.webp").to_str().unwrap()).unwrap();

    file.read_to_end(&mut body).unwrap();

    let client = Client::new(rocket()).unwrap();
    let response = client
        .post("/api/v1/images/upload")
        .header(ContentType::Plain)
        .body(body)
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}
