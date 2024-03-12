#!/bin/sh

export DATABASE_URL=mysql://$DB_USERNAME:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME

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
