#!/bin/bash

target="$*"
if [ -z "${target}" ]; then
	targe="build --release"
fi

for toml in $(find . -name Cargo.toml); do
	project=$(dirname $toml)
	(cd ${project} && cargo ${target})
done

