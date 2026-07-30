//! Lightweight internationalization for the Codex TUI.
//!
//! Provides a `tr!()` macro that returns translated strings based on the
//! `CODEX_LANG` environment variable. Falls back to English when no
//! translation is available.
//!
//! # Usage
//!
//! ```ignore
//! use crate::tr;
//!
//! // Simple lookup
//! let label = tr!("Working");
//!
//! // With formatted arguments
//! let msg = tr_with_args!("Failed to {action}: {reason}", action = "install", reason = "timeout");
//! ```
//!
//! # Environment
//!
//! - `CODEX_LANG=zh` — Chinese (Simplified)
//! - `CODEX_LANG=en` or unset — English (default)

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;

/// Supported languages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Language {
    En,
    Zh,
}

impl Language {
    fn from_env() -> Self {
        match std::env::var("CODEX_LANG").as_deref() {
            Ok("zh" | "zh-CN" | "zh-cn" | "zh_Hans" | "zh_CN") => Language::Zh,
            _ => Language::En,
        }
    }
}

/// Global language setting, detected once at first access.
static CURRENT_LANG: LazyLock<Mutex<Language>> =
    LazyLock::new(|| Mutex::new(Language::from_env()));

/// Re-check the `CODEX_LANG` environment variable at runtime.
/// This allows switching language without restarting the process.
pub fn refresh_language() {
    if let Ok(mut lang) = CURRENT_LANG.lock() {
        *lang = Language::from_env();
    }
}

/// Force-set language for testing.
#[cfg(test)]
pub fn set_language(lang: Language) {
    if let Ok(mut current) = CURRENT_LANG.lock() {
        *current = lang;
    }
}

/// Look up a translated string by key.
/// Returns the English key as fallback when no translation exists.
pub fn lookup(key: &str) -> &str {
    let lang = CURRENT_LANG.lock().unwrap_or_else(|e| e.into_inner());
    match *lang {
        Language::En => key,
        Language::Zh => ZH_TRANSLATIONS
            .get(key)
            .copied()
            .unwrap_or(key),
    }
}

/// Look up with format arguments.
/// The key is first translated, then formatted with the provided arguments.
pub fn lookup_with_args(key: &str, args: &[(&str, &str)]) -> String {
    let translated = lookup(key);
    if args.is_empty() {
        return translated.to_string();
    }
    let mut result = translated.to_string();
    for (k, v) in args {
        let placeholder = format!("{{{}}}", k);
        result = result.replace(&placeholder, v);
    }
    result
}

/// Macro for simple string lookup.
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::i18n::lookup($key)
    };
    ($key:expr $(, $arg_name:ident = $arg_val:expr )* $(,)?) => {
        $crate::i18n::lookup_with_args($key, &[$((stringify!($arg_name), $arg_val)),*])
    };
}

