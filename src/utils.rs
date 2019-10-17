extern crate chrono;
extern crate uuid;

use std::io::{Error, ErrorKind};

use chrono::prelude::*;
use image::{guess_format, ImageFormat};
use log::warn;
use uuid::Uuid;

/**
 * Get file extension by content-type string
 */
pub fn get_ext_by_type(cont_type: &str) -> Result<&str, Error> {
    match &cont_type[..] {
        "image/gif" => Ok("gif"),
        "image/jpeg" => Ok("jpg"),
        "image/pjpeg" => Ok("jpeg"),
        "image/png" => Ok("png"),
        "image/svg+xml" => Ok("svg"),
        "image/tiff" => Ok("tif"),
        "image/vnd.microsoft.icon" => Ok("ico"),
        "image/vnd.wap.wbmp" => Ok("bmp"),
        "image/webp" => Ok("webp"),
        _ => {
            warn!(
                "Unknown image mime type: {}. Will be returned empty string",
                cont_type
            );

            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unknown image mime type: {}", cont_type),
            ));
        }
    }
}

/**
 * Generate unique file name
 */
pub fn generate_filename(ext: &str) -> String {
    let uuid = Uuid::new_v4();
    let datetime = Utc::now().format("%Y-%m-%d_%H:%M:%S");

    format!("image_{}_{}.{}", uuid, datetime, ext)
}

/**
 * Get file content type from four first bytes
 */
pub fn get_type_from_bytes(bytes: &[u8]) -> Result<&str, Error> {
    match guess_format(bytes).unwrap() {
        ImageFormat::PNG => Ok("image/png"),
        ImageFormat::JPEG => Ok("image/jpeg"),
        ImageFormat::TIFF => Ok("image/tiff"),
        ImageFormat::BMP => Ok("image/vnd.wap.wbmp"),
        ImageFormat::ICO => Ok("image/vnd.microsoft.icon"),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            "Cannot determine type of image. Found unsupported image.",
        )),
    }
}

/**
 * Get file content type from four first bytes
 */
pub fn get_ext_from_bytes(bytes: &[u8]) -> Result<&str, Error> {
    match guess_format(bytes).unwrap() {
        ImageFormat::PNG => Ok("png"),
        ImageFormat::JPEG => Ok("jpeg"),
        ImageFormat::TIFF => Ok("tif"),
        ImageFormat::BMP => Ok("bmp"),
        ImageFormat::ICO => Ok("ico"),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            "Cannot determine type of image. Found unsupported image.",
        )),
    }
}