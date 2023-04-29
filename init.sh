#!/usr/bin/env bash

# This script renames the default project name, description and
# public key with the provided values. Simply update the values
# below, run "./init.sh" in your terminal and you're good to go!

NAME="mpl-foo-name",
DESCRIPTION="My foo description",
PUBLIC_KEY="MplFoo1111111111111111111111111111111111"

# ------------------------------------
# --- Do not edit below this line. ---
# ------------------------------------

# Initial variables
ROOT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
OLD_NAME="mpl-project-name",
OLD_DESCRIPTION="My project description",
OLD_PUBLIC_KEY="MplProgram1111111111111111111111111111111111"

# snake_case
SNAKE_NAME=$(echo "$NAME" | perl -pe 's/-/_/g')
SNAKE_OLD_NAME=$(echo "$OLD_NAME" | perl -pe 's/-/_/g')

# camelCase
CAMEL_NAME=$(echo "$NAME" | perl -pe 's/-(\w)/\U$1/g')
CAMEL_OLD_NAME=$(echo "$OLD_NAME" | perl -pe 's/-(\w)/\U$1/g')

# PascalCase
PASCAL_NAME=$(echo "$NAME" | perl -pe 's/(^|-)(\w)/\U$2/g')
PASCAL_OLD_NAME=$(echo "$OLD_NAME" | perl -pe 's/(^|-)(\w)/\U$2/g')

# Title Case
TITLE_NAME=$(echo "$PASCAL_NAME" | perl -pe 's/(\B[A-Z])/ $1/g')
TITLE_OLD_NAME=$(echo "$PASCAL_OLD_NAME" | perl -pe 's/(\B[A-Z])/ $1/g')

echo "ROOT_DIR: $ROOT_DIR"
echo "OLD_NAME: $OLD_NAME"
echo "NAME: $NAME"
echo "SNAKE_NAME: $SNAKE_NAME"
echo "SNAKE_OLD_NAME: $SNAKE_OLD_NAME"
echo "CAMEL_NAME: $CAMEL_NAME"
echo "CAMEL_OLD_NAME: $CAMEL_OLD_NAME"
echo "PASCAL_NAME: $PASCAL_NAME"
echo "PASCAL_OLD_NAME: $PASCAL_OLD_NAME"
echo "TITLE_NAME: $TITLE_NAME"
echo "TITLE_OLD_NAME: $TITLE_OLD_NAME"

find $ROOT_DIR \
  \( -type d -name .git -prune \) -o \
  \( -type d -name node_modules -prune \) -o \
  \( -type d -name dist -prune \) -o \
  ! -name 'README' \
  ! -name '*.sh' \
  -type f -print0 |
  xargs -0 perl -pi -e 's/$ENV{OLD_NAME}/$ENV{NAME}/g'

# ID_FILES=(
#   "$ROOT_DIR"/program/src/lib.rs
# )

# NAME_FILES=(
#   "$ROOT_DIR"/.solitarc.js
#   "$ROOT_DIR"/.ammanrc.js
#   "$ROOT_DIR"/yarn.lock
#   "$ROOT_DIR"/packages/sdk/idl/"$OLD_NAME".json
# )

# NAME_FILES_TS=(
#   "$ROOT_DIR"/program/Cargo.toml
#   "$ROOT_DIR"/program/Cargo.lock
#   "$ROOT_DIR"/packages/sdk/package.json
#   "$ROOT_DIR"/package.json
#   "$ROOT_DIR"/packages/sdk/typedoc.json
#   "$ROOT_DIR"/yarn.lock
# )

# OLD_NAME_TS=$(echo "$OLD_NAME" | tr _ -)
# NEW_NAME_TS=$(echo "$NEW_NAME" | tr _ -)

# if [ "$NEW_ID" == "" ] && [ "$NEW_NAME" == "" ]; then
#   print_usage
#   exit 1
# fi

# function replace() {
#   if [ "$2" != "" ]; then
#     local old=$1
#     local new=$2
#     shift
#     shift
#     local arr=("$@")
#     for file in "${arr[@]}"; do
#       sed -i "s/$old/$new/g" "${file}"
#     done
#     echo "Replaced all $old's with $new!"
#   fi
# }

# replace "$OLD_ID" "$NEW_ID" "${ID_FILES[@]}"
# replace "$OLD_NAME" "$NEW_NAME" "${NAME_FILES[@]}"
# replace "$OLD_NAME_TS" "$NEW_NAME_TS" "${NAME_FILES_TS[@]}"

# mv "$ROOT_DIR"/packages/sdk/src/"$OLD_NAME_TS".ts "$ROOT_DIR"/packages/sdk/src/"$NEW_NAME_TS".ts
# mv "$ROOT_DIR"/packages/sdk/idl/"$OLD_NAME".json "$ROOT_DIR"/packages/sdk/idl/"$NEW_NAME".json
# echo "Renamed the default files!"
