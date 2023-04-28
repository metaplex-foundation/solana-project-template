#!/usr/bin/env bash

# Replaces the references to my_program_name and MyProgram1111111111111111111111111111111111 with the provided name and ID, respectively

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
ROOT_DIR=$(dirname "$SCRIPT_DIR")

OLD_NAME="my_program_name"
OLD_ID="MyProgram1111111111111111111111111111111111"

print_usage() {
  printf "Usage: \n-n: Name to replace %s \n-i: Base-58 ID to replace %s \n" $OLD_NAME $OLD_ID
}

NEW_NAME=""
NEW_ID=""

while getopts 'n:i:r:a:h' flag; do
  case "${flag}" in
  n) NEW_NAME="${OPTARG}" ;;
  i) NEW_ID="${OPTARG}" ;;
  r) OLD_NAME="${OPTARG}" ;;
  a) OLD_ID="${OPTARG}" ;;
  *)
    print_usage
    exit 1
    ;;
  esac
done

ID_FILES=(
  "$ROOT_DIR"/program/src/lib.rs
)

NAME_FILES=(
  "$ROOT_DIR"/.solitarc.js
  "$ROOT_DIR"/.ammanrc.js
  "$ROOT_DIR"/yarn.lock
  "$ROOT_DIR"/packages/sdk/idl/"$OLD_NAME".json
)

NAME_FILES_TS=(
  "$ROOT_DIR"/program/Cargo.toml
  "$ROOT_DIR"/program/Cargo.lock
  "$ROOT_DIR"/packages/sdk/package.json
  "$ROOT_DIR"/package.json
  "$ROOT_DIR"/packages/sdk/typedoc.json
  "$ROOT_DIR"/yarn.lock
)

OLD_NAME_TS=$(echo "$OLD_NAME" | tr _ -)
NEW_NAME_TS=$(echo "$NEW_NAME" | tr _ -)

if [ "$NEW_ID" == "" ] && [ "$NEW_NAME" == "" ]; then
  print_usage
  exit 1
fi

function replace() {
  if [ "$2" != "" ]; then
    local old=$1
    local new=$2
    shift
    shift
    local arr=("$@")
    for file in "${arr[@]}"; do
      sed -i "s/$old/$new/g" "${file}"
    done
    echo "Replaced all $old's with $new!"
  fi
}

replace "$OLD_ID" "$NEW_ID" "${ID_FILES[@]}"
replace "$OLD_NAME" "$NEW_NAME" "${NAME_FILES[@]}"
replace "$OLD_NAME_TS" "$NEW_NAME_TS" "${NAME_FILES_TS[@]}"

mv "$ROOT_DIR"/packages/sdk/src/"$OLD_NAME_TS".ts "$ROOT_DIR"/packages/sdk/src/"$NEW_NAME_TS".ts
mv "$ROOT_DIR"/packages/sdk/idl/"$OLD_NAME".json "$ROOT_DIR"/packages/sdk/idl/"$NEW_NAME".json
echo "Renamed the default files!"
