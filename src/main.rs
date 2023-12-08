#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    let redirect = Redirect::to("https://summit-lsp.lsst.codes/rolex");

    redirect
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
