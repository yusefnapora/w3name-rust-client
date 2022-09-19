#!/bin/bash

# copied from https://github.com/JasonEtco/upload-to-release/blob/master/upload-to-release
# because we can't run docker-based workflows on macos runners, apparently

set -e
set -o pipefail

# Ensure that the GITHUB_TOKEN secret is included
if [[ -z "$GITHUB_TOKEN" ]]; then
  echo "Set the GITHUB_TOKEN env variable."
  exit 1
fi

# Ensure that the file path is present
if [[ -z "$1" ]]; then
  echo "You must pass at least one argument to this action, the path to the file to upload."
  exit 1
fi

# Only upload to non-draft releases
IS_DRAFT=$(jq --raw-output '.release.draft' $GITHUB_EVENT_PATH)
if [ "$IS_DRAFT" = "true" ]; then
  echo "This is a draft, so nothing to do!"
  exit 0
fi

# Prepare the headers
AUTH_HEADER="Authorization: Bearer ${GITHUB_TOKEN}"

if [[ -z "$2" ]]; then
  CONTENT_TYPE_HEADER="Content-Type: ${2}"
else
  CONTENT_TYPE_HEADER="Content-Type: application/gzip"
fi

# Construct the upload url
RELEASE_ID=$(jq --raw-output '.release.id' $GITHUB_EVENT_PATH)
if [[ -z "${RELEASE_ID}" ]]; then
  echo "There was no release ID in the GitHub event. Are you using the release event type?"
  exit 1
fi

FILENAME=$(basename $1)
UPLOAD_URL="https://uploads.github.com/repos/${GITHUB_REPOSITORY}/releases/${RELEASE_ID}/assets?name=${FILENAME}"

echo "$UPLOAD_URL"

echo "uploading file:"
ls -lh $1

# Upload the file
curl \
  -f \
  -sSL \
  -XPOST \
  -H "Accept: application/vnd.github+json" \
  -H "${AUTH_HEADER}" \
  -H "${CONTENT_TYPE_HEADER}" \
  --data-binary "@${1}" \
  "${UPLOAD_URL}"