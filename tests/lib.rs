extern crate hal;
extern crate postgres;
extern crate pgsql_hal;

use hal::resource::Resource;
use postgres::{Connection, SslMode};
use std::env;
use pgsql_hal::pgsql_row_to_hal;

fn get_option(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val.parse().unwrap(),
        Err(..) => default.to_string()
    }
}

fn connect() -> Connection {
    let host = get_option("DBHOST", "localhost");
    let port = get_option("DBPORT", "15432");
    let user = get_option("DBUSER", "myapp");
    let password = get_option("DBPASS", "dbpass");
    let dbname = get_option("DBNAME", "myapp");

    let dsn = format!("postgres://{}:{}@{}:{}/{}", user, password, host,
                      port, dbname);
    Connection::connect(dsn.as_ref(), &SslMode::None).unwrap()
}

#[test]
// There is no easy way to create a Row object without connecting to the
// database. At least we are not dependent on a table structure.
fn test_row_to_hal() {
    let conn = connect();
    let stmt = conn.prepare("SELECT 1::int as id, 'Jane Doe'::varchar as name").unwrap();

    let rows = match stmt.query(&[]) {
        Ok(rows) => rows,
        Err(err) => panic!("error running query: {}", err)
    };

    let row = match rows.iter().next() {
        Some(row) => row,
        None => panic!("no row found")
    };

    let expected = pgsql_row_to_hal(stmt.columns(), &row);

    let mut given = Resource::new();
    given.add_state("id", 1i64)
        .add_state("name", "Jane Doe");

    assert_eq!(given, expected);
}
