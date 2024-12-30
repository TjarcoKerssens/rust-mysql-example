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

#[get("/owners/{id}")]
pub(crate) async fn owner_find(
    data: web::Data<mysql::Pool>,
    path: web::Path<u32>,
) -> actix_web::Result<impl Responder> {
    let series = web::block(move || find_owner(&data, path.into_inner())).await??;

    Ok(web::Json(series))
}

fn get_owners(pool: &mysql::Pool) -> Result<Vec<Owner>, PetsDbError> {
    let mut conn = pool.get_conn()?;

    Ok(Owner::all(&mut conn)?)
}

fn find_owner(pool: &mysql::Pool, id: u32) -> Result<Owner, PetsDbError> {
    let mut conn = pool.get_conn()?;

    Ok(Owner::find(&mut conn, id)?)
}
