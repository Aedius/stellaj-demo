
use rocket::response::Redirect;
use rocket::http::{Header, ContentType};

use rocket::form::Form;
use rocket::State;
use crate::EventDb;


#[derive(Debug, FromForm)]
pub struct LoginForm<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

// #[derive(Responder)]
// #[response(status = 303, content_type = "json")]
// struct MyResponder {
//     inner: OtherResponder,
//     header: ContentType,
//     more: Header<'static>,
//     #[response(ignore)]
//     unrelated: MyType,
// }

#[post("/login", data= "<login>")]
pub fn login(db_state: &State<EventDb>, login: Form<LoginForm<'_>>) -> Redirect{

    // TODO check password
    if login.username == login.password{


        let mut redirect = Redirect::to("/auth/success");

        return redirect;
    }

    Redirect::to("/")
}

