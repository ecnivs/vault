# Vault
> Secrets Manager

#### Global secrets
cargo run -- add --global API_KEY=secret123

#### Project secrets (note the --project and --env flags)
cargo run -- add --project myapp --env dev API_KEY=secret456

#### List commands
cargo run -- list --global
cargo run -- list --project myapp --env dev

#### Load commands
cargo run -- load --global --export
cargo run -- load --project myapp --env dev --export
