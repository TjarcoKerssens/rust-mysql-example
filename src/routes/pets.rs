use crate::models::{pets::Owner, PetsDbError};
use actix_web::{get, web, Responder};

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/owners")]
pub(crate) async fn owners(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let series = web::block(move || get_owners(&data)).await??;

    Ok(web::Json(series))
}

fn get_owners(pool: &mysql::Pool) -> Result<Vec<Owner>, PetsDbError> {
    let mut conn = pool.get_conn()?;

    Ok(Owner::all(&mut conn)?)
}
