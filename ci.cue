package ci

import (
	"dagger.io/dagger"
	"dagger.io/dagger/core"
	"universe.dagger.io/bash" // import this package to execute bash commands inside a docker container
	"universe.dagger.io/docker" // import this package to set up docker
)

dagger.#Plan & {
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
		"release-builds": write: contents: actions.release_build_all.output
	}


	actions: {
		// builds the default Docker image used for testing, without cross compiler support 
		image: docker.#Dockerfile & {
			source: client.filesystem."./".read.contents
			dockerfile: path: "Dockerfile.test"
		}

		// builds the "release build image", which has cross-compilation toolchains installed
		build_image_main: docker.#Dockerfile & {
			source: client.filesystem."./".read.contents
			dockerfile: path: "Dockerfile.build"
			buildArg: { 
				CROSS_LINUX: "true" 
				CROSS_WINDOWS: "true"
			}
		}

		// it's faster overall to do a separate macos build image, since it takes so long to
		// install the toolchain.
		build_image_macos: docker.#Dockerfile & {
			source: client.filesystem."./".read.contents
			dockerfile: path: "Dockerfile.build"
			buildArg: CROSS_MACOS: "true"
		}

		release_build_all: core.#Merge & {
			inputs: [ 
				release_build_linux.export.directories.release,
				release_build_mac.export.directories.release,
				release_build_windows.export.directories.release,
			] 
		}

		release_build_mac: bash.#Run & {
			input: build_image_macos.output
			workdir: "/src"
			export: directories: {
				"release": dagger.#FS,
			}
			env: CLI_VERSION: "\(cli_version.version)"
			script: contents: #"""
				source /build/cross-env-macos.sh	

				cargo build -p w3name-cli --release --target aarch64-apple-darwin
				cargo build -p w3name-cli --release --target x86_64-apple-darwin

				# make tarballs for each architecture
				mkdir -p release
				tar -czf release/w3name-cli-${CLI_VERSION}-macos-x86_64.tar.gz -C target/x86_64-apple-darwin/release w3name
				tar -czf release/w3name-cli-${CLI_VERSION}-macos-aarch64.tar.gz -C target/aarch64-apple-darwin/release w3name
			"""#
		}

		release_build_linux: bash.#Run & {
			input: build_image_main.output
			export: directories: {
				"release": dagger.#FS
			}
			workdir: "/src"
			env: CLI_VERSION: "\(cli_version.version)"
			script: contents: #"""
				source /build/cross-env-linux.sh	

				cargo build -p w3name-cli --release --target aarch64-unknown-linux-gnu
				cargo build -p w3name-cli --release --target x86_64-unknown-linux-gnu

				# make tarballs for each architecture
				mkdir -p release
				tar -czf release/w3name-cli-${CLI_VERSION}-linux-x86_64.tar.gz -C target/x86_64-unknown-linux-gnu/release w3name
				tar -czf release/w3name-cli-${CLI_VERSION}-linux-aarch64.tar.gz -C target/aarch64-unknown-linux-gnu/release w3name
			"""#
		}

		release_build_windows: bash.#Run & {
			input: build_image_main.output
			workdir: "/src"
			export: directories: {
				"release": dagger.#FS
			}
			env: CLI_VERSION: "\(cli_version.version)"
			script: contents: #"""
				cargo build -p w3name-cli --release --target x86_64-pc-windows-gnu

				# make a zip file containing the w3name.exe file
				mkdir -p release
				cd target/x86_64-pc-windows-gnu/release
				zip /src/release/w3name-cli-${CLI_VERSION}-windows-x86_64.zip w3name.exe
			"""#
		}

		// get the w3name-cli version from cargo metadata
		cli_version: {
			_op: bash.#Run & {
				input: image.output
				workdir: "/src"
				script: contents: #"""
					cargo metadata --format-version=1 --no-deps \
					  | jq -j '.packages[] | select(.name == "w3name-cli") | .version' \
						> /w3cli-version
				"""#
				export: files: {
					"/w3cli-version": _
				}
			}
			version: _op.export.files."/w3cli-version"
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