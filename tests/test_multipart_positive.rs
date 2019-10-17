extern crate rust_test_task;

#[cfg(test)]
mod dataset;
#[cfg(test)]
mod utils;

use std::env;

use rocket::http::{Header, Status};
use rocket::local::Client;

use rust_test_task::rocket;

#[test]
fn test_url() {
  let mut form = utils::Form::new();
  form.add_text(
    "image[]",
    "https://cdn.pixabay.com/photo/2018/01/14/23/12/nature-3082832_960_720.jpg",
  );

  let client = Client::new(rocket()).unwrap();
  let mut response = client
    .post("/api/v1/images/upload")
    .header(Header::new(
      "Content-Type",
      format!("multipart/form-data; boundary={}", form.boundary),
    ))
    .body(form.get())
    .dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_urls() {
  let mut form = utils::Form::new();
  form.add_text(
    "image[]",
    "https://cdn.pixabay.com/photo/2018/01/14/23/12/nature-3082832_960_720.jpg",
  );
  form.add_text(
    "image[]",
    "https://cdn.pixabay.com/photo/2019/09/04/02/52/road-4450611_960_720.jpg",
  );

  let client = Client::new(rocket()).unwrap();
  let mut response = client
    .post("/api/v1/images/upload")
    .header(Header::new(
      "Content-Type",
      format!("multipart/form-data; boundary={}", form.boundary),
    ))
    .body(form.get())
    .dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_base64() {
  let mut form = utils::Form::new();
  form.add_text("image[]", dataset::get_valid_png_base64());
  form.add_text("image[]", dataset::get_valid_jpg_base64());

  let client = Client::new(rocket()).unwrap();
  let mut response = client
    .post("/api/v1/images/upload")
    .header(Header::new(
      "Content-Type",
      format!("multipart/form-data; boundary={}", form.boundary),
    ))
    .body(form.get())
    .dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_binary() {
  let cur_dir = env::current_dir().unwrap();
  let mut form = utils::Form::new();

  form.add_text(
    "image[]",
    "https://cdn.pixabay.com/photo/2018/01/14/23/12/nature-3082832_960_720.jpg",
  );
  form.add_file(
    "image[]",
    "image/tiff",
    cur_dir.join("tests/dataset/tiff.tiff").to_str().unwrap(),
  );
  form.add_file(
    "image[]",
    "image/jpeg",
    cur_dir.join("tests/dataset/jpg.jpg").to_str().unwrap(),
  );
  form.add_file(
    "image[]",
    "image/png",
    cur_dir.join("tests/dataset/png.png").to_str().unwrap(),
  );
  form.add_text(
    "image[]",
    "https://cdn.pixabay.com/photo/2019/09/04/02/52/road-4450611_960_720.jpg",
  );
  form.add_file(
    "image[]",
    "image/vnd.microsoft.icon",
    cur_dir.join("tests/dataset/ico.ico").to_str().unwrap(),
  );
  form.add_file(
    "image[]",
    "image/vnd.wap.wbmp",
    cur_dir.join("tests/dataset/bmp.bmp").to_str().unwrap(),
  );

  let client = Client::new(rocket()).unwrap();
  let mut response = client
    .post("/api/v1/images/upload")
    .header(Header::new(
      "Content-Type",
      format!("multipart/form-data; boundary={}", form.boundary),
    ))
    .body(form.get())
    .dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}

#[test]
fn test_mixed() {
  let cur_dir = env::current_dir().unwrap();
  let mut form = utils::Form::new();
  form.add_file(
    "image[]",
    "image/tiff",
    cur_dir.join("tests/dataset/tiff.tiff").to_str().unwrap(),
  );
  form.add_file(
    "image[]",
    "image/jpeg",
    cur_dir.join("tests/dataset/jpg.jpg").to_str().unwrap(),
  );
  form.add_file(
    "image[]",
    "image/png",
    cur_dir.join("tests/dataset/png.png").to_str().unwrap(),
  );
  form.add_file(
    "image[]",
    "image/vnd.microsoft.icon",
    cur_dir.join("tests/dataset/ico.ico").to_str().unwrap(),
  );
  form.add_file(
    "image[]",
    "image/vnd.wap.wbmp",
    cur_dir.join("tests/dataset/bmp.bmp").to_str().unwrap(),
  );
  form.add_text("image[]", dataset::get_valid_jpg_base64());

  let client = Client::new(rocket()).unwrap();
  let mut response = client
    .post("/api/v1/images/upload")
    .header(Header::new(
      "Content-Type",
      format!("multipart/form-data; boundary={}", form.boundary),
    ))
    .body(form.get())
    .dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string().unwrap(), "Ok".to_owned());
}
