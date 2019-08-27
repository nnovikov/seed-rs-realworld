[![Build Status](https://travis-ci.org/MartinKavik/seed-rs-realworld.svg?branch=master)](https://travis-ci.org/MartinKavik/seed-rs-realworld) [![Netlify Status](https://api.netlify.com/api/v1/badges/e4891ed3-3ecd-43ae-ab60-0fd8647b372d/deploy-status)](https://app.netlify.com/sites/seed-rs-realworld/deploys)

# ![RealWorld Example App](logo.png)

> ### Seed codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

### [Demo](https://seed-rs-realworld.netlify.com/) &nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

This codebase was created to demonstrate a fully fledged fullstack application built with **Seed** including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the **Seed** community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

# How it works

I think the best way to show you how it works is to describe what's going on step by step when you open this example in your browser. So let's say you've just written `https://seed-rs-realworld.netlify.com/` to URL bar and pressed Enter:

1. Netlify redirects your request to `index.html`. (See `/netlify.toml`.)
1. There is a script in `/index.html` that loads `wasm` file and starts application.
1. Application is initialized in `/src/lib.rs` - see block `Start` at the end of that file.
1. The first is called function `init` (we are still in file `lib.rs`):
   1. It tries to load `Viewer` from local storage. `Viewer` is the object that contains info about currently logged in user (name, auth. token, avatar image url, etc).
   1. Then it creates a new `Session` with `Viewer` (or without it if you are not logged in). `Session` is de facto shared state - you are able to get it from all pages.
   1. Then the `Model` is created with `Session`. `Model` is enum, where each variant represents one page. Here, in `init` function, we create `Model` from variant `Redirect` because we haven't decided yet which page to show (i.e. `Redirect` is a "placeholder" variant).
   1. And we also try to parse given URL and send result to Seed's runtime.
1. `init` function sends message `RouteChanged` which is handled in `update` function. Handler calls function `change_model_by_route` which choose the right `Model` according to the URL path.
1. `Model::Home` is chosen in our example. However this variant should contain "sub-model" from `/src/page/home` so we need to call `page::home::init(..)` to get it.
1. `page::home::init` loads required data (e.g. article feed or tags).
1. We have data to show so when Seed's runtime calls `View` function again we can see it rendered in the browser.

We didn't talked about function `sink` in `lib.rs` in the steps above. `sink` has the similar purpose like `update` function but it handles global messages. It's the way how modules communicate with parents or with other modules. The most use case is to send `GMsg::RoutePushed` when we want to go to another page (see function `go_to` in `/src/route.rs`).

# Getting started

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Update Rust: `$ rustup update`
3. Install WASM target: `$ rustup target add wasm32-unknown-unknown`
4. Install [cargo-make](https://sagiegurari.github.io/cargo-make/): `$ cargo install --force cargo-make`
5. Build project from its root: `$ cargo make all` or `$ cargo make watch`
   - _Note:_ You need some dependencies like `pkg-config` on Linux - just follow recommendations after compilation errors.
6. Start local server: `$ cargo make serve`
   - _Note:_ You have to open a new terminal tab/window if you used `$ cargo make watch`.
7. Open in you browser [localhost:8000](http://localhost:8000/)

# Contributing

1. Create issues and PRs - bugs, missing documentation, typos, unreadable code...
2. Squash commits, rebase on the current `master` and run `$ cargo make verify` (+ commit changes, if any) before creating PR.

# Statistics

```bash
$ tokei
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 HTML                    1           21           19            2            0
 Markdown                1           64           64            0            0
 Rust                   81         5888         4864          338          686
 SVG                     1           17           17            0            0
 TOML                    4          198          145           20           33
-------------------------------------------------------------------------------
 Total                  88         6188         5109          360          719
-------------------------------------------------------------------------------
```
