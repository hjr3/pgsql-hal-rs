//! Convert Postgres results into Hal

extern crate hal;
extern crate postgres;

use hal::Resource;
use postgres::{ResultDescription, Row, Type};

#[cfg(test)]
mod tests;

pub struct PgsqlHal(Resource);

impl PgsqlHal {

    /// Convert a postgres::Row object into a hal::Resource
    pub fn row_to_hal(descs: &[ResultDescription], row: &Row) -> PgsqlHal {
        let mut hal = Resource::new();
        for desc in descs.iter() {
            let column_name = desc.name.as_slice();

            match desc.ty {
                Type::Varchar => {
                    let value: String = row.get(column_name);
                    hal = hal.add_state(column_name, value);
                },
                Type::Int4 => {
                    let value: i32 = row.get(column_name);
                    hal = hal.add_state(column_name, value as i64);
                },
                Type::Float8 => {
                    let value: f64 = row.get(column_name);
                    hal = hal.add_state(column_name, value);
                },
                _ => {
                    panic!("unhandled type is: {}", desc.ty);
                }
            }
        }

        PgsqlHal(hal)
    }
}
