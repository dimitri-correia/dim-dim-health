DATABASE_URL="postgres://dev:dev-db@localhost:5432/dimdimhealthdev"

sea-orm-cli migrate up --database-url $DATABASE_URL -v
sea-orm-cli generate entity --database-url $DATABASE_URL --output-dir entities/src/generated  --with-serde "both" -v