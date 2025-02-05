#!/bin/sh -e

sqlx database create
./src/scripts/migrations/run
