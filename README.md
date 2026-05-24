# system-calls

API for syscall reference data across OSes × architectures. Axum on AWS Lambda backed by DynamoDB single-table.

## Stack

- Rust + Axum on AWS Lambda (`provided.al2023`, ARM64)
- DynamoDB single-table (`PK = "{OS}#{arch}"`)
- AWS CDK (TypeScript) for infra
- Nix flake for local dev

## Layout

```
src/                Rust crate (axum router, ddb repository, handlers)
examples/           one-shot tools (gen_seed parses syscall_64.tbl)
scripts/            create / drop / seed / fetch / deploy
data/               vendored kernel data (syscall_64.tbl)
infra/              CDK app (ApiStack)
```

## Local development

Open the Nix shell:

```sh
nix develop
```

In one terminal, run DynamoDB Local:

```sh
dynamodb-local -port 8000
```

In another (also inside `nix develop`):

```sh
./scripts/create-table.sh
./scripts/fetch-syscall-tbl.sh
cargo run --example gen_seed
./scripts/seed.sh scripts/seed-linux-x86_64.json
./scripts/seed.sh scripts/seed.json
cargo run --bin local
```

The local server listens on `:3000`. `seed.json` carries the register convention; the order of the two seed runs doesn't matter.

## Endpoints

| Method | Path | Returns |
| --- | --- | --- |
| GET | `/v1/{os}/{arch}/syscalls` | list of syscalls |
| GET | `/v1/{os}/{arch}/syscalls/{name}` | syscall by name |
| GET | `/v1/{os}/{arch}/syscalls/number/{n}` | syscall by number |
| GET | `/v1/{os}/{arch}/registers/{instruction}` | register convention |
| GET | `/health` | `"ok"` |

## Schema

Single table `system-calls-{env}`, keyed by `pk`/`sk` (both string).

| SK pattern | Item |
| --- | --- |
| `SYSCALL#NAME#{name}` | syscall by name |
| `SYSCALL#NR#{number}` | syscall by number — duplicates the full body for 1-read access |
| `REGISTERS#{instruction}` | register convention |

No GSI in the MVP. Cross-arch lookups are deferred until needed.

## Deploy

First time per account/region:

```sh
(cd infra && npm install)
(cd infra && npx cdk bootstrap)
```

Build and deploy:

```sh
./scripts/deploy.sh dev
```

The stack outputs `ApiUrl`, `TableName`, `FunctionName`. To seed the deployed table, unset the local DDB endpoint and target the real one:

```sh
unset DDB_ENDPOINT AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY
DDB_TABLE=system-calls-dev ./scripts/seed.sh scripts/seed-linux-x86_64.json
DDB_TABLE=system-calls-dev ./scripts/seed.sh scripts/seed.json
```

(Or run from a shell that has your real AWS credentials configured, outside `nix develop`.)

## Kernel data

`data/linux-x86_64-syscall_64.tbl` is vendored from `torvalds/linux@v6.12`. Refresh:

```sh
KERNEL_TAG=v6.13 ./scripts/fetch-syscall-tbl.sh
cargo run --example gen_seed
```

Per-argument metadata (register/name/type) is intentionally out of scope. Responses include a `man_url` pointing at `man7.org/linux/man-pages/man2/{name}.2.html` (linux only) — clients should follow it for signatures and semantics.
