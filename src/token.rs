/// トークン使用量情報を表す型
#[derive(Debug, Clone)]
pub struct TokenInfo {
    tokens: u64,
}

impl TokenInfo {
    /// 新しいTokenInfoを作成
    pub fn new(tokens: u64) -> Self {
        Self { tokens }
    }

    /// トークン数を取得
    pub fn count(&self) -> u64 {
        self.tokens
    }

    /// 200Kトークンに対するパーセンテージを計算
    pub fn percentage(&self) -> u32 {
        (self.tokens as f32 / 2000.0).round() as u32
    }

    /// 表示用のトークン数文字列を生成（例: "72K" or "500"）
    pub fn display_count(&self) -> String {
        if self.tokens >= 1000 {
            format!("{}K", self.tokens / 1000)
        } else {
            self.tokens.to_string()
        }
    }

    /// ステータス絵文字を取得
    pub fn status_emoji(&self) -> &str {
        match self.percentage() {
            p if p > 75 => "🔴", // 危険域：コンパクティング間近（80%で発生）
            p if p > 60 => "🟡", // 注意域：残り少ない
            _ => "🟢",           // 安全域：通常使用
        }
    }

    /// ステータスライン用のフォーマット済み文字列を生成
    pub fn format_status(&self) -> String {
        if self.tokens == 0 {
            String::new()
        } else {
            format!(
                " | {} {} ({}%)",
                self.status_emoji(),
                self.display_count(),
                self.percentage()
            )
        }
    }
}

impl From<u64> for TokenInfo {
    fn from(tokens: u64) -> Self {
        Self::new(tokens)
    }
}
