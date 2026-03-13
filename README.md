# Finicky Whiskers

To learn more about "The World's Most Adorable Manual Load Generator", Finicky Whiskers, and the technical details under the covers, you should read the four-part blog post series accompanying the project:

1. [The World's Most Adorable Manual Load Generator](https://www.fermyon.com/blog/finicky-whiskers-part-1-intro)
2. [Serving the HTML, CSS, and static assets](https://www.fermyon.com/blog/finicky-whiskers-part-2-fileserver)
3. [The Microservices](https://www.fermyon.com/blog/finicky-whiskers-part-3-microservices)
4. [Spin, Containers, Nomad, and Infrastructure](https://www.fermyon.com/blog/finicky-whiskers-part-4-infrastructure)

Finicky Whiskers is comprised of a handful of microservices.

- [site](./site/README.md)
- [scoreboard](./scoreboard/README.md)
- [session](./session/README.md)
- [tally](./tally/README.md)
- [reset](./reset/README.md)
- [highscore](./highscore/)

## Prerequisites

You'll need the `spin` CLI (`3.6.2` or newer) installed on your machine. You can install `spin` using `brew` (Addition installation approaches are described over on [spinframework.dev](https://spinframework.dev)):

```bash
brew tap spinframework/tap
brew install spinframework/tap/spin
```

Finicky Whiskers is implemented using different programming languages. To compile the entire app, you must have the following language toolchains installed on your machine:

- Rust (`1.92.0` or newer) including the `wasm32-wasip1` target
- Node.js (`24.12.0` or newer)

## Compiling Finicky Whiskers

The `spin` CLI streamlines compiling even complex applications that consist of multiple services.

Simply run

```bash
spin build
```

## To Run

The following command will serve the Finicky Whiskers site locally:

```console
spin up
```

This will run the game at [http://127.0.0.1:3000](http://127.0.0.1:3000)

## To Test

The following command will serve the site and then run the integration test
as seen [here](./tests/test-server.sh):

```console
make test-server
```

## Development Notes

For working on the game UI (styles, etc):

Recompiling Assets:

```console
cd site
npm i
npm run styles
```

To just run the UI locally (without the other services) use [Parcel](https://parceljs.org/features/development/) via `npm run dev` and then view the site at [localhost:1234](http://localhost:1234/)
