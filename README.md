# Voice Korea

## Common Environments
| Name     | Description                                                                    |
|----------|--------------------------------------------------------------------------------|
| RUST_LOG | Logging libraries based on tracing(`debug`, `info`, `error`)                   |
| SERVICE  | Package name to be executed. default is `main-ui`                              |
| DOMAIN   | Base domain name (ex. dev.example.com) will be used to compose signing message |


## Development
### Running API Server(main-api)
- It runs `SERVICE` crate.
  - Default `SERVICE` is `main-ui`.

``` bash
export SERVICE=main-api
export NONCE_LAB_API_TOKEN="TOKEN"
make run
```

### Running Web UI(main-ui)
- It will interact with API server in `dev` environment.
  - If you want to change it, set `API_URL` environment.
- Before running UI, set up firebase configuration

``` bash
export SERVICE=main-ui
export API_URL=http://localhost:3000

make run
```


## Deployment
### Main UI

``` bash
export STACK=voice-korea-dev-stack
export SERVICE=main-ui
export ENV=dev
export BASE_DOMAIN=biyard.co
export DOMAIN=voice-korea.dev.biyard.co

export ENABLE_S3=true
export ENABLE_LAMBDA=true
export ENABLE_FARGATE=false
export ENABLE_DYNAMO=false
export ENABLECRON=false


```

### Main API

``` bash
export ENV=dev
export STACK=voice-korea-api-dev-stack
export SERVICE=api
export ENABLE_S3=false
export ENABLE_FARGATE=false
export ENABLE_DYNAMO=false
export ENABLE_RDS=false
export ENABLE_LAMBDA=true
export BASE_DOMAIN=voice-korea.com
export DOMAIN=voice-korea-api.dev.voice-korea.com

export RDS_ADMIN_PASSWORD=""
export DATABASE_TYPE=postgres
export DATABASE_URL=""
export NONCE_LAB_API_TOKEN=
export INTERNAL_SERVER_KEY=


ENV=dev make deploy
```

### Watcher API

``` bash
export STACK=voice-korea-watcher-api-dev-stack
export ENV=dev
export SERVICE=watcher
export ENABLE_S3=false
export ENABLE_FARGATE=false
export ENABLE_DYNAMO=false
export ENABLE_RDS=false
export ENABLE_LAMBDA=true
export BASE_DOMAIN=biyard.co
export ENABLE_CRON=true
export SCHEDULE='cron(0 15 * * ? *)'
export DOMAIN=voice-korea-watcher-api.dev.biyard.co


```
