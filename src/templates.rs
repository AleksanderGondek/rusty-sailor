use std::fs::File;
use std::io::Write;
use std::path::Path;

use askama::Template;

use crate::errors::InstallError;

pub fn render_and_save<T: Template>(
  template: T,
  destination_path: &Path
) -> Result<(), InstallError> {
  let rendered_template = template.render()?;
  let mut file = File::create(destination_path)?;
  file.write_all(rendered_template.as_bytes())?;
  Ok(())
}
