use extism_pdk::*;
use fluentci_pdk::dag;

use crate::helpers::setup_dagger;
pub mod helpers;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let stdout = setup_dagger(version, true)?;
    Ok(stdout)
}

#[plugin_fn]
pub fn call(args: String) -> FnResult<String> {
    let version = dag().get_env("DAGGER_VERSION").unwrap_or_default();
    setup_dagger(version, false)?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["dagger", "call", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn run(args: String) -> FnResult<String> {
    let version = dag().get_env("DAGGER_VERSION").unwrap_or_default();
    setup_dagger(version, false)?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["dagger", "run", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn query(args: String) -> FnResult<String> {
    let version = dag().get_env("DAGGER_VERSION").unwrap_or_default();
    setup_dagger(version, false)?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["dagger", "query", &args])?
        .stdout()?;
    Ok(stdout)
}
