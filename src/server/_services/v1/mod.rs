pub mod todo;

pub fn init_v1() -> actix_web::Scope {
    actix_web::web::scope("/v1/todo")
        .service(todo::list::list)
        .service(todo::create::create)
        .service(todo::get::get)
        .service(todo::update::update)
        .service(todo::delete::delete)
}
