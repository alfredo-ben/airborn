#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::*;
use rocket_dyn_templates::*;
use rocket::request::Form;
use rocket::response::content::Json;
use rocket::Request;


#[derive(FromForm, Debug)]
struct Book {
  title: String,
  author: String,
  isbn: String
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
  let book: Book = book_form.into_inner();
  let mut dummy_db: Vec<Book> = Vec::new();
  dummy_db.push(book);
  format!("Book added successfully: {:?}", dummy_db)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
  rocket::ignite()
  .register(catchers![not_found])
  .mount("/api", routes![hello, new_book])
  .launch();
}
