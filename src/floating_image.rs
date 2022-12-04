use crate::image_data_error::ImageDataError;
use std::convert::TryInto;

#[derive(Debug)]
pub struct FloatingImage {
  pub width: u32,
  pub height: u32,
  pub data: Vec<u8>,
  pub name: String
}

impl FloatingImage {
  pub fn new(width: u32, height: u32, name: &str) -> Self{
    let buffer_capacity = height * width * 4;
    let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
    FloatingImage {
      width,
      height,
      data: buffer,
      name: name.to_string()
    }
  }

  pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataError> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataError::BufferTooSmall);
    }

    self.data = data;
    Ok(())
  }
}