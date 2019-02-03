#![feature(try_from, nll)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate ruma_identifiers;

use std::convert::TryFrom;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use ruma_identifiers::UserId;

table! {
    tests (user_id, other_id) {
        user_id -> Text,
        other_id -> Text,
    }
}

#[derive(Associations, Debug, Identifiable, Queryable)]
#[primary_key(user_id, other_id)]
#[table_name = "tests"]
pub struct TestEntry {
    pub user_id: UserId,
    pub other_id: String,
}

#[derive(Insertable)]
#[table_name = "tests"]
pub struct NewTestEntry<'a> {
    pub user_id: &'a UserId,
    pub other_id: &'a str,
}

embed_migrations!();

fn main() {
    let connection = SqliteConnection::establish("./test.sqlite3").unwrap();
    embedded_migrations::run(&connection).unwrap();

    let user_id = UserId::try_from("@test:example.com").unwrap();
    let other_id = "foo";
    let new_test_entry = NewTestEntry {
        user_id: &user_id,
        other_id: other_id,
    };

    diesel::insert_into(tests::table)
        .values(&new_test_entry)
        .execute(&connection)
        .unwrap();

    let test_entry: TestEntry = tests::table
        .find((user_id, other_id))
        .first(&connection)
        .unwrap();

    println!("{:?}", test_entry);
}
