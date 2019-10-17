#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin, custom_attribute)]

#[macro_use]
extern crate rocket;
extern crate image;
extern crate log;
extern crate multipart;

mod guards;
mod logger;

use guards::DataImages;

use log::error;
use std::io::Cursor;
use std::{env, fs};

use image::{open, FilterType};
use rocket::http::Status;
use rocket::Rocket;
use rocket::{Request, Response};

#[catch(404)]
fn not_found(request: &Request) -> String {
    format!("Path {} is not a valid path :C", request.uri())
}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
}

pub struct Server;

impl Server {
    pub fn start() {
        rocket().launch();
    }
}

#[post("/api/v1/images/upload", data = "<data>")]
fn upload_image_handler<'a>(data: Result<DataImages, String>) -> Result<&'a str, Response<'static>> {
    let path = env::temp_dir().join("uploaded_files");
    let thum_path = env::temp_dir().join("uploaded_files/thumbnails");

    let d = match data {
        Ok(f) => f,
        Err(err) => {
            return Err(
                Response::build()
                    .status(Status::raw(400))
                    .sized_body(Cursor::new(err))
                    .ok()?,
            )
        }
    };

    fs::create_dir_all(&path).expect("Cannot create dir path for upload images");
    fs::create_dir_all(&thum_path).expect("Cannot create dir for thumbnails");

    for image in d.files {
        let p = path.join(&image.name);

        fs::write(&p, image.buffer).expect("Cannot write file");

        let img = match open(p) {
            Ok(i) => i,
            Err(err) => {
                error!("Error: {:?}", err);

                return Err(
                    Response::build()
                        .status(Status::raw(400))
                        .sized_body(Cursor::new(err.to_string()))
                        .ok()?,
                );
            }
        };

        let thumbnail = img.resize_to_fill(100, 100, FilterType::Triangle);

        thumbnail.save(thum_path.join(&image.name)).unwrap();
    }

    return Ok("Ok");
}