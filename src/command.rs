//! Which binary to run as the language server, and with what arguments.

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
}
