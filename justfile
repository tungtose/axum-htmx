# Load env
set dotenv-load

format:
  prettier --write .

styles:
  pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css --watch

dev:
  cargo watch -x 'run'
