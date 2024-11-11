use zed::LanguageServerId;
use zed_extension_api as zed;

struct BuflsZedExtension {}

// https://buf.build/docs/reference/cli/buf/beta/lsp/?h=lsp
impl zed::Extension for BuflsZedExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        match worktree.which("buf") {
            Some(path) => Ok(zed::Command {
                command: path,
                args: vec![
                    String::from("beta"),
                    String::from("lsp"),
                    String::from("--debug"),
                    String::from("--timeout=5m"),
                ],
                env: vec![],
            }),
            None => Err("buf in not found.".into()),
        }
    }
}

zed::register_extension!(BuflsZedExtension);
