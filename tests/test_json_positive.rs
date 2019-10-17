extern crate rust_test_task;

#[cfg(test)]
mod dataset;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use rust_test_task::rocket;

#[test]
fn test_valid_json_png_base64() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client
        .post("/api/v1/images/upload")
        .header(ContentType::JSON)
        .body(format!(r#"["{}"]"#, dataset::get_valid_png_base64()))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_valid_json_bmp_base64() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client
        .post("/api/v1/images/upload")
        .header(ContentType::JSON)
        .body(format!(r#"["{}"]"#, dataset::get_valid_bmp_base64()))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_valid_json_jpg_base64() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client
        .post("/api/v1/images/upload")
        .header(ContentType::JSON)
        .body(format!(r#"["{}"]"#, dataset::get_valid_jpg_base64()))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_valid_json_mixed_base64() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client
        .post("/api/v1/images/upload")
        .header(ContentType::JSON)
        .body(format!(
            r#"["{}", "{}", "{}"]"#,
            dataset::get_valid_jpg_base64(),
            dataset::get_valid_bmp_base64(),
            dataset::get_valid_png_base64()
        ))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}
