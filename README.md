# OpenAPI

OpenAPI definitions are in `./static` directory.


# Run the app locally

__1. Create .env file in the root of the project with following variables__

```sh
PORT=8000 # app fallbacks to 8000 if PORT env variable is not found

AWS_ACCESS_KEY_ID=''
AWS_SECRET_ACCESS_KEY=''
AWS_REGION='eu-central-1'

# `compose.yaml` currently requires `.env` file to exist
```

__2. Build & run__
```sh
docker compose up --build
```


