# Marc's UNSW CSE Tournament Infrastructure
Heavy work in progress -- see a live version [here](https://marc.hamishwhc.com/).

## Local Development Setup (and Deployment)
This application is developed on and for Linux. Development on Windows is unlikely to work. WSL and MacOS are likely to work but untested.

To begin, clone this repo.

### API Server
You will need [Rust](https://www.rust-lang.org/tools/install) installed.
Then run `cargo install sqlx-cli`.
This may crash due to either openssl-sys or sqlite3.
You will need to install the build dependencies for your OS for these libraries.

Run `openssl rand -base64 32` and copy the output. In the `api` directory create a file named `.env` with the following content:
```
DATABASE_URL=sqlite://../database.db
ROCKET_SECRET_KEY=<output of openssl command>
REDIS_URL=redis://127.0.0.1:6379
```
If in production, you'll also need to add `ROCKET_WEBAPP_URL=<webapp url>` to this file, for CORS configuration. Also, if your production server is not behind SSL, then you will need to edit `Rocket.toml` to allow for insecure cookies in production.

Run `cargo sqlx database reset`. Note that this will reset your database! If you want to keep existing data, use `cargo sqlx database migrate run` to run any pending migrations.

Note: Due to being mid-development, we're just editing the original migration, so database resets are necessary.

You will also need to install [docker](https://docker.com) and either [add your user to the docker group](https://docs.docker.com/engine/install/linux-postinstall/#manage-docker-as-a-non-root-user) or [run docker without root](https://docs.docker.com/engine/security/rootless/).

Then, run `docker run --name redis -p 6379:6379 -d redis` to run the Redis server in a docker container.
This can be started again (e.g. after a restart) with `docker start redis` or stopped with `docker stop redis`.
To view all running containers (including player binaries), use `docker ps` (to include stopped containers, use `-a`).

You can now run `cargo run --bin <component>` to run a given component of the server for development and can add `--release` to this to run it in production with an optimised build.
The following components exist:
* `api`: API server that responds to the webapp.
* `worker`: Background worker that listens for games to play.
* `rating-audit`: Mangement tool that recalculates player ratings for all games (takes a while, probably best to run while worker and api processes are stopped). This exists so that changes to the rating system can be made mid-competition with minimal interruption.
### Webapp and Multiplayer Server
You will need [Node (LTS) and NPM](https://nodejs.org/en/) installed.
I recommend using [`nvm`](https://github.com/nvm-sh/nvm) (or [`n`](https://formulae.brew.sh/formula/n) on MacOS) to manage Node versions.

In the `webapp` directory create a `.env` file with the following content (assuming this is for development):
```
SNOWPACK_PUBLIC_API_URL=http://localhost:8000
SNOWPACK_PUBLIC_LOBBY_SERVER_URL=http://locahost:8081
SNOWPACK_PUBLIC_GAME_SERVER_URL=http://locahost:8081
```
If in production, these will need to point to the API, lobby and game servers, and the line `ORIGIN=<url of webapp>` must also be added.

Run `npm install`.

To run a local development server for the webapp, run `npm run start`.
To build static files for production, run `npm run build`.
To run the lobby and game server, run `npm run server`.

## Contributing
Feel free to fork this repo and create pull requests. Issues with bugs, feedback or feature requests are also appreciated.