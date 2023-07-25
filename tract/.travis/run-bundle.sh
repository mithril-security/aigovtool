#!/bin/sh

set -ex

BUNDLE_NAME=$1

tar zxf $BUNDLE_NAME.tgz
(
    cd $BUNDLE_NAME
    . ./vars
    ./entrypoint.sh
)
# rm -rf "$BUNDLE_NAME" "$BUNDLE_NAME.tgz"
