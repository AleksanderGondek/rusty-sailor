use std::borrow::Cow;
use std::convert::From;
use std::io::Cursor;

use rust_embed::RustEmbed;
use tar::Archive;

use crate::errors::{ErrorKind, InstallError};

#[derive(RustEmbed)]
#[folder = "vendored"]
struct Archives;

pub fn unpack_archive(
  name: &str,
  destination: &str
) -> Result<(), InstallError> {
  let archive = Archives::get(name).ok_or_else(||
    InstallError::new(
      ErrorKind::UnpackArchive,
      format!("Archive with name '{}' was not found", name)
    )
  )?;
  match archive {
    Cow::Borrowed(byte_contents) => {
      let result = Archive::new(
        Cursor::new(byte_contents)
      ).unpack(destination)?;
      Ok(result)
    }
    Cow::Owned(_) => {
      Err(
        InstallError::new(
          ErrorKind::UnpackArchive,
          format!("Could not properly read contents of archive '{}'", name)
        )
      )
    }
  }
}
