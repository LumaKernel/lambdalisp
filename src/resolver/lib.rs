use crate::common::resolver::{ContentResolveResult, ContentResolver};
use rust_embed::RustEmbed;
use std::path::PathBuf;
use std::str;

#[derive(RustEmbed)]
#[folder = "lisp-lib/"]
struct LispLib;

pub struct LibResolver {}
impl Default for LibResolver {
    fn default() -> Self {
        Self {}
    }
}
impl ContentResolver for LibResolver {
    fn resolve(
        &mut self,
        _from_path: &Option<PathBuf>,
        to_resolve: &String,
    ) -> Result<ContentResolveResult, String> {
        match LispLib::get(&(to_resolve.as_str().to_owned() + ".lisp")) {
            Some(file) => Ok(match str::from_utf8(&file.data.into_owned()) {
                Ok(c) => ContentResolveResult {
                    content: c.into(),
                    filepath: None,
                },
                Err(..) => Err(format!("could not read \"{}\" as UTF-8", to_resolve))?,
            }),
            None => Err(format!("not found built-in lib \"{}\"", to_resolve)),
        }
    }
}
