#[derive(Debug)]
pub enum ImageDataError {
  DifferentImageFormats,
  BufferTooSmall
}