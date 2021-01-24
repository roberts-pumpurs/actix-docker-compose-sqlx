#!/bin/bash
# While developing, we want to keep the database state up-to date at all times
cargo sqlx prepare
cargo run
