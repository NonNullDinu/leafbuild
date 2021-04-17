use crate::workspace_root;
use cmd::CmdResult;
use cmd::{cargo_cmd, cmd_call};
use std::path::Path;

pub fn format() -> CmdResult {
    cargo_cmd!("+stable fmt -- --emit=files")
}

pub fn format_check() -> CmdResult {
    cargo_cmd!("+stable fmt -- --check")
}

pub fn format_file(file: &Path) -> CmdResult<()> {
    cmd_call!(&format!(
        "rustfmt +stable --config-path {}/.rustfmt.toml {}",
        workspace_root().display(),
        file.display()
    ))
}
