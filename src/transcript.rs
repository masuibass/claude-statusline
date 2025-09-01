use crate::models::TranscriptEntry;
use std::{
    fs,
    io::{Read, Seek, SeekFrom},
};

/// トランスクリプトファイルの読み込みとパースを担当
pub struct TranscriptReader;

impl TranscriptReader {
    const MAX_READ_BYTES: usize = 100_000;

    /// トランスクリプトファイルから最新のトークン使用量を取得
    pub fn get_latest_token_usage(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let content = Self::read_tail(path)?;
        Ok(Self::parse_latest_usage(&content).unwrap_or(0))
    }

    /// ファイルの末尾部分を読み込む（大きなファイルの場合は最後の100KB）
    fn read_tail(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let file_size = fs::metadata(path)?.len();

        if file_size > Self::MAX_READ_BYTES as u64 {
            // 大きなファイルは末尾のみ読む
            let mut file = fs::File::open(path)?;
            let mut buffer = vec![0u8; Self::MAX_READ_BYTES];
            file.seek(SeekFrom::End(-(Self::MAX_READ_BYTES as i64)))?;
            file.read_exact(&mut buffer)?;
            Ok(String::from_utf8_lossy(&buffer).into_owned())
        } else {
            // 小さなファイルは全体を読む
            fs::read_to_string(path).map_err(Into::into)
        }
    }

    /// コンテンツから最新のトークン使用量を解析
    fn parse_latest_usage(content: &str) -> Option<u64> {
        content
            .lines()
            .rev()
            .filter_map(|line| TranscriptEntry::parse_line(line))
            .find_map(|entry| entry.total_tokens())
    }
}

// TranscriptEntryの拡張実装
impl TranscriptEntry {
    /// 1行からTranscriptEntryをパース
    pub fn parse_line(line: &str) -> Option<Self> {
        line.try_into().ok()
    }

    /// エントリからトークン総数を取得
    pub fn total_tokens(&self) -> Option<u64> {
        self.message
            .usage
            .as_ref()
            .map(|usage| usage.total_tokens())
    }
}

// TryFromの実装
impl TryFrom<&str> for TranscriptEntry {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}
