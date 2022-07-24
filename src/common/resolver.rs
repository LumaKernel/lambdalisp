use std::path::PathBuf;

pub struct ContentResolveResult {
    pub content: String,
    pub filepath: Option<PathBuf>,
}

pub trait ContentResolver {
    /// returns (content, filepath)
    fn resolve(
        &mut self,
        from_path: &Option<PathBuf>,
        to_resolve: &String,
    ) -> Result<ContentResolveResult, String>;
}
