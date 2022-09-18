# w3name-rust-client

> A client library and command line tool for w3name IPNS over HTTP service.

<h1 align="center">⁂<br/>w3name</h1>
<p align="center">Content addressing for a dynamic web.</p>

## About

`w3name-rust-client` provides a rust client library and a command line tool for interacting with the [w3name][] service.

w3name is a service that implements [IPNS](https://docs.ipfs.io/concepts/ipns/), which is a protocol that uses public key cryptography to allow for updatable naming in an atomically verifiable way. 

For more about w3name in general, see the [main github repository][w3name].

`w3name-rust-client` provides two crates: [`w3name`][w3name-crate], which provides an HTTP client and tools for creating and validating name records, and [`w3name-cli`][w3name-cli-crate], which lets you publish and resolve name records from the command line.

### Status & API stability

This project is quite new, and there may be breaking API changes as things are developed. If you notice that the description below has drifted from the published API, please open an issue so we can update the docs!

## Using the `w3name` client library

There are two main types that represent "names":

- `Name` is used to fetch and verify name records. It contains the public key for a name, but not the private key, and so cannot be used to publish records.

- `WritableName` is used to sign records for publication. It contains the private key as well as the public key. Calling `to_name()` on a `WritableName` instance will return a `Name` containing just the public half of the keypair.

### Creating a `WritableName`

To create a new name, use `WritableName::new()`, which will generate a new keypair.

You can save this to disk by calling `keypair().to_protobuf_encoding()` on a `WritableName` instance, which will give you a `Vec<u8>` in a format that's acceptable to `WritableName::from_private_key()`. Please keep the key in a safe location, as it will allow the holder to update your published records.




[w3name]: https://github.com/web3-storage/w3name

[w3name-crate]: https://example.com/FIXME/upate-once-crate-is-published
[w3name-cli-crate]: https://example.com/FIXME/upate-once-crate-is-published