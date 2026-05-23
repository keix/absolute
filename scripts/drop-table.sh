#!/usr/bin/env bash
set -euo pipefail

TABLE="${DDB_TABLE:-system-calls-dev}"
ENDPOINT="${DDB_ENDPOINT:-http://localhost:8000}"
REGION="${AWS_REGION:-ap-northeast-1}"

aws dynamodb delete-table \
  --table-name "$TABLE" \
  --endpoint-url "$ENDPOINT" \
  --region "$REGION"
