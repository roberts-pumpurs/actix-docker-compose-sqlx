#!/bin/bash
# While developing, we want to keep the database state up-to date at all times

# Kill previous instance of script
killall -s 9 actix-mysql-sql
cargo sqlx prepare
cargo run
