<h1 align="center">⁂<br/>w3name</h1>
<p align="center">Content addressing for a dynamic web. Now available from Rust!</p>

## About

The `w3name` crate provides a client library for the w3name service, an implementation of the  [IPNS](https://docs.ipfs.io/concepts/ipns/) decentralized naming protocol.

For more about w3name in general, see the [main github repository](https://github.com/web3-storage/w3name).

## Install

Add the `w3name` crate to your Cargo.toml:

```toml
[dependencies]
w3name = "0.1.0"
```

### Native dependencies

To install with `cargo`, you'll need the [Protocol Buffers compiler](https://grpc.io/docs/protoc-installation/), and the `protoc` command must be on your `$PATH`. Version `3.20.2` is known to work, and other 3.x versions are likely to work as well.

If you can't install `protoc`, but you do have `cmake`, you can set the `protoc-src` feature, which will build the protobuf compiler from source at build time.

You'll also need `perl`, since we build openssl from source, and `perl` is required by the build process.

## Usage

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

See [w3name-cli/src/main.rs](../w3name-cli/src/main.rs) for an example of using the client to publish and resolve names.

<!-- TODO: add publish and resolve examples here -->
