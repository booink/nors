use std::ffi::OsStr;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub enum FileType {
    PlainText,
    CSV,
}

impl FromStr for FileType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "csv" => Self::CSV,
            _ => Self::PlainText,
        })
    }
}

impl FileType {
    pub fn from_path(path: &str) -> Self {
        let ext = Path::new(&path)
            .extension()
            .and_then(OsStr::to_str)
            .or(Some(""))
            .unwrap();
        Self::from_str(ext).unwrap()
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::PlainText
    }
}
