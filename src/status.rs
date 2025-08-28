use crate::{models::StatusEvent, token::TokenInfo, transcript::TranscriptReader};
use std::{fs, path::Path};

impl StatusEvent {
    /// モデル名を取得
    pub fn model_name(&self) -> &str {
        &self.model.display_name
    }

    /// 現在のディレクトリパスを取得
    pub fn current_dir(&self) -> &str {
        &self.workspace.current_dir
    }

    /// ディレクトリ名を取得
    pub fn dir_name(&self) -> &str {
        Path::new(&self.workspace.current_dir)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }

    /// Gitブランチ情報を取得
    pub fn git_branch(&self) -> String {
        fs::read_to_string(
            Path::new(&self.workspace.current_dir)
                .join(".git")
                .join("HEAD"),
        )
        .ok()
        .and_then(|content| {
            content
                .strip_prefix("ref: refs/heads/")
                .map(|b| format!(" | 🌿 {}", b.trim()))
        })
        .unwrap_or_default()
    }

    /// トークン使用情報を取得
    pub fn get_token_info(&self) -> Option<TokenInfo> {
        let path = self.transcript_path.as_ref()?;
        let tokens = TranscriptReader::get_latest_token_usage(path).ok()?;
        if tokens > 0 {
            Some(TokenInfo::new(tokens))
        } else {
            None
        }
    }

    /// ステータスライン文字列を生成
    pub fn format_statusline(&self) -> String {
        let token_status = self
            .get_token_info()
            .map(|info| info.format_status())
            .unwrap_or_default();

        format!(
            "[{}] 📁 {}{}{}",
            self.model_name(),
            self.dir_name(),
            self.git_branch(),
            token_status
        )
    }
}
