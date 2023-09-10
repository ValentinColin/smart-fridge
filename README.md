# Smart fridge

## Run

Before Running:
```bash
export DATABASE_PASSWORD=fridge
```

Then run:
```bash
docker compose up
```

## Environment

Variables d'environnement par défaut par niveau:

| variables         | application | docker compose     | My env or .env |
|-------------------|-------------|--------------------|----------------|
| DATABASE_ADDR     | db          | db                 | localhost      |
| DATABASE_NAME     | postgres    | postgres           |                |
| DATABASE_USERNAME | postgres    | postgres           | fridge         |
| DATABASE_PASSWORD |             |                    | fridge         |
| WEB_APP_HOST      | 0.0.0.0     |                    | 127.0.0.1      |
| WEB_APP_PORT      | 80          |                    | 8000           |
| RUST_LOG          |             | smart_fridge=debug |                |