package ci

import (
	"dagger.io/dagger"
	"universe.dagger.io/bash" // import this package to execute bash commands inside a docker container
	"universe.dagger.io/docker" // import this package to set up docker
)

dagger.#Plan & {
	// configure the client so that dagger takes only the files it needs
	client: filesystem: "./": read: {
		contents: dagger.#FS
		exclude: [
			"README.md",
			"rust-ci.cue",
			"target/",
			"result/"
		]
	}

	actions: {
		// pull the official rust image and copy my code
		deps: docker.#Build & {
			steps: [
				docker.#Pull & {
					source: "rust:1.61.0"
				},
				docker.#Copy & {
					contents: client.filesystem."./".read.contents
					dest:     "/src"
				},
			]
		}
		// run the test suite inside the docker container
		test: bash.#Run & {
			input:   deps.output
			workdir: "/src"
			script: contents: #"""
				cargo test
				"""#
		}
	}
}