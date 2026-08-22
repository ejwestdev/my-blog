This is my personal site built using Dioxus (Rust compiled to WASM). It is hosted on Cloudflare Pages and deployed using the Wrangler CLI. Everything is written using either Markdown or Dioxus components. url: https://ejwest-personal.pages.dev/

## 🧞 Commands

All commands are run from the root of the project, from a terminal:

| Command                    | Action                                  |
| :------------------------- | :-------------------------------------- |
| `cargo install dioxus-cli` | Installs the Dioxus CLI (once)          |
| `dx serve`                 | Starts local dev server with hot reload |
| `dx build --release`       | Build your production site to `./dist/` |
| `dx serve --release`       | Preview the release build locally       |

## Deploy

```sh
dx build --release
npx wrangler pages deploy ./dist
```
