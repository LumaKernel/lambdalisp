use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    /// 0-based line number
    pub line: usize,
    /// 0-based column number
    pub col: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    /// range from inclusive
    pub from: Location,
    /// range to inclusive
    pub to: Location,
}
#[derive(Clone, Debug)]
pub struct FileInfo {
    /// filepath
    pub filepath: String,
    pub range: Range,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {} col {}", self.line + 1, self.col + 1)
    }
}
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.from.fmt(f)?;
        if self.from != self.to {
            write!(f, "-")?;
            self.to.fmt(f)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.filepath)?;
        self.range.fmt(f)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CompileError {
    pub info: Option<FileInfo>,
    pub message: Option<String>,
}
impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.info {
            Some(info) => info.fmt(f)?,
            None => write!(f, "<no file>")?,
        };
        write!(f, ": ")?;
        match &self.message {
            Some(message) => write!(f, "{}", message)?,
            None => write!(f, "<no message>")?,
        };
        Ok(())
    }
}
impl std::error::Error for CompileError {}
