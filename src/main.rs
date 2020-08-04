#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::fs::File;
use std::io;
use std::path::Path;

use rocket::http::RawStr;
use rocket::Data;

mod paste_id;

use paste_id::PasteId;

//This is the page users will see when they first visit the service.
#[get("/")]
fn index() {}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, std::io::Error> {
    // Create a new PasteId of a length of your choosing.
    let id = PasteId::new(3);
    // Construct a filename inside upload/ given the PasteId.
    let filename = format!("upload/{}", id);
    let host = "http://localhost:8000".to_string();
    let url = format!("{}////{}", host, id);
    // Stream the Data to the file with the constructed filename.
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
    // Construct a URL given the PasteId.
    // Return the URL to the client.
}

#[get("/<id>")]
fn retrieve(id: &RawStr) -> Option<File> {
    let filename = format!("upload/...{}", id);
    File::open(&filename).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, upload, retrieve])
        .launch();
}
