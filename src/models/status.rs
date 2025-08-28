use serde::Deserialize;

// Claude Code Status Hook Event の型定義（実際の形式）
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct StatusEvent {
    pub session_id: String,
    pub transcript_path: Option<String>,
    pub cwd: String,
    pub model: Model,
    pub workspace: Workspace,
    pub version: String,
    pub output_style: OutputStyle,
    pub cost: Cost,
    pub exceeds_200k_tokens: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Model {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Workspace {
    pub current_dir: String,
    pub project_dir: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OutputStyle {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Cost {
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub total_api_duration_ms: u64,
    pub total_lines_added: u64,
    pub total_lines_removed: u64,
}