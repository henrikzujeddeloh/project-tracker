# Project Tracker
A simple project tracker web app written entirely in Rust.

## Usage
To use app, just clone repository and use docker-compose.
The docker-compose.yml file contains a mariadb database server.
Make sure correct environment variables are set.
```bash
docker compose up --build
```

## Development
The following is if changes to the app are made.

### Prerequisites
- Rust
- sqlx-cli
- Node.js
- npm
- existing mariadb database server (for development)
- docker (for generating the production image)
- npm packages:
```bash
npm install -g tailwindcss
npm install @fortawesome/fontawesome-free
```

### Database
Use sqlx-cli to generate the database schema (defined in migrations).
For this, make sure `DATABASE_URL` pointing to the "test" database is set in `.env` and run:
```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```
This only needs to be done if changes to the queries are made.
To compile the rust code without connection to the database (cached database queries) use:
```bash
cargo sqlx prepare
```
This generates a .sqlx directory with the compiled queries.

### Styling
Make sure tailwind.config.js points to the template directory with the HTML files.
Then build the CSS file with:
```bash
npx tailwindcss build src/input.css -o public/ouptut.css
```

### To Do
- Backup project list
- Restore from backup
- Update components using htmx
