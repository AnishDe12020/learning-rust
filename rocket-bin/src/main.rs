// #[macro_use] extern crate rocket;
//
// mod paste_id;
//
// use std::io;
//
// use paste_id::PasteId;
//
// use rocket::data::{Data, ToByteUnit};
// use rocket::http::uri::Absolute;
// use rocket::response::content::RawText;
// use rocket::tokio::fs::{self, File};
//
//
// const HOST: Absolute<'static> = uri!("http://localhost:8000");
// const ID_LENGTH: usize = 3;
//
//
// #[post("/", data = "<paste>")]
// async fn upload(paste: Data<'_>) -> io::Result<String> {
//     let id = PasteId::new(ID_LENGTH);
//     paste.open(128.kibibytes()).into_file(id.file_path()).await?;
//     Ok(uri!(HOST, retrieve(id)).to_string())
// }
//
// #[get("/")]
// fn index() -> &'static str {
//     "
//     Hi ser
//     Here is the usage ser:
//         POST `/`
//             accepts raw data in the body of the request and responds with a URL of
//             a page containing the body's content
//
//         GET `/<id>`
//             responds with the content of the paste with the given id
//
//     Hope this is fine ser
//     "
// }
//
// #[get("/<id>")]
// async fn retrieve(id: PasteId<'_>) -> Option<RawText<File>> {
//     File::open(id.file_path()).await.map(RawText).ok()
// }
//
// #[delete("/<id>")]
// async fn delete(id: PasteId<'_>) -> Option<()> {
//     fs::remove_file(id.file_path()).await.ok()
// }
//
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index, upload, retrieve])
// }

#[macro_use] extern crate rocket;

mod paste_id;

use std::io;

use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};

use paste_id::PasteId;

// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:8000");
const ID_LENGTH: usize = 3;

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<RawText<File>> {
    File::open(id.file_path()).await.map(RawText).ok()
}

#[delete("/<id>")]
async fn delete(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXAMPLE: curl --data-binary @file.txt http://localhost:8000

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload, delete, retrieve])
}