/// Chinese (Simplified) translations.
static ZH_TRANSLATIONS: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        let mut m: HashMap<&'static str, &'static str> = HashMap::new();

        // === Status State (status_state.rs) ===
        m.insert("Working", "执行中");
        m.insert("Thinking", "思考中");
        m.insert("Reviewing approval request", "审查审批请求");
        m.insert("Reviewing", "审查中");

        // === Terminal Title Status (status_surfaces.rs) ===
        m.insert("Starting", "启动中");
        m.insert("Ready", "就绪");
        m.insert("Waiting", "等待中");
        m.insert("No changes", "无变更");
        m.insert("Fast on", "快速模式开");
        m.insert("Fast off", "快速模式关");

        // === Command lifecycle ===
        m.insert("Waiting for background terminal", "等待后台终端");

        // === Status card labels (status/card.rs) ===
        m.insert("OpenAI Codex", "OpenAI Codex");
        m.insert("Model", "模型");
        m.insert("Directory", "目录");
        m.insert("Permissions", "权限");
        m.insert("Agents.md", "Agents.md（子智能体）");
        m.insert("Model provider", "模型提供商");
        m.insert("Account", "账户");
        m.insert("Thread name", "会话名称");
        m.insert("Session", "会话");
        m.insert("Forked from", "派生自");
        m.insert("Collaboration mode", "协作模式");
        m.insert("Token usage", "Token 用量");
        m.insert("Context window", "上下文窗口");
        m.insert("Limits", "限制");
        m.insert("Warning", "警告");
        m.insert("Read Only", "只读");
        m.insert("Read Only with network access", "只读（网络访问）");
        m.insert("Workspace", "工作区");
        m.insert("Workspace with network access", "工作区（网络访问）");
        m.insert("Full Access", "完全访问");
        m.insert("Custom permissions", "自定义权限");
        m.insert("Approve for me", "自动审批");
        m.insert("Ask for approval", "请求审批");
        m.insert("Remote", "远程");
        m.insert("information on rate limits and credits", "关于速率限制和额度的信息");

        // === Footer / Mode labels (footer.rs) ===
        m.insert("Plan mode", "计划模式");
        m.insert("Pair Programming mode", "结对编程模式");
        m.insert("Execute mode", "执行模式");
        m.insert("Pursuing goal", "推进目标中");
        m.insert("Goal paused", "目标已暂停");
        m.insert("Goal stalled", "目标已阻塞");
        m.insert("Goal hit usage limits", "目标达到用量限制");
        m.insert("Goal achieved", "目标已达成");
        m.insert("Goal abandoned", "目标已放弃");
        m.insert("Goal unmet", "目标未达成");

        // === Welcome screen ===
        m.insert("Welcome to ", "欢迎使用 ");
        m.insert(", OpenAI's command-line coding agent", "，OpenAI 的命令行编码助手");

        // === Placeholder texts ===
        m.insert("Ask Codex to do anything", "向 Codex 询问任何事情");
        m.insert("Type to search skills", "输入以搜索技能");
        m.insert("Type your answer", "输入您的回答");
        m.insert("Type your answer (optional)", "输入您的回答（可选）");
        m.insert("Type to search", "输入以搜索");
        m.insert("Add notes", "添加备注");
        m.insert("None of the above", "以上都不是");
        m.insert("Submit with unanswered questions?", "提交未回答的问题？");
        m.insert("Go back", "返回");
        m.insert("Proceed", "继续");

        // === Plan implementation ===
        m.insert("Implement this plan?", "实施此计划？");
        m.insert("Yes, implement this plan", "是，实施此计划");
        m.insert("Yes, clear context and implement", "是，清除上下文并实施");
        m.insert("No, stay in Plan mode", "否，保留在计划模式");
        m.insert("Default mode unavailable", "默认模式不可用");
        m.insert("No approved plan available", "无已批准的计划");

        // === Settings ===
        m.insert("Select Personality", "选择个性");
        m.insert("None", "无");
        m.insert("Friendly", "友好");
        m.insert("Pragmatic", "务实");
        m.insert("Select Model", "选择模型");
        m.insert("Select Model and Effort", "选择模型和推理程度");
        m.insert("All models", "所有模型");
        m.insert("Configure Terminal Title", "配置终端标题");
        m.insert("Enable/Disable Skills", "启用/禁用技能");

        // === Reasoning effort ===
        m.insert("Minimal", "最低");
        m.insert("Low", "低");
        m.insert("Medium", "中");
        m.insert("High", "高");
        m.insert("Extra high", "极高");
        m.insert("Max", "最高");
        m.insert("Ultra", "极致");

        // === Update prompt ===
        m.insert("Skip", "跳过");
        m.insert("Skip until next version", "跳过直到下一版本");

        // === Notifications / status ===
        m.insert("not available for this account", "当前账户不可用");
        m.insert("data not available yet", "数据尚不可用");
        m.insert("API key configured (run codex login to use ChatGPT)", "已配置 API Key（运行 codex login 以使用 ChatGPT）");

        // === Popup hints ===
        m.insert("Press ", "按 ");
        m.insert(" to confirm", " 确认");
        m.insert(" to go back", " 返回");
        m.insert(" to toggle", " 切换");

        // === Goal display ===
        m.insert("active", "活跃中");
        m.insert("paused", "已暂停");
        m.insert("blocked", "已阻塞");
        m.insert("usage limited", "用量限制");
        m.insert("limited by budget", "预算限制");
        m.insert("complete", "已完成");

        // === MCP ===
        m.insert("Booting MCP server:", "启动 MCP 服务器：");
        m.insert("Starting MCP servers", "启动 MCP 服务器");

        // === Status line items ===
        m.insert("Current model name", "当前模型名称");
        m.insert("Current model name with reasoning level", "模型名称（含推理程度）");
        m.insert("Current reasoning level", "当前推理程度");
        m.insert("Current working directory", "当前工作目录");
        m.insert("Active permission profile or sandbox mode", "活跃权限配置或沙箱模式");
        m.insert("Active command approval mode", "活跃命令审批模式");
        m.insert("Codex application version", "Codex 应用版本");
        m.insert("Total input tokens used in session", "会话中使用的输入 Token 总数");
        m.insert("Total output tokens used in session", "会话中使用的输出 Token 总数");
        m.insert("Whether Fast mode is currently active", "快速模式是否启用");
        m.insert("Codex app name", "Codex 应用名称");
        m.insert("Current thread title, or thread identifier when unnamed", "当前会话标题（无标题时显示会话 ID）");

        // === Permission labels ===
        m.insert("Update Model Permissions", "更新模型权限");

        // === Multi-agents ===
        m.insert("Waiting for agents", "等待子智能体");
        m.insert("Waiting for", "等待");

        // === API key configuration ===
        m.insert("Visit ", "访问 ");
        m.insert(" for up-to-date", " 获取最新的");

        // === Response to tool / user input ===
        m.insert("Answer the questions to continue.", "回答问题以继续。");
        m.insert("Respond to the tool suggestion to continue.", "响应工具建议以继续。");
        m.insert("Respond to the MCP server request to continue.", "响应 MCP 服务器请求以继续。");
        m.insert("Install this app in your browser, then return here.", "在浏览器中安装此应用，然后返回此处。");
        m.insert("Enable this app to use it for the current request.", "启用此应用以用于当前请求。");

        // === Search ===
        m.insert("no matches", "无匹配");

        // === Keyboard hints ===
        m.insert("alt + ", "Alt + ");
        m.insert("ctrl + ", "Ctrl + ");
        m.insert("shift + ", "Shift + ");

        // === Transcript ===
        m.insert("ctrl + t to view transcript", "Ctrl + T 查看对话记录");

        // === Side / agent mode ===
        m.insert("Side ", "子智能体 ");

        // === Multi-agents ===
        m.insert("Waiting for agents", "等待子智能体");
        m.insert("Waiting for", "等待");

        // === "no reasoning" / reasoning ===
        m.insert("no reasoning", "无推理");
        m.insert("the selected reasoning", "选定的推理程度");
        m.insert("the default reasoning", "默认推理程度");
        m.insert("no reasoning effort configured", "未配置推理程度");

        // === Permission presets ===
        m.insert("Full Access", "完全访问");
        m.insert("No Sandbox", "无沙箱");
        m.insert("Custom permissions", "自定义权限");

        // === Service tiers ===
        m.insert("built-in Plan default", "内置计划默认值");
        m.insert("default", "默认");

        // === Side agent ===
        m.insert("Side ", "子智能体 ");

        // === Collaboration mode ===
        m.insert("Plan", "计划");
        m.insert("Execute", "执行");
        m.insert("Pair Programming", "结对编程");

        // === Goal status ===
        m.insert("active", "活跃中");
        m.insert("paused", "已暂停");
        m.insert("blocked", "已阻塞");
        m.insert("usage limited", "用量已达上限");
        m.insert("limited by budget", "已超出预算限制");
        m.insert("complete", "已完成");

        // === Transcript hint ===
        m.insert("ctrl + t to view transcript", "Ctrl + T 查看对话记录");

        m
    });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_fallback() {
        set_language(Language::En);
        assert_eq!(lookup("Working"), "Working");
        assert_eq!(lookup("NonExistentKey"), "NonExistentKey");
    }

    #[test]
    fn test_chinese_translation() {
        set_language(Language::Zh);
        assert_eq!(lookup("Working"), "执行中");
        assert_eq!(lookup("Thinking"), "思考中");
        assert_eq!(lookup("Ready"), "就绪");
        assert_eq!(lookup("Plan mode"), "计划模式");
    }

    #[test]
    fn test_fallback_for_missing_key() {
        set_language(Language::Zh);
        // Keys not in the translation map should return the key itself
        assert_eq!(lookup("SomeNewFeature"), "SomeNewFeature");
    }

    #[test]
    fn test_macro() {
        set_language(Language::Zh);
        assert_eq!(tr!("Working"), "执行中");
        assert_eq!(tr!("Thinking"), "思考中");
    }

    #[test]
    fn test_refresh_language() {
        set_language(Language::En);
        assert_eq!(lookup("Working"), "Working");
        set_language(Language::Zh);
        assert_eq!(lookup("Working"), "执行中");
    }

    #[test]
    fn test_keyboard_hints_translated() {
        set_language(Language::Zh);
        assert_eq!(lookup("alt + "), "Alt + ");
        assert_eq!(lookup("ctrl + "), "Ctrl + ");
    }

    #[test]
    fn test_goal_status_translated() {
        set_language(Language::Zh);
        assert_eq!(lookup("active"), "活跃中");
        assert_eq!(lookup("paused"), "已暂停");
        assert_eq!(lookup("complete"), "已完成");
    }
}
