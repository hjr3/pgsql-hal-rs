//! Convert Postgres results into Hal

extern crate hal;
extern crate postgres;

use hal::resource::Resource;
use postgres::{Column, Row, Type};

/// Convert a postgres::Row object into a hal::Resource
pub fn pgsql_row_to_hal(descs: &[Column], row: &Row) -> Resource {
    let mut resource = Resource::new();
    for desc in descs.iter() {
        let column_name: &str = desc.name();

        match desc.type_() {
            &Type::Bool => {
                let value: bool = row.get(column_name);
                resource.add_state(column_name, value);
            },
            &Type::Varchar => {
                let value: String = row.get(column_name);
                resource.add_state(column_name, value);
            },
            &Type::Text => {
                let value: String = row.get(column_name);
                resource.add_state(column_name, value);
            },
            &Type::Int2 => {
                let value: i16 = row.get(column_name);
                resource.add_state(column_name, value as i64);
            },
            &Type::Int4 => {
                let value: i32 = row.get(column_name);
                resource.add_state(column_name, value as i64);
            },
            &Type::Int8 => {
                let value: i64 = row.get(column_name);
                resource.add_state(column_name, value as i64);
            },
            &Type::Float4 => {
                let value: f32 = row.get(column_name);
                resource.add_state(column_name, value as f64);
            },
            &Type::Float8 => {
                let value: f64 = row.get(column_name);
                resource.add_state(column_name, value);
            },
            _ => {
                panic!("unhandled type is: {:?}", desc.type_());
            }
        }
    }

    resource
}
