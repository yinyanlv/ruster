use actix_web::{fs::NamedFile, HttpRequest, Error, Result};
use router::AppState;

pub fn index(_req: &HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

pub fn path(_req: &HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

