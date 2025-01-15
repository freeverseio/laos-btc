#!/bin/bash

VERSION=$(grep -m1 version Cargo.toml | sed -E 's/.*"(.*)".*/\1/')

if echo "$TAG_NAME" | grep -qE '^v[0-9][0-9]?\.[0-9][0-9]?\.([0-9]|[1-8][0-9])$'; then
  echo "is_release_version=true" >> $GITHUB_OUTPUT
  if [ "v$VERSION" == "$TAG_NAME" ]; then
    echo "Proceeding with tag $TAG_NAME matching Cargo.toml version v$VERSION."
  else
    echo "Version in Cargo.toml ($VERSION) does not match current tag $TAG_NAME. Canceling release..."
    exit 1
  fi
else
  echo "is_release_version=false" >> $GITHUB_OUTPUT
  echo "Tag $TAG_NAME does not have the format 'v*.*.*'. Skipping version check and proceeding..."
fi
