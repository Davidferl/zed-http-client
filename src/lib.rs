use std::{env, fs};
use zed_extension_api::{self as zed, Result};

const LS_PATH: &str = "node_modules/@mistweaverco/kulala-ls/cli.cjs";
const LS_PACKAGE_NAME: &str = "@mistweaverco/kulala-ls";

struct HttpClientExtension {
    did_find_server: bool,
}

impl HttpClientExtension {
    fn language_server_exists(&self) -> bool {
        fs::metadata(LS_PATH).map_or(false, |stat| stat.is_file())
    }

    fn ensure_language_server_installed(
        &mut self,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<String> {
        let server_exists = self.language_server_exists();
        if self.did_find_server && server_exists {
            return Ok(LS_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let version = zed::npm_package_latest_version(LS_PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(LS_PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(LS_PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.language_server_exists() {
                        Err(format!(
                            "installed package '{LS_PACKAGE_NAME}' did not contain expected path '{LS_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.language_server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(LS_PATH.to_string())
    }
}

impl zed::Extension for HttpClientExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.ensure_language_server_installed(language_server_id)?;
        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(HttpClientExtension);
