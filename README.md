# find-a-date-everyone-can-do-api

## Dev

export $(cat .env.dev | xargs) && cargo watch -x run

## Deployment

Deployments are triggered whenever a pull request to the 'prod' branch is merged.

You can test the Dockerfile with `docker build .`.
