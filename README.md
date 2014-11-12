# Rust Postgres to Hal helper

A helper to convert Postgres results into `hal::Resource` objects.

[![Build Status](https://travis-ci.org/hjr3/pgsql-hal-rs.svg)](https://travis-ci.org/hjr3/pgsql-hal-rs)

## Use

Add this library to `Cargo.toml`:

```toml
[dependencies.pgsql-hal]

git = "https://github.com/hjr3/pgsql-hal-rs.git"
```

## Build Instructions

The tests require a postgres database server. The postgres database server
credentials can be customized using the following environment variable:

   * DBHOST
   * DBPORT
   * DBUSER
   * DBPASS
   * DBNAME

```
$ git clone https://github.com/hjr3/pgsql-hal-rs
$ cd pgsql-hal-rs
$ cargo test
```
