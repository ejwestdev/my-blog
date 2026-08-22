---
title = "Creating a Personal Site using WASM and Cloudflare Workers"
date = "July 31, 2026"
---

I originally made my website using React + Astro. This was a pretty effective stack for me, and a lot of the annoying bits were abstracted away. I think a blog should have a simple but elegant UI. I tried to make one that fit that description, mostly by messing with it until it looked nice. Hot-reloading being so quick on that stack helped a lot (foreshadowing).

The port was surprisingly easy to do. Rust is a very ergonomic language once you get over the _initial_ learning curve, and Dioxus very deftly brings in JSX-style development into a Rusty environment. Rust structs are pretty useful, and being able to treat them like types is very helpful for web development. Take this small example:

```Rust
pub struct BlogPost {
  pub id: i32,
  pub title: String,
  pub date: String,
  pub html: String,
}
```

Pretty easy. I could then do things like create a `Vec<BlogPost>` of all my posts, and sort them using iterators. Nothing that's impossible in TypeScript, but it was enjoyable and intuitive to do in Rust for me.

There are a few tradeoffs though. Mainly, the bundle size is fairly large, a little under 1MB compressed. In contrast the Astro site was around 300KB. But, once we load everything, the user experience is faster.

I was hoping to have less dependencies, inspired by [this great talk from Richard Feldman](https://www.youtube.com/watch?v=E82ly38YEEQ), but that didn't really end up being the case. We have 548 dependencies in the Astro site vs 482 in my Cargo.lock. However, what's nice is I don't have to hold a 540MB `node_modules/` folder on disk for this project, all I have is the 2.5MB WASM blob and the 50MB dev blob. If I revisit the design of this site again, I'd probably try to use 0 dependencies and see how far I can get. The [Roc website](https://www.roc-lang.org/) (an FP language written in Rust) has a good amount of features with very few dependencies, and I'd be interested in taking a challenge like that on. A project for another time!
