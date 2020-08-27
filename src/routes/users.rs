use crate::util::read_from_static;
use actix_web::{web, HttpRequest, HttpResponse, Scope};

/// Returns hardcoded data for a request to `https://api.github.com/users/{user}/events`
fn fake_users_events(_: HttpRequest) -> HttpResponse {
    match read_from_static("users/events.json") {
        Ok(data) => HttpResponse::from(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn users_router() -> Scope {
    Scope::new("/users").route("/{user}/events", web::get().to(fake_users_events))
}
