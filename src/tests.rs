extern crate hal;
extern crate postgres;

use hal::Resource;
use postgres::{Connection, NoSsl};
use std::os;
use super::PgsqlHal;

fn get_option(key: &str, default: &str) -> String {
    match os::getenv(key) {
        Some(val) => val,
        None => String::from_str(default)
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
    Connection::connect(dsn.as_slice(), &NoSsl).unwrap()
}

#[test]
// There is no easy way to create a Row object without connecting to the
// database. At least we are not dependent on a table structure.
fn test_row_to_hal() {
    let conn = connect();
    let stmt = conn.prepare("SELECT 1::int as id, 'Jane Doe'::varchar as name").unwrap();

    let mut rows = match stmt.query([]) {
        Ok(rows) => rows,
        Err(err) => panic!("error running query: {}", err)
    };

    let row = match rows.next() {
        Some(row) => row,
        None => panic!("no row found")
    };

    let expected = match PgsqlHal::row_to_hal(stmt.result_descriptions(), &row) {
        PgsqlHal(resource) => resource
    };
    let given = Resource::new()
        .add_state("id", 1i64)
        .add_state("name", "Jane Doe");

    assert_eq!(given, expected);
}