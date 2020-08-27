use crate::util::read_from_static;
use actix_web::{web, HttpRequest, HttpResponse, Scope};

/// Returns hardcoded data for a request to `https://api.github.com/orgs/{org}/repos`
fn fake_orgs_repos(_: HttpRequest) -> HttpResponse {
    match read_from_static("orgs/repos.json") {
        Ok(data) => HttpResponse::from(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn orgs_router() -> Scope {
    Scope::new("/orgs").route("/{org}/repos", web::get().to(fake_orgs_repos))
}
