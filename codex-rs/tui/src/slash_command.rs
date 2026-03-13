use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Fast,
    Approvals,
    Permissions,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    #[strum(serialize = "sandbox-add-read-dir")]
    SandboxReadRoot,
    Experimental,
    Skills,
    Review,
    Rename,
    New,
    Resume,
    Fork,
    Init,
    Compact,
    Plan,
    Collab,
    Agent,
    // Undo,
    Diff,
    Copy,
    Mention,
    Status,
    DebugConfig,
    Statusline,
    Theme,
    Mcp,
    Apps,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    Clean,
    Clear,
    Personality,
    Realtime,
    Settings,
    TestApproval,
    MultiAgents,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self, brand_name: &str) -> String {
        match self {
            SlashCommand::Feedback => "send logs to maintainers".to_string(),
            SlashCommand::New => "start a new chat during a conversation".to_string(),
            SlashCommand::Init => format!("create an AGENTS.md file with instructions for {brand_name}"),
            SlashCommand::Compact => "summarize conversation to prevent hitting the context limit".to_string(),
            SlashCommand::Review => "review my current changes and find issues".to_string(),
            SlashCommand::Rename => "rename the current thread".to_string(),
            SlashCommand::Resume => "resume a saved chat".to_string(),
            SlashCommand::Clear => "clear the terminal and start a new chat".to_string(),
            SlashCommand::Fork => "fork the current chat".to_string(),
            SlashCommand::Quit | SlashCommand::Exit => format!("exit {brand_name}"),
            SlashCommand::Diff => "show git diff (including untracked files)".to_string(),
            SlashCommand::Copy => format!("copy the latest {brand_name} output to your clipboard"),
            SlashCommand::Mention => "mention a file".to_string(),
            SlashCommand::Skills => format!("use skills to improve how {brand_name} performs specific tasks"),
            SlashCommand::Status => "show current session configuration and token usage".to_string(),
            SlashCommand::DebugConfig => "show config layers and requirement sources for debugging".to_string(),
            SlashCommand::Statusline => "configure which items appear in the status line".to_string(),
            SlashCommand::Theme => "choose a syntax highlighting theme".to_string(),
            SlashCommand::Ps => "list background terminals".to_string(),
            SlashCommand::Clean => "stop all background terminals".to_string(),
            SlashCommand::MemoryDrop => "DO NOT USE".to_string(),
            SlashCommand::MemoryUpdate => "DO NOT USE".to_string(),
            SlashCommand::Model => "choose what model and reasoning effort to use".to_string(),
            SlashCommand::Fast => "toggle Fast mode to enable fastest inference at 2X plan usage".to_string(),
            SlashCommand::Personality => format!("choose a communication style for {brand_name}"),
            SlashCommand::Realtime => "toggle realtime voice mode (experimental)".to_string(),
            SlashCommand::Settings => "configure realtime microphone/speaker".to_string(),
            SlashCommand::Plan => "switch to Plan mode".to_string(),
            SlashCommand::Collab => "change collaboration mode (experimental)".to_string(),
            SlashCommand::Agent | SlashCommand::MultiAgents => "switch the active agent thread".to_string(),
            SlashCommand::Approvals => format!("choose what {brand_name} is allowed to do"),
            SlashCommand::Permissions => format!("choose what {brand_name} is allowed to do"),
            SlashCommand::ElevateSandbox => "set up elevated agent sandbox".to_string(),
            SlashCommand::SandboxReadRoot => {
                "let sandbox read a directory: /sandbox-add-read-dir <absolute_path>".to_string()
            }
            SlashCommand::Experimental => "toggle experimental features".to_string(),
            SlashCommand::Mcp => "list configured MCP tools".to_string(),
            SlashCommand::Apps => "manage apps".to_string(),
            SlashCommand::Logout => format!("log out of {brand_name}"),
            SlashCommand::Rollout => "print the rollout file path".to_string(),
            SlashCommand::TestApproval => "test approval request".to_string(),
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Rename
                | SlashCommand::Plan
                | SlashCommand::Fast
                | SlashCommand::SandboxReadRoot
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Resume
            | SlashCommand::Fork
            | SlashCommand::Init
            | SlashCommand::Compact
            // | SlashCommand::Undo
            | SlashCommand::Model
            | SlashCommand::Fast
            | SlashCommand::Personality
            | SlashCommand::Approvals
            | SlashCommand::Permissions
            | SlashCommand::ElevateSandbox
            | SlashCommand::SandboxReadRoot
            | SlashCommand::Experimental
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Clear
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Copy
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Status
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Clean
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Feedback
            | SlashCommand::Quit
            | SlashCommand::Exit => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Realtime => true,
            SlashCommand::Settings => true,
            SlashCommand::Collab => true,
            SlashCommand::Agent | SlashCommand::MultiAgents => true,
            SlashCommand::Statusline => false,
            SlashCommand::Theme => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::SandboxReadRoot => cfg!(target_os = "windows"),
            SlashCommand::Copy => !cfg!(target_os = "android"),
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}
