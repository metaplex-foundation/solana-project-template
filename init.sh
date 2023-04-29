#!/usr/bin/env bash

# This script renames the default project name, description and
# public key with the provided values. Simply update the values
# below, run "./init.sh" in your terminal and you're good to go!

NAME="mpl-project-name",
DESCRIPTION="My project description",
PUBLIC_KEY="MplProgram1111111111111111111111111111111111"

# ------------------------------------
# --- Do not edit below this line. ---
# ------------------------------------

ROOT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
OLD_NAME="mpl-project-name",
OLD_DESCRIPTION="My project description",
OLD_PUBLIC_KEY="MplProgram1111111111111111111111111111111111"

echo "ROOT_DIR: $ROOT_DIR"
echo "OLD_NAME: $OLD_NAME"
echo "OLD_DESCRIPTION: $OLD_DESCRIPTION"
echo "OLD_PUBLIC_KEY: $OLD_PUBLIC_KEY"
echo "NAME: $NAME"
echo "DESCRIPTION: $DESCRIPTION"
echo "PUBLIC_KEY: $PUBLIC_KEY"

# find $ROOT_DIR \( -type d -name .git -prune \) -o -type f -print0 | xargs -0 sed -i "s/mpl-project-name/mpl-foo-bar/g"

find $ROOT_DIR \
  \( -type d -name .git -prune \) -o \
  \( -type d -name node_modules -prune \) -o \
  ! -name 'README' \
  ! -name '*.sh' \
  -type f -print0 |
  xargs -0 perl -pi -e "s/mpl-project-name/mpl-foo-bar/g"

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
