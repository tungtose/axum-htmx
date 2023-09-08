# Load env
set dotenv-load

format:
  prettier --write .

styles:
  pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css --watch

dev:
  cargo watch -x 'run'

export DATABASE_URL := "sqlite:todos.db"

create-db:
  sqlx db create

create-migrate name:
  sqlx migrate add {{name}}

migrate:
  sqlx migrate run

init:
  just create-db
  just migrate

clean:
  rm -rf *.db*


