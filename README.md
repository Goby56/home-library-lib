## Launching the app

### Dependencies
- docker

1. Create a `.env` file in the root of the project
2. Populate the following environment variables:
- `DATABASE_DIR` - where the app's data should be stored
- `SITE_DOMAIN` - what domain/address to serve the webapp from
3.  Run the following command to start the webapp. Add the flag `--build` if it is the first time.
```

docker compose up
```


## Develop (Linux)

### Dependencies
- rust
- npm

To start the webapp in dev mode you'll need to start the front and backend separately.
Run the script `develop.sh` in `frontend` and `backend` respectively
