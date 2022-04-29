# Running

First, export the database url:

    export DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter

Then, start the database using docker:

    ./scripts/init_db.sh

Finally, run the project:

    cargo run

To enable nice display of logs:

    RUST_LOG=trace cargo run | bunyan
