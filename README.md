# Marc's UNSW CSE Tournament Infrastructure
Heavy work in progress -- see a very outdated demo at [https://tourn.insou.dev/](https://tourn.insou.dev/)

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
```
If in production, you'll also need to add `ROCKET_WEBAPP_URL=<webapp url>` to this file, for CORS configuration. Also, if your production server is behind SSL, then you will need to edit `Rocket.toml` to allow for secure cookies in production.

Run `cargo sqlx database reset`.

You can now run `cargo build` to build a debug executable, `cargo run` to run for development and can add `--release` to either of these to run in production with an optimised build.

### Webapp and Multiplayer Server
You will need [Node (LTS) and NPM](https://nodejs.org/en/) installed.
I recommend using [`nvm`](https://github.com/nvm-sh/nvm) (or [`n`](https://formulae.brew.sh/formula/n) on MacOS) to manage Node versions.

In the `webapp` directory create a `.env` file with the following content (assuming this is for development):
```
SNOWPACK_PUBLIC_API_URL=http://localhost:8000
SNOWPACK_PUBLIC_LOBBY_SERVER_URL=http://locahost:8081
SNOWPACK_PUBLIC_GAME_SERVER_URL=http://locahost:8081
```
If in production, these will need to point to the API, lobby and game servers.

Run `npm install`.

To run a local development server for the webapp, run `npm run start`.
To build static files for production, run `npm run build`.
To run the lobby and game server, run `npm run server`.

## Contributing
Feel free to fork this repo and create pull requests. Issues with bugs, feedback or feature requests are also appreciated.