use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::{copy, read_dir, read_link, remove_dir, remove_file};
use std::io;
use std::path::Path;

// Rename will not work across different mount-points
pub fn mv(
  file_path: &Path,
  dest_dir: &Path  
) -> io::Result<()> {
  let filename = file_name(file_path);
  let dest_path = dest_dir.join(
    filename?
  );
  copy(file_path, dest_path)?;
  remove_file(file_path)
}

pub fn flatten(
  path: &Path,
  file_name_whitelist: Option<&HashSet<OsString>>
) -> io::Result<()> {
  let cannonical_path = path.canonicalize()?;
  let cannonical_path = cannonical_path.as_path();
  _flatten(&cannonical_path, &cannonical_path, file_name_whitelist)
}

fn file_name(
  file_path: &Path
) -> io::Result<OsString> {
  file_path.file_name().map_or_else(
    || Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!(
        "Could not procure the name for file '{}'",
        file_path.display()
      )
    )),
    |name| Ok(name.to_os_string())
  )
}

fn _flatten(
  from: &Path,
  to: &Path,
  file_name_whitelist: Option<&HashSet<OsString>>
) -> io::Result<()> {
  // If destination is a file
  if to.is_file() {
    return Err(
      io::Error::new(
        io::ErrorKind::InvalidInput,
        "Destination path is not a directory"
      )
    )
  }

  // For a single file
  if from.is_file() {
    // If file is already in destination
    let is_already_in_place: bool = from.parent().map_or_else(
      || false,
      |parent_name| parent_name == to
    );
    if is_already_in_place {
      return Ok(());
    }
    
    // Whitelist handling
    return match file_name_whitelist {
      Some(whitelist) => {
        let filename = file_name(from)?;
        if whitelist.contains(&filename) {
          mv(from, to)
        }
        else {
          remove_file(from)
        }
      },
      None => mv(from, to)
    }
  }
  // For a symlink
  let is_symlink = read_link(from).is_ok();
  if is_symlink {
    return remove_file(from)
  }

  // For a directory
  for file in read_dir(from)? {
    _flatten(&file?.path(), to, file_name_whitelist)?;
  }

  // Cleanup dir
  if from != to {
    remove_dir(from)?;
  }
  Ok(())
}
