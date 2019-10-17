extern crate base64;
extern crate reqwest;

use std::error::Error;
use std::io::{self, Cursor, Read};
use std::str;

use log::info;
use multipart::server::Multipart;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use serde_json;

use super::utils;

#[derive(Debug)]
pub struct Image {
    pub name: String,
    pub buffer: Vec<u8>,
}

/**
 * Custom Rocker Guard which parse image payload
 */
#[derive(Debug)]
pub struct DataImages {
    pub files: Vec<Image>,
}

impl FromDataSimple for DataImages {
    type Error = String;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let mut files: Vec<Image> = Vec::new();
        let ct = match request.headers().get_one("Content-Type") {
            Some(t) => t,
            None => return Failure((Status::raw(400), format!("Invalid Content-Type"))),
        };

        if ct.starts_with("multipart/form-data") {
            // parse form data with binary, URI or base64 data
            files = match Self::parse_multipart(ct, data) {
                Ok(images) => images,
                Err(err) => {
                    return Failure((Status::raw(400), format!("{:?}", err)));
                }
            };
        } else if ct.starts_with("application/json") {
            // parse JSON
            files = match Self::parse_json(data) {
                Ok(images) => images,
                Err(err) => return Failure((Status::raw(400), format!("{:?}", err))),
            };
        } else if ct.starts_with("text/plain") {
            // parse single URI or base64 data
            let mut buf: Vec<u8> = Vec::new();
            data.stream_to(&mut buf).unwrap();
            let text = match str::from_utf8(&buf) {
                Ok(t) => t,
                Err(_) => return Failure((Status::raw(400), "Invalid text data".to_owned())),
            };

            let image = match Self::parse_text_entry(text) {
                Ok(i) => i,
                Err(err) => return Failure((Status::raw(400), format!("{:?}", err))),
            };

            files.push(image);
        } else if ct.starts_with("image/") {
            // parse single binary data
            let mut image = Image {
                name: "unknown_filename".to_owned(),
                buffer: vec![],
            };

            data.stream_to(&mut image.buffer).unwrap();

            let ext = utils::get_ext_from_bytes(&image.buffer).unwrap();

            image.name = utils::generate_filename(ext);

            files.push(image);
        } else {
            return Failure((Status::raw(400), format!("Invalid Content-Type: {}", ct)));
        }

        data::Outcome::Success(DataImages { files })
    }
}

impl DataImages {
    /**
     * Downloads file from passed URI
     */
    fn from_uri(uri: &str) -> Result<Image, Box<dyn Error>> {
        info!("download image from: {}", uri);

        let mut response = match reqwest::get(uri) {
            Ok(response) => response,
            Err(err) => return Err(Box::new(err)),
        };

        let ct = response.headers()[CONTENT_TYPE].to_str().unwrap();

        if !ct.starts_with("image/") {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot download image. Invalid data",
            )));
        }

        let mut image = Image {
            name: "Unknown_file_name".to_owned(),
            buffer: vec![],
        };
        let mut buffer = Vec::new();
        response.read_to_end(&mut buffer).unwrap();
        let header: &HeaderMap = response.headers();
        let cont_type = header.get("Content-Type").unwrap().to_str().unwrap();
        let ext = match utils::get_ext_by_type(cont_type) {
            Ok(e) => e,
            Err(err) => return Err(Box::new(err)),
        };

        image.name = utils::generate_filename(ext);
        image.buffer = buffer;

        return Ok(image);
    }

    /**
     * Parse base64
     * Accepts string with "data:image/png;base64," and without it
     */
    fn from_base64(data: &str) -> Result<Image, Box<dyn Error>> {
        let mut base64 = data;
        let mut have_metadata: bool = false;
        let mut file_type = "unknown_type";
        let mut image = Image {
            name: "Unknown_file_name".to_owned(),
            buffer: vec![],
        };

        if base64.starts_with("data") {
            let idx = base64.find(",").unwrap() + 1;

            file_type = &base64[5..base64.find(";").unwrap()];
            base64 = &base64[idx..];

            have_metadata = true;
        }

        let buffer = match base64::decode(&base64) {
            Ok(buffer) => buffer,
            Err(err) => return Err(Box::new(err)),
        };

        if !have_metadata {
            file_type = match utils::get_type_from_bytes(&buffer) {
                Ok(ct) => ct,
                Err(err) => return Err(Box::new(err)),
            };
        }

        let ext = match utils::get_ext_by_type(file_type) {
            Ok(e) => e,
            Err(err) => return Err(Box::new(err)),
        };

        image.name = utils::generate_filename(ext);
        image.buffer = buffer;

        return Ok(image);
    }

    /**
     * Parse uri and base64 entry
     */
    fn parse_text_entry(text: &str) -> Result<Image, Box<dyn Error>> {
        if text.starts_with("http") {
            match Self::from_uri(text) {
                Err(err) => return Err(err),
                Ok(buffer) => return Ok(buffer),
            }
        } else {
            match Self::from_base64(text) {
                Ok(buffer) => return Ok(buffer),
                Err(err) => return Err(err),
            }
        }
    }

    fn parse_json(data: Data) -> Result<Vec<Image>, Box<dyn Error>> {
        let mut files: Vec<Image> = Vec::new();
        let mut d = Vec::new();

        data.stream_to(&mut d).expect("Unable to read");

        match serde_json::from_str(&String::from_utf8(d).unwrap()[..]) {
            Ok(json) => {
                let array: Vec<&str> = json;

                for base64 in array {
                    if base64.len() <= 0 {
                        return Err(Box::new(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid base64 string",
                        )));
                    }

                    match Self::from_base64(base64) {
                        Ok(image) => files.push(image),
                        Err(err) => return Err(err),
                    }
                }
            }
            Err(err) => return Err(Box::new(err)),
        }

        Ok(files)
    }

    /**
     * Parse multipart form data.
     * Form can be contain binary data, URI or base64 or all together
     */
    fn parse_multipart(cont_type: &str, data: Data) -> Result<Vec<Image>, Box<dyn Error>> {
        let idx = cont_type
            .find("boundary=")
            .expect("Cannot parse multipart/form-data. boundary not found");
        let boundary = &cont_type[(idx + "boundary=".len())..];
        let mut d = Vec::new();

        data.stream_to(&mut d).expect("Unable to read");

        let mut mp = Multipart::with_body(Cursor::new(d), boundary);
        let mut files: Vec<Image> = Vec::new();

        loop {
            let mut entry = match mp.read_entry() {
                Ok(Some(entry)) => entry,
                Ok(None) => break,
                Err(err) => return Err(Box::new(err)),
            };

            let mut image = Image {
                name: "unknown_filename".to_owned(),
                buffer: vec![],
            };

            if entry.is_text() {
                let mut buf: Vec<u8> = Vec::new();
                entry.data.read_to_end(&mut buf)?;
                let text = str::from_utf8(&buf).unwrap();

                match Self::parse_text_entry(text) {
                    Ok(i) => image = i,
                    Err(err) => return Err(err),
                };
            }

            if let Some(content_type) = entry.headers.content_type {
                if content_type.to_string().starts_with("image/") {
                    entry.data.read_to_end(&mut image.buffer)?;

                    let ext = utils::get_ext_from_bytes(&image.buffer)?;

                    image.name = utils::generate_filename(ext);
                }
            }

            files.push(image);
        }

        Ok(files)
    }
}
