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
fn test_valid_single_png_base64() {
  let client = Client::new(rocket()).unwrap();
  let mut response = client
    .post("/api/v1/images/upload")
    .header(ContentType::Plain)
    .body(dataset::get_valid_png_base64())
    .dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_valid_single_uri() {
  let client = Client::new(rocket()).unwrap();
  let mut response = client
    .post("/api/v1/images/upload")
    .header(ContentType::Plain)
    .body("https://cdn.pixabay.com/photo/2018/01/14/23/12/nature-3082832_960_720.jpg")
    .dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_valid_single_file() {
  let mut body = Vec::new();
  let cur_dir = env::current_dir().unwrap();
  let mut file = File::open(cur_dir.join("tests/dataset/png.png").to_str().unwrap()).unwrap();

  file.read_to_end(&mut body).unwrap();

  let client = Client::new(rocket()).unwrap();
  let mut response = client
    .post("/api/v1/images/upload")
    .header(ContentType::parse_flexible("image/png").unwrap())
    .body(body)
    .dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}
