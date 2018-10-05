#!/usr/bin/env bash

echo "copy src/front_end to target/$PROFILE/front_end"
cp -r $CARGO_MANIFEST_DIR/src/front_end $CARGO_MANIFEST_DIR/target/$PROFILE
