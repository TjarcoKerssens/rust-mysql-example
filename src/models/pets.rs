use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, From)]
pub enum PetsDbError {
    MysqlError(mysql::Error),
    Unknown,
}

impl actix_web::ResponseError for PetsDbError {
    fn status_code(&self) -> StatusCode {
        match self {
            PetsDbError::MysqlError(_) | PetsDbError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub id: i32,
    pub name: String,
    pub pets: Vec<Pets>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pets {
    pub id: i32,
    pub name: String,
    pub pet_type: PetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetType {
    pub id: i32,
    pub name: String,
}

impl Owner {
    pub fn all(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<Owner>> {
        let result: Vec<mysql::Row> = conn.query(OWNER_QUERY)?;
        Ok(Self::parse_result(result))
    }

    fn parse_result(result: Vec<mysql::Row>) -> Vec<Owner> {
        result
            .chunk_by(|r1, r2| r1.get::<i32, &str>("owner_id") == r2.get("owner_id"))
            .into_iter()
            .map(Self::parse_owner)
            .collect()
    }

    fn parse_owner(grouped_rows: &[mysql::Row]) -> Owner {
        let first_row = &grouped_rows[0];
        Owner {
            id: first_row.get("owner_id").unwrap(),
            name: first_row.get("owner_name").unwrap(),
            pets: grouped_rows
                .iter()
                .map(|val| Pets {
                    id: val.get("pet_id").unwrap(),
                    name: val.get("pet_name").unwrap(),
                    pet_type: PetType {
                        id: 0,
                        name: val.get("pet_type").unwrap(),
                    },
                })
                .collect(),
        }
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
    JOIN pet_types pet_type ON pet.type_id = pet_type.id;";
