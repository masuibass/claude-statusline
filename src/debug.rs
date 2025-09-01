#[cfg(feature = "debug_json")]
use std::io::Write;

/// デバッグログの種類
#[cfg(feature = "debug_json")]
pub enum LogType {
    /// stdin から受け取ったステータスJSON
    Status,
    /// .jsonl ファイルのトランスクリプトエントリ
    Transcript,
}

/// デバッグログ出力を管理
#[cfg(feature = "debug_json")]
pub struct DebugLogger;

#[cfg(feature = "debug_json")]
impl DebugLogger {
    /// JSONデータをデバッグログファイルに出力
    pub fn log_json(log_type: LogType, content: &str) {
        let log_path = match log_type {
            LogType::Status => "/tmp/claude-code-statusline-debug-status.log",
            LogType::Transcript => "/tmp/claude-code-statusline-debug-transcript.log",
        };
        
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
        {
            // タイムスタンプを生成
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            // ログタイプに応じたヘッダーを出力
            let header = match log_type {
                LogType::Status => "Received JSON from stdin:",
                LogType::Transcript => "Transcript entry from .jsonl:",
            };
            
            let _ = writeln!(file, "[{}] {}", timestamp, header);
            let _ = writeln!(file, "{}", content);
            
            // JSONのパースを試みて、整形版も出力
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(content) {
                if let Ok(pretty) = serde_json::to_string_pretty(&json_value) {
                    let _ = writeln!(file, "Pretty formatted:");
                    let _ = writeln!(file, "{}", pretty);
                }
            }
            
            let footer = match log_type {
                LogType::Status => "--- END stdin input ---",
                LogType::Transcript => "--- END transcript entry ---",
            };
            let _ = writeln!(file, "{}", footer);
        }
    }
}