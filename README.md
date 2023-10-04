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

Default environment variables per level:

| variables         | application | docker compose     | My env or .env |
|-------------------|-------------|--------------------|----------------|
| DATABASE_ADDR     | db          | db                 | localhost      |
| DATABASE_NAME     | postgres    | postgres           |                |
| DATABASE_USERNAME | postgres    | postgres           | fridge         |
| DATABASE_PASSWORD |             |                    | fridge         |
| WEB_APP_HOST      | 0.0.0.0     |                    | 127.0.0.1      |
| WEB_APP_PORT      | 80          |                    | 8000           |
| RUST_LOG          |             | smart_fridge=debug |                |

## API

| METHOD | ROUTE               | DESCRIPTION                                 | RETURN                   |
|--------|---------------------|---------------------------------------------|--------------------------|
| GET    | /api/v2/healthcheck | Used to check the health of the http server | (200, body: "OK")        |
| GET    | /api/v2/food        | Get all row/food from the database          | (200, body: JSON) or 500 |
| POST   | /api/v2/food        | Add food in the database                    | 204 or 500               |
| GET    | /api/v2/food/:uuid  | Get one row/food from the database          | (200, body: JSON) or 500 |
| DELETE | /api/v2/food/:uuid  | Delete on row/food in the database          | 204 or 500               |


## Documentation

Build and open the doc in your browser

```bash
cargo doc --open
```

## 

```bash
cli healthcheck
cli list
cli add <NAME> [<expiration_date>]
cli get <UUID>
cli delete <UUID>
```
