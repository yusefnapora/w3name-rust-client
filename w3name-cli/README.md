<h1 align="center">⁂<br/>w3name</h1>
<p align="center">Content addressing for a dynamic web. Now available from the command line!</p>

## About

The `w3name-cli` binary crate provides a command-line tool called `w3name`, which can be used to interact with the w3name service.

For more about w3name in general, see the [main github repository](https://github.com/web3-storage/w3name).

## Install

### Binary CLI releases

To download a pre-compiled binary of the `w3name` command-line tool, grab the `.tar.gz` file for your platform from the [latest release](https://github.com/yusefnapora/w3name-rust-client/releases).

### Using `cargo install`

Make sure to read the [Native dependencies](#native-dependencies) section below!

```sh
cargo install w3name-cli
```

### Native dependencies

To install with `cargo`, you'll need the [Protocol Buffers compiler](https://grpc.io/docs/protoc-installation/), and the `protoc` command must be on your `$PATH`. Version `3.20.2` is known to work, and other 3.x versions are likely to work as well.

If you can't install `protoc`, but you do have `cmake`, you can set the `protoc-src` feature, which will build the protobuf compiler from source at build time.

You'll also need `perl`, since we build openssl from source, and `perl` is required by the build process.

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

