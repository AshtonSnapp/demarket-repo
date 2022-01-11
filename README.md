# Demarket Repo

Demarket (or, if you're a startup or hipster, "dmarqet") is an idea I've had for a while - a protocol or API for creating a decentralized app store. Although, now, the scope has been expanded to allow for other kinds of media - video, audio, and eBooks - so to keep things simple I'll refer to everything as "assets".

This repository is for the repository server, which actually serves the assets and metadata. The market server is implemented in Rust using Rocket with a Redis database, and has a Svelte+TypeScript+Bootstrap frontend built-in. There's also a repository for [the market server](https://github.com/AshtonSnapp/demarket-repo). \(I guess this app uses the Svelte Red Rocket stack :grinning:\)

This project is currently in its early stages. Many things are still undetermined.