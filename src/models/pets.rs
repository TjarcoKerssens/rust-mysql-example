use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use itertools::Itertools;
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

// Result row of a query to to the owners table
struct OwnerResult {
    pub owner_id: i32,
    pub owner_name: String,
    pub pet_id: i32,
    pub pet_name: String,
    pub pet_type: String,
}

impl OwnerResult {
    fn from_row(row: mysql::Row) -> OwnerResult {
        let (owner_id, owner_name, pet_id, pet_name, pet_type) = mysql::from_row(row);
        OwnerResult {
            owner_id,
            owner_name,
            pet_id,
            pet_name,
            pet_type,
        }
    }
}

impl Owner {
    pub fn all(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<Owner>> {
        let result = conn.query_map(OWNER_QUERY, |row| OwnerResult::from_row(row));

        match result {
            Ok(result) => Ok(Owner::from_result(result)),
            Err(err) => Err(err),
        }
    }

    fn from_result(result: Vec<OwnerResult>) -> Vec<Owner> {
        let mut owners = Vec::new();

        for (key, chunk) in &result.into_iter().chunk_by(|res| res.owner_id) {
            let values: Vec<OwnerResult> = chunk.collect();
            owners.push(Owner {
                id: key,
                name: values[0].owner_name.clone(),
                pets: values
                    .into_iter()
                    .map(|val| Pets {
                        id: val.pet_id,
                        name: val.pet_name,
                        pet_type: PetType {
                            id: 0,
                            name: val.pet_type,
                        },
                    })
                    .collect(),
            });
        }

        owners
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
