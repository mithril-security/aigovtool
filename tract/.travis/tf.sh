#!/bin/sh

set -ex

if [ -z "$CACHEDIR" ]
then
    CACHEDIR=`dirname $0`/../.cached
fi


(cd tensorflow; cargo test --release --features conform)
