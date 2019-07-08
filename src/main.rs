#![feature(proc_macro_hygiene, decl_macro)]

use nix::sys::statvfs::statvfs;
use rocket::get;
use rocket::routes;


fn available_space() -> Result<f64, nix::Error> {
    let stats = statvfs("/")?;
    Ok(stats.blocks_available() as f64 / stats.blocks() as f64)
}

#[get("/df")]
fn index() -> String {
    available_space().unwrap().to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

