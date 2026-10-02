mod command;

#[cfg(target_family = "wasm")]
mod extension {
    use crate::command;
    use zed_extension_api::{
        self as zed, serde_json, settings::LspSettings, LanguageServerId, Result,
    };

    /// The key under `lsp` in Zed's settings, and the server's id in `extension.toml`.
    const SERVER: &str = "schemata";

    struct SchemataExtension;

    impl zed::Extension for SchemataExtension {
        fn new() -> Self {
            SchemataExtension
        }

        fn language_server_command(
            &mut self,
            _language_server_id: &LanguageServerId,
            worktree: &zed::Worktree,
        ) -> Result<zed::Command> {
            let binary = LspSettings::for_worktree(SERVER, worktree)
                .ok()
                .and_then(|settings| settings.binary);
            let (command, args) = command::choose(
                binary.as_ref().and_then(|binary| binary.path.clone()),
                binary.as_ref().and_then(|binary| binary.arguments.clone()),
                worktree.which("schemata"),
            )?;
            Ok(zed::Command {
                command,
                args,
                env: command::merge_env(worktree.shell_env(), binary.and_then(|binary| binary.env)),
            })
        }

        fn language_server_initialization_options(
            &mut self,
            _language_server_id: &LanguageServerId,
            worktree: &zed::Worktree,
        ) -> Result<Option<serde_json::Value>> {
            Ok(LspSettings::for_worktree(SERVER, worktree)
                .ok()
                .and_then(|settings| settings.initialization_options))
        }

        fn language_server_workspace_configuration(
            &mut self,
            _language_server_id: &LanguageServerId,
            worktree: &zed::Worktree,
        ) -> Result<Option<serde_json::Value>> {
            Ok(LspSettings::for_worktree(SERVER, worktree)
                .ok()
                .and_then(|settings| settings.settings)
                .map(command::workspace_configuration))
        }
    }

    zed::register_extension!(SchemataExtension);
}
