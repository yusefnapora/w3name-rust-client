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

### Native dependencies

To install with `cargo`, you'll need the [Protocol Buffers compiler](https://grpc.io/docs/protoc-installation/), and the `protoc` command must be on your `$PATH`. Version `3.20.2` is known to work, and other 3.x versions are likely to work as well.

You'll also need `perl`, since we build openssl from source, and `perl` is required by the build process.

### Add `w3name` client library as a dependency

If you're building a rust project, you can add the `w3name` crate to your `Cargo.toml`:

```toml
[dependencies]
w3name = "0.1.0"
```

### Install the `w3name` command-line tool with `cargo`

You can install the CLI tool with `cargo install`:

```sh
cargo install w3name-cli
```

This should make the `w3name` command available on your `$PATH`.

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
## Using the `w3name` command-line tool

The `w3name` command line tool has commands for creating a new name keypair, publishing values, and retrieving the latest value for a name.

You can get an overview with `w3name help`:

```
w3name 0.1.0
A tool for creating verifiable names in a web3 world

USAGE:
    w3name <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    create     Create a new public/private keypair and save it to disk
    help       Print this message or the help of the given subcommand(s)
    publish    Publish a new value for a name, signed with the name's private key
    resolve    Lookup the current value for a name record
```

Each of the subcommands has it's own help text available using `w3name help <command>` or `w3name <command> --help`, for example:

```sh
w3name help create
```

```
w3name-create 
Create a new public/private keypair and save it to disk

USAGE:
    w3name create [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

    -o, --output <OUTPUT>
            Filename to write the key to.
            
            If not given, will write to a file named `<name>.key`, where `<name>` is the string form
            of the public key.

```

### Resolving the value of a name

To lookup the current value for a name record, use `w3name resolve <name>`, where `<name>` is string name identifier.

For example:

```sh
w3name resolve k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu
```

```
hello from w3name-rust-client!
```

### Creating a new keypair

Before you can publish name records, you need to create a keypair using `w3name create`.

With no arguments, it will create a file named `<name>.key` in the current directory, where `<name>` is the string form of the public key.

```sh
w3name create
```

```
wrote new keypair to k51qzi5uqu5dhm3u68li82fpf3952az41aqs0k3opk0wtjyevfud1ohv2qkyrc.key
```

If you want, you can pass the `--output` flag to control the output filename:

```sh
w3name create --output foo.key
```

```
wrote new keypair to foo.key
```

### Publishing values

Once you have a key file, you can publish values with `w3name publish`:

```sh
w3name publish --key your-key-file.key --value "A shiny new value"
```

```
published new value for key k51qzi5uqu5dka3tmn6ipgsrq1u2bkuowdwlqcw0vibledypt1y9y5i8v8xwvu: A shiny new value
```

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




[w3name]: https://github.com/web3-storage/w3name

[w3name-crate]: https://example.com/FIXME/upate-once-crate-is-published
[w3name-cli-crate]: https://example.com/FIXME/upate-once-crate-is-published