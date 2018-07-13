use actix::{ Addr, Syn};
use actix_web::{fs::NamedFile, HttpRequest, Error, Result};
use model::db::ConnDsl;

pub struct AppState {
    pub db: Addr<Syn, ConnDsl>
}

pub fn home(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

pub fn path(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

