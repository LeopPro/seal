#!/usr/bin/env bash

echo "copy src/front_end to target/$PROFILE/front_end"
# cp -r $CARGO_MANIFEST_DIR/src/front_end $CARGO_MANIFEST_DIR/target/$PROFILE

cd src/front_end
npm run debug
cd -
cp $CARGO_MANIFEST_DIR/src/front_end/index.html $CARGO_MANIFEST_DIR/target/$PROFILE/front_end/index.html