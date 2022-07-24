use super::lib::LibResolver;
use crate::common::resolver::{ContentResolveResult, ContentResolver};
use std::env::current_dir;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct FsResolver {}
impl Default for FsResolver {
    fn default() -> Self {
        Self {}
    }
}
impl ContentResolver for FsResolver {
    fn resolve(
        &mut self,
        from_path: &Option<PathBuf>,
        to_resolve: &String,
    ) -> Result<ContentResolveResult, String> {
        if to_resolve.chars().nth(0) == Some('.') {
            let from_path_defaulted = match from_path
                .clone()
                .map(|p| io::Result::Ok(PathBuf::from(p)))
                .unwrap_or_else(|| current_dir())
            {
                Ok(c) => c,
                Err(..) => Err(format!("could not get current directly"))?,
            };
            let abs = match from_path_defaulted
                .join(PathBuf::from(to_resolve))
                .canonicalize()
            {
                Ok(abs) => abs,
                Err(..) => Err(format!("could not canonicalize \"{}\"", to_resolve))?,
            };
            let mut file = match File::open(&abs) {
                Ok(file) => file,
                Err(..) => Err(format!("could not open \"{}\"", to_resolve))?,
            };
            let mut buf = String::new();
            if let Err(..) = file.read_to_string(&mut buf) {
                return Err(format!("could not read \"{}\" as UTF-8", to_resolve));
            };
            Ok(ContentResolveResult {
                content: buf,
                filepath: Some(abs),
            })
        } else {
            let mut lib_resolver = LibResolver::default();
            lib_resolver.resolve(from_path, to_resolve)
        }
    }
}
