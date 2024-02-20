
// https://docs.google.com/presentation/d/1sVCh5_5vMPs0ZjHKyDLyv_ZWGos4eXY07mVJ6slW1tg/edit#slide=id.g1f205c34dc6_0_0

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
