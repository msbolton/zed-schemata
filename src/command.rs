//! Which binary to run as the language server, with what arguments and environment, and what
//! configuration to send it.

use serde_json::{json, Value};
use std::collections::HashMap;

/// Shown when no binary can be found.
pub const NOT_FOUND: &str = "schemata was not found on PATH. Install it with \"brew install \
msbolton/schemata/schemata\", or set lsp.schemata.binary.path. The language server needs \
schemata 0.8.0 or later.";

/// The command and arguments to start the server with: the configured path when one is set,
/// otherwise the binary found on PATH; the configured arguments, otherwise `lsp`.
pub fn choose(
    configured_path: Option<String>,
    configured_arguments: Option<Vec<String>>,
    on_path: Option<String>,
) -> Result<(String, Vec<String>), String> {
    let command = configured_path
        .filter(|path| !path.is_empty())
        .or(on_path)
        .ok_or_else(|| NOT_FOUND.to_string())?;
    let arguments = configured_arguments.unwrap_or_else(|| vec!["lsp".to_string()]);
    Ok((command, arguments))
}

/// The environment to start the server in: the worktree's shell environment with the entries of
/// `binary.env` laid over it. A setting wins on a shared name.
pub fn merge_env(
    shell: Vec<(String, String)>,
    configured: Option<HashMap<String, String>>,
) -> Vec<(String, String)> {
    let configured = configured.unwrap_or_default();
    let mut merged: Vec<(String, String)> = shell
        .into_iter()
        .filter(|(name, _)| !configured.contains_key(name))
        .collect();
    let mut overrides: Vec<(String, String)> = configured.into_iter().collect();
    overrides.sort();
    merged.extend(overrides);
    merged
}

/// The workspace configuration to send: the settings as they are when they already hold a
/// `schemata` key, otherwise wrapped as `{"schemata": settings}`, which is where the server
/// reads its options.
pub fn workspace_configuration(settings: Value) -> Value {
    if settings.get("schemata").is_some() {
        settings
    } else {
        json!({ "schemata": settings })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_configured_path_wins_over_path() {
        let chosen = choose(
            Some("/opt/schemata".to_string()),
            None,
            Some("/usr/local/bin/schemata".to_string()),
        );
        assert_eq!(chosen, Ok(("/opt/schemata".to_string(), strings(&["lsp"]))));
    }

    #[test]
    fn path_is_used_when_nothing_is_configured() {
        let chosen = choose(None, None, Some("/usr/local/bin/schemata".to_string()));
        assert_eq!(
            chosen,
            Ok(("/usr/local/bin/schemata".to_string(), strings(&["lsp"])))
        );
    }

    #[test]
    fn configured_arguments_replace_the_default() {
        let chosen = choose(
            None,
            Some(strings(&["lsp", "--verbose"])),
            Some("/usr/local/bin/schemata".to_string()),
        );
        assert_eq!(
            chosen,
            Ok((
                "/usr/local/bin/schemata".to_string(),
                strings(&["lsp", "--verbose"])
            ))
        );
    }

    #[test]
    fn no_binary_anywhere_is_the_install_message() {
        assert_eq!(choose(None, None, None), Err(NOT_FOUND.to_string()));
    }

    #[test]
    fn an_empty_configured_path_falls_back_to_path() {
        let chosen = choose(Some(String::new()), None, Some("/bin/schemata".to_string()));
        assert_eq!(chosen, Ok(("/bin/schemata".to_string(), strings(&["lsp"]))));
    }

    fn pair(name: &str, value: &str) -> (String, String) {
        (name.to_string(), value.to_string())
    }

    #[test]
    fn a_setting_wins_over_the_shell_on_a_shared_name() {
        let shell = vec![pair("PATH", "/usr/bin"), pair("JAVA_HOME", "/shell/jdk")];
        let configured = HashMap::from([("JAVA_HOME".to_string(), "/setting/jdk".to_string())]);
        assert_eq!(
            merge_env(shell, Some(configured)),
            vec![pair("PATH", "/usr/bin"), pair("JAVA_HOME", "/setting/jdk")]
        );
    }

    #[test]
    fn settings_add_names_the_shell_lacks() {
        let configured = HashMap::from([("EXTRA".to_string(), "1".to_string())]);
        assert_eq!(
            merge_env(vec![pair("PATH", "/usr/bin")], Some(configured)),
            vec![pair("PATH", "/usr/bin"), pair("EXTRA", "1")]
        );
    }

    #[test]
    fn no_configured_env_leaves_the_shell_as_it_is() {
        let shell = vec![pair("PATH", "/usr/bin")];
        assert_eq!(merge_env(shell.clone(), None), shell);
    }

    #[test]
    fn settings_without_a_schemata_key_are_wrapped() {
        assert_eq!(
            workspace_configuration(json!({ "roots": ["model"], "strict": true })),
            json!({ "schemata": { "roots": ["model"], "strict": true } })
        );
    }

    #[test]
    fn settings_with_a_schemata_key_pass_through() {
        let settings = json!({ "schemata": { "roots": ["model"] } });
        assert_eq!(workspace_configuration(settings.clone()), settings);
    }
}
