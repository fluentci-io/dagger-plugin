use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_dagger(version: String, force: bool) -> Result<String, Error> {
    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;

    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    let check = if force {
        ""
    } else {
        "type dagger > /dev/null ||"
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![&format!(
            "{} pkgx curl -L https://dl.dagger.io/dagger/install.sh | BIN_DIR=$HOME/.local/bin DAGGER_VERSION={} sh",
            check, version
        )])?
        .stdout()?;

    Ok(stdout)
}
