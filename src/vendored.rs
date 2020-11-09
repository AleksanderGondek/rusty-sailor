use std::borrow::Cow;
use std::convert::From;
use std::io::Cursor;

use flate2::read::GzDecoder;
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
  let package_bytes = Archives::get(name).ok_or_else(||
    InstallError::new(
      ErrorKind::UnpackArchive,
      format!("Archive with name '{}' was not found", name)
    )
  )?;
  match package_bytes {
    Cow::Borrowed(bytes) => {
      let tar = GzDecoder::new(
        Cursor::new(bytes)
      );
      let mut archive = Archive::new(
        tar
      );
      let result = archive.unpack(
        destination
      )?;
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
