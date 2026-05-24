#!/usr/bin/env bash
set -euo pipefail

TABLE="${DDB_TABLE:-system-calls-dev}"
REGION="${AWS_REGION:-ap-northeast-1}"
SEED_FILE="${1:-$(dirname "$0")/seed.json}"

ENDPOINT_FLAG=()
if [[ -n "${DDB_ENDPOINT:-}" ]]; then
  ENDPOINT_FLAG=(--endpoint-url "$DDB_ENDPOINT")
fi

if [[ ! -f "$SEED_FILE" ]]; then
  echo "seed file not found: $SEED_FILE" >&2
  exit 1
fi

COUNT=$(jq 'length' "$SEED_FILE")
echo "loading $COUNT items from $SEED_FILE into $TABLE"

for ((i = 0; i < COUNT; i += 25)); do
  CHUNK=$(jq --arg t "$TABLE" --argjson off "$i" \
    '{($t): .[$off:($off + 25)]}' "$SEED_FILE")

  aws dynamodb batch-write-item \
    --request-items "$CHUNK" \
    "${ENDPOINT_FLAG[@]}" \
    --region "$REGION" \
    >/dev/null

  END=$((i + 25 < COUNT ? i + 25 : COUNT))
  printf "  %d/%d\n" "$END" "$COUNT"
done

echo "done"
