#!/bin/sh

# Set the database URL environment variable
export DATABASE_URL=$DATABASE_URL

export TZ=$TZ

# Run SQLx migrations
/usr/local/bin/sqlx db create
echo Created database

# Run SQLx migrations
/usr/local/bin/sqlx migrate run
echo Migrated database

# Run your application with the provided arguments
echo Running
./project-tracker
echo Stopped
