<h1 align="center">⁂<br/>w3name-rust-client</h1>
<p align="center">Content addressing for a dynamic web. Now available from Rust!</p>

## About

`w3name-rust-client` provides a rust client library and a command line tool for interacting with the [w3name][] service.

w3name is a service that implements [IPNS](https://docs.ipfs.io/concepts/ipns/), which is a protocol that uses public key cryptography to allow for updatable naming in an atomically verifiable way. 

For more about w3name in general, see the [main github repository][w3name].

`w3name-rust-client` provides two crates: [`w3name`][w3name-crate], which provides an HTTP client and tools for creating and validating name records, and [`w3name-cli`][w3name-cli-crate], which lets you publish and resolve name records from the command line.

### Status & API stability

This project is quite new, and there may be breaking API changes as things are developed. If you notice that the description below has drifted from the published API, please open an issue so we can update the docs!

## Install

Until this gets published on crates.io, you can install from git using:

```sh
cargo install --git https://github.com/yusefnapora/w3name-rust-client
```

Precompiled binaries coming soon!

## Using the `w3name` client library

There are two main types that represent "names":

- `Name` is used to fetch and verify name records. It contains the public key for a name, but not the private key, and so cannot be used to publish records.

- `WritableName` is used to sign records for publication. It contains the private key as well as the public key. Calling `to_name()` on a `WritableName` instance will return a `Name` containing just the public half of the keypair.

### Creating a `WritableName`

To create a new name, use `WritableName::new()`, which will generate a new keypair.

You can save this to disk by calling `keypair().to_protobuf_encoding()` on a `WritableName` instance, which will give you a `Vec<u8>` in a format that's acceptable to `WritableName::from_private_key()`. Please keep the key in a safe location, as it will allow the holder to update your published records.


### Parsing a `Name` from string

A `Name` is a wrapper around a public key, which when encoded to a string looks something like this:

```
k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu
```

This string is a Content Identifier (CID) with the bytes of the public key embedded within it.

You can convert the string representation to a `Name` struct by calling `Name::parse`.


### Using the `W3NameClient` to publish and resolve names

The `W3NameClient` struct provides a [reqwest](https://docs.rs/reqwest/latest/reqwest/)-based HTTP client for interacting with the w3name service. As it uses the `async` reqwest implementation, you'll need a [tokio](https://tokio.rs/) runtime in order to use it.

See [w3name-cli/src/main.rs](./w3name-cli/src/main.rs) for an example of using the client to publish and resolve names.

<!-- TODO: add publish and resolve examples here -->

## Using the `w3name-cli` command-line tool

Instructions coming soon! In the meantime, try `w3name-cli help` to get a feel for what's there.


[w3name]: https://github.com/web3-storage/w3name

[w3name-crate]: https://example.com/FIXME/upate-once-crate-is-published
[w3name-cli-crate]: https://example.com/FIXME/upate-once-crate-is-published