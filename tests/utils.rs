use std::fs::File;
use std::io::{Read, Write};

pub struct Form<'a> {
  pub boundary: &'a str,
  data: Vec<u8>,
}

impl<'a> Form<'a> {
  pub fn new() -> Self {
    Form {
      boundary: "-------------573cf973d5228",
      data: vec![],
    }
  }

  pub fn add_text(&mut self, name: &str, text: &str) {
    write!(&mut self.data, "--{}\r\n", self.boundary).unwrap();
    write!(
      &mut self.data,
      "Content-Disposition: form-data; name=\"{}\"\r\n",
      name
    )
    .unwrap();
    write!(&mut self.data, "Content-Type: text/plain\r\n").unwrap();
    write!(&mut self.data, "\r\n").unwrap();
    write!(&mut self.data, "{}\r\n", text).unwrap();
  }
  pub fn add_file(&mut self, name: &str, cont_type: &str, filepath: &str) {
    write!(&mut self.data, "--{}\r\n", self.boundary).unwrap();
    write!(
      &mut self.data,
      "Content-Disposition: form-data; name=\"{}\" filename=\"{}\"\r\n",
      name, filepath
    )
    .unwrap();
    write!(&mut self.data, "Content-Type: {}\r\n", cont_type).unwrap();
    write!(&mut self.data, "\r\n").unwrap();

    let mut file = File::open(filepath).unwrap();
    file.read_to_end(&mut self.data).unwrap();

    write!(&mut self.data, "\r\n").unwrap();
  }
  pub fn get(mut self) -> Vec<u8> {
    write!(&mut self.data, "--{}--\r\n", self.boundary).unwrap();

    self.data
  }
}
