# Subtitle server pro max

<!--toc:start-->

- [Subtitle server pro max](#subtitle-server-pro-max)
  - [Local development](#local-development)
    - [Requirements](#requirements)
    - [Backend](#backend)
    - [Frontend](#frontend)

<!--toc:end-->

## Local development

### Requirements

- Rust
- pnpm
- node
- Docker
- Diesel CLI

### Backend

First create `.env` file with the following set

```sh
DATABSE_URL=<path/to/db>
```

```sh
docker compose up db
diesel migration run
cargo run
```

### Frontend

First create `.env` file with the following set

```sh
PUBLIC_BACKEND_URL=<url_to_backend>
```

```sh
pnpm i
pnpm run dev
```
