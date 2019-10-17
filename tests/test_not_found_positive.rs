extern crate rust_test_task;

use rocket::http::Status;
use rocket::local::Client;

use rust_test_task::rocket;

#[test]
fn test_not_found() {
  let client = Client::new(rocket()).unwrap();

  {
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
      response.body_string().unwrap(),
      "Path / is not a valid path :C"
    );
  }

  {
    let mut response = client.get("/api/v1/images/upload").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
      response.body_string().unwrap(),
      "Path /api/v1/images/upload is not a valid path :C"
    );
  }

  {
    let mut response = client.put("/api/v1/images/upload").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
      response.body_string().unwrap(),
      "Path /api/v1/images/upload is not a valid path :C"
    );
  }

  {
    let mut response = client.patch("/api/v1/images/upload").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
      response.body_string().unwrap(),
      "Path /api/v1/images/upload is not a valid path :C"
    );
  }

  {
    let mut response = client.post("/api/v1/images/uploads").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
      response.body_string().unwrap(),
      "Path /api/v1/images/uploads is not a valid path :C"
    );
  }
}
