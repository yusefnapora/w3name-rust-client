package ci

import (
	"dagger.io/dagger"
	"universe.dagger.io/bash" // import this package to execute bash commands inside a docker container
	"universe.dagger.io/docker" // import this package to set up docker
)

dagger.#Plan & {
	// mount the docker socket, for docker-in-docker cross compiling
	client: network: "unix:///var/run/docker.sock": connect: dagger.#Socket

	// read some env vars from the client 
	client: env: {
		RUST_TARGET: string | *"x86_64-unknown-linux-gnu",
	}

	client: filesystem: "./": read: {
		contents: dagger.#FS
		exclude: [
			"README.md",
			"ci.cue",
			"target/",
			"result/",
		]
	}

	client: filesystem: { 
		"release-builds/linux_x86_64": write: contents: actions.release_build_linux.export.directories."/src/target/x86_64-unknown-linux-gnu/release"
    "release-builds/linux_aarch64": write: contents: actions.release_build_linux.export.directories."/src/target/aarch64-unknown-linux-gnu/release"
		"release-builds/macos_x86_64": write: contents: actions.release_build_mac.export.directories."/src/target/x86_64-apple-darwin/release"
    "release-builds/macos_aarch64": write: contents: actions.release_build_mac.export.directories."/src/target/aarch64-apple-darwin/release"
	}


	actions: {
		// builds the default Docker image used for testing, without cross compiler support 
		image: docker.#Dockerfile & {
			source: client.filesystem."./".read.contents
		}

		// builds the "release image", which is the same as default but with cross compilers installed
		release_image: docker.#Dockerfile & {
			source: client.filesystem."./".read.contents
			buildArg: CROSS_COMPILERS: "true"
		}

		// builds an image with a cross compilation toolchain for macOS builds installed
		release_image_mac: docker.#Dockerfile & {
			source: client.filesystem."./".read.contents
			buildArg: CROSS_MACOS: "true"
		}

		release_build_mac: bash.#Run & {
			input: release_image_mac.output
			workdir: "/src"
			export: directories: {
				"/src/target/aarch64-apple-darwin/release": dagger.#FS,
				"/src/target/x86_64-apple-darwin/release": dagger.#FS,
			}
			script: contents: #"""
				CROSS_BIN=/build/osxcross/target/bin
				export PATH=$PATH:$CROSS_BIN

				export CC_x86_64_apple_darwin=$CROSS_BIN/x86_64-apple-darwin20.4-clang
			  export CC_aarch64_apple_darwin=$CROSS_BIN/aarch64-apple-darwin20.4-clang

				cargo build -p w3name-cli --release --target aarch64-apple-darwin
				cargo build -p w3name-cli --release --target x86_64-apple-darwin
			"""#
		}

		release_build_linux: bash.#Run & {
			input: release_image.output
			export: directories: {
				"/src/target/aarch64-unknown-linux-gnu/release": dagger.#FS,
				"/src/target/x86_64-unknown-linux-gnu/release": dagger.#FS,
			}
			workdir: "/src"
			script: contents: #"""
				export HOST_CC=gcc
				export CC_x86_64_unknown_linux_gnu=/usr/bin/x86_64-linux-gnu-gcc
				export CC_aarch64_unknown_linux_gnu=/usr/bin/aarch64-linux-gnu-gcc
				export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc
				export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/x86_64-linux-gnu-gcc

				cargo build -p w3name-cli --release --target aarch64-unknown-linux-gnu
				cargo build -p w3name-cli --release --target x86_64-unknown-linux-gnu
			"""#
		}

		// run the test suite inside the docker container
		test: bash.#Run & {
			input:   image.output
			workdir: "/src"
			script: contents: #"""
				cargo test
			"""#
		}
	}
}