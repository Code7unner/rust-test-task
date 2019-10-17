extern crate rust_test_task;

#[cfg(test)]
mod dataset;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use rust_test_task::rocket;

#[test]
fn test_invalid_json_png_base64() {
  let client = Client::new(rocket()).unwrap();
  let response = client
    .post("/api/v1/images/upload")
    .header(ContentType::JSON)
    .body(format!(r#"["{}"]"#, dataset::get_invalid_png_base64()))
    .dispatch();

  assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_invalid_json_bmp_base64() {
  let client = Client::new(rocket()).unwrap();
  let response = client
    .post("/api/v1/images/upload")
    .header(ContentType::JSON)
    .body(format!(r#"["{}"]"#, dataset::get_invalid_bmp_base64()))
    .dispatch();

  assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_invalid_json_jpg_base64() {
  let client = Client::new(rocket()).unwrap();
  let response = client
    .post("/api/v1/images/upload")
    .header(ContentType::JSON)
    .body(format!(r#"["{}"]"#, dataset::get_invalid_jpg_base64()))
    .dispatch();

  assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_invalid_json_mixed_base64() {
  let client = Client::new(rocket()).unwrap();
  let response = client
    .post("/api/v1/images/upload")
    .header(ContentType::JSON)
    .body(format!(
      r#"["{}", "{}", "{}"]"#,
      dataset::get_invalid_jpg_base64(),
      dataset::get_invalid_bmp_base64(),
      dataset::get_invalid_png_base64()
    ))
    .dispatch();

  assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_empty_json_mixed_base64() {
  let client = Client::new(rocket()).unwrap();
  let response = client
    .post("/api/v1/images/upload")
    .header(ContentType::JSON)
    .body(r#"["", "", ""]"#)
    .dispatch();

  assert_eq!(response.status(), Status::BadRequest);
}
