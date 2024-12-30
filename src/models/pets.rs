use actix_web::http::{header::ContentType, StatusCode};
use const_format::formatcp;
use derive_more::{Display, Error, From};
use mysql::prelude::{BinQuery, Queryable, WithParams};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, From)]
pub(crate) enum PetsDbError {
    #[display(fmt = "Internal server error")]
    MysqlError(mysql::Error),
    #[display(fmt = "Owner not found")]
    OwnerNotFound,
}

impl actix_web::ResponseError for PetsDbError {
    fn status_code(&self) -> StatusCode {
        match self {
            PetsDbError::MysqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PetsDbError::OwnerNotFound => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Owner {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) pets: Vec<Pets>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Pets {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) pet_type: PetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PetType {
    pub(crate) id: i32,
    pub(crate) name: String,
}

impl Owner {
    pub(crate) fn all(conn: &mut mysql::PooledConn) -> Result<Vec<Owner>, PetsDbError> {
        let result: Vec<mysql::Row> = conn.query(OWNER_QUERY)?;
        Ok(Self::parse_result(result))
    }

    pub(crate) fn find(conn: &mut mysql::PooledConn, id: u32) -> Result<Owner, PetsDbError> {
        let result = FIND_OWNER.with((id,)).fetch(conn)?;
        let owner = Self::parse_result(result).into_iter().nth(0);
        match owner {
            Some(owner) => Ok(owner),
            None => Err(PetsDbError::OwnerNotFound),
        }
    }

    fn parse_result(result: Vec<mysql::Row>) -> Vec<Owner> {
        result
            .chunk_by(|r1, r2| r1.get::<i32, &str>("owner_id") == r2.get("owner_id"))
            .filter_map(Self::parse_owner)
            .collect()
    }

    fn parse_owner(grouped_rows: &[mysql::Row]) -> Option<Owner> {
        let first_row = &grouped_rows[0];
        Some(Owner {
            id: first_row.get("owner_id")?,
            name: first_row.get("owner_name")?,
            pets: grouped_rows
                .iter()
                .filter_map(|val| {
                    Some(Pets {
                        id: val.get("pet_id")?,
                        name: val.get("pet_name")?,
                        pet_type: PetType {
                            id: 0,
                            name: val.get("pet_type")?,
                        },
                    })
                })
                .collect(),
        })
    }
}

const OWNER_QUERY: &str = "SELECT
    owner.id as owner_id,
    owner.name as owner_name,
    pet.id as pet_id,
    pet.name as pet_name,
    pet_type.name as pet_type
FROM
    owners owner
    JOIN pets pet ON owner.id = pet.owner_id
    JOIN pet_types pet_type ON pet.type_id = pet_type.id
ORDER BY owner_id";

const OWNER_ID_FILTER: &str = "owner.id = ?";

const FIND_OWNER: &str = formatcp!("{OWNER_QUERY} WHERE {OWNER_ID_FILTER}");
