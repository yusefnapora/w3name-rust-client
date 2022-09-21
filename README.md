<h1 align="center">⁂<br/>w3name-rust-client</h1>
<p align="center">Content addressing for a dynamic web. Now available from Rust!</p>

## About

`w3name-rust-client` provides a rust client library and a command line tool for interacting with the [w3name][] service.

w3name is a service that implements [IPNS](https://docs.ipfs.io/concepts/ipns/), which is a protocol that uses public key cryptography to allow for updatable naming in an atomically verifiable way. 

For more about w3name in general, see the [main github repository][w3name].

`w3name-rust-client` provides two crates: [`w3name`][w3name-crate], which provides an HTTP client and tools for creating and validating name records, and [`w3name-cli`][w3name-cli-crate], which lets you publish and resolve name records from the command line.

### Status & API stability

This project is quite new, and there may be breaking API changes as things are developed. If you notice that the description below has drifted from the published API, please open an issue so we can update the docs!

## Install & Usage

Please see the READMEs for the individual crates:

- [w3name](./w3name/README.md) contains the rust library
- [w3name-cli](./w3name-cli/README.md) contains the command-line tool

### Binary CLI releases

To download a pre-compiled binary of the `w3name` command-line tool, grab the `.tar.gz` file for your platform from the [latest release](https://github.com/yusefnapora/w3name-rust-client/releases).

### Nix

If you use the [Nix package manager](https://nixos.org/), you can start a new shell with the `w3name` command line tool installed:

```sh
nix shell github:yusefnapora/w3name-rust-client
```

The above assumes that you're using Nix flakes. If not, you can clone this repository and build a "legacy" package with:

```sh
nix-build . -A defaultPackage.x86_64-linux # or aarch64-linux, etc
```

[w3name]: https://github.com/web3-storage/w3name

[w3name-crate]: https://example.com/FIXME/upate-once-crate-is-published
[w3name-cli-crate]: https://example.com/FIXME/upate-once-crate-is-published