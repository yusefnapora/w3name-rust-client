#!/usr/bin/env bash

apt-get update
apt-get install -y curl unzip

PROTOC_VERSION="3.20.2"
ARCH=$(uname -m)
PB_REL="https://github.com/protocolbuffers/protobuf/releases"
ZIPFILE="protoc-$PROTOC_VERSION-linux-$ARCH.zip"
curl -LO $PB_REL/download/v$PROTOC_VERSION/$ZIPFILE

mkdir -p /opt/protobuf
unzip $ZIPFILE -d /opt/protobuf
