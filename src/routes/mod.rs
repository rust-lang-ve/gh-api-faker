use actix_web::Scope;

pub mod orgs;
pub mod users;

pub fn router() -> Scope {
    Scope::new("/")
        .service(users::users_router())
        .service(orgs::orgs_router())
}
