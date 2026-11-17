# Day 26: Environment and Configuration — solutions

Try the [exercises](../days/26-environment-and-configuration/README.md#exercises) first.

## 1. Add a region setting

Follow the existing pattern: read `APP_REGION`, then fall back to a `REGION` key from the config file and finally a safe default.

## 2. Override a setting

On a Unix-like shell, run `APP_MODE=production ./config`. In PowerShell, set `$env:APP_MODE = "production"` before running the program. The environment value takes precedence over the file.

## 3. Parse a port

```rust
fn parse_port(text: &str) -> Result<u16, std::num::ParseIntError> {
    text.parse()
}
```

Handle the returned `Result` and report invalid values clearly.

## 4–5. Local settings and placeholders

Add private local files to `.gitignore`; commit only templates such as `APP_TOKEN=replace-me`. A real credential must never be committed because repository history can retain it after deletion.
