# Dagger Plugin

[![ci](https://github.com/fluentci-io/dagger-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/dagger-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [dagger](https://dagger.io).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm dagger setup
```

## Functions

| Name   | Description                                |
| ------ | ------------------------------------------ |
| setup  | Installs a specific version of dagger.     |
| call   | Call a module function                     |
| run    | Run a command in a Dagger session          |
| query  | Send API queries to a dagger engine        |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/dagger@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: dagger
    args: |
      setup
    working-directory: example
- name: Show dagger version
  run: |
    export PATH=${HOME}/.local/bin:${PATH}
    type dagger
    dagger version
```
