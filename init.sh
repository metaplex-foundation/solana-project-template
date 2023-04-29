#!/usr/bin/env bash

# This script renames the default project name, description and
# public key with the provided values. Simply update the values
# below, run "./init.sh" in your terminal and you're good to go!

NAME="mpl-foo-name"
DESCRIPTION="My foo description"
PUBLIC_KEY="MyFoo1111111111111111111111111111111111"

# ------------------------------------
# --- Do not edit below this line. ---
# ------------------------------------

# Initial variables
ROOT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
OLD_NAME="mpl-project-name"
OLD_DESCRIPTION="My project description"
OLD_PUBLIC_KEY="MyProgram1111111111111111111111111111111111"

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

# Find and replace
find $ROOT_DIR \
  \( -type d -name .git -prune \) -o \
  \( -type d -name node_modules -prune \) -o \
  \( -type d -name dist -prune \) -o \
  \( -type d -name .crates -prune \) -o \
  ! -name 'README.md' \
  ! -name '*.sh' \
  -type f -print0 |
  xargs -0 perl -pi -e "s/$OLD_NAME/$NAME/g; s/$SNAKE_OLD_NAME/$SNAKE_NAME/g; s/$CAMEL_OLD_NAME/$CAMEL_NAME/g; s/$PASCAL_OLD_NAME/$PASCAL_NAME/g; s/$TITLE_OLD_NAME/$TITLE_NAME/g; s/$OLD_DESCRIPTION/$DESCRIPTION/g; s/$OLD_PUBLIC_KEY/$PUBLIC_KEY/g"

# Update folder names
# mv "$ROOT_DIR"/packages/sdk/src/"$OLD_NAME_TS".ts "$ROOT_DIR"/packages/sdk/src/"$NEW_NAME_TS".ts
# mv "$ROOT_DIR"/packages/sdk/idl/"$OLD_NAME".json "$ROOT_DIR"/packages/sdk/idl/"$NEW_NAME".json
# echo "Renamed the default files!"
