use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeInstance {
    pub id: Uuid,
    pub role: InstanceRole,
    pub status: InstanceStatus,
    pub current_task: Option<Task>,
    pub capabilities: Vec<String>,
    pub config: InstanceConfig,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    pub claude_code_path: String,
    pub workspace_dir: String,
    pub max_concurrent_tasks: u32,
    pub timeout_seconds: u64,
    pub auto_accept_tasks: bool,
    pub preferred_languages: Vec<String>,
    pub custom_prompts: HashMap<String, String>,
    pub environment_vars: HashMap<String, String>,
}

impl Default for InstanceConfig {
    fn default() -> Self {
        Self {
            name: "Claude Instance".to_string(),
            claude_code_path: "claude".to_string(),
            workspace_dir: ".".to_string(),
            max_concurrent_tasks: 1,
            timeout_seconds: 300,
            auto_accept_tasks: true,
            preferred_languages: vec!["rust".to_string(), "typescript".to_string()],
            custom_prompts: HashMap::new(),
            environment_vars: HashMap::new(),
        }
    }
}

impl InstanceConfig {
    pub fn for_role(role: InstanceRole) -> Self {
        let mut config = Self::default();
        match role {
            InstanceRole::ProjectManager => {
                config.name = "Project Manager".to_string();
                config.preferred_languages = vec!["markdown".to_string(), "yaml".to_string(), "json".to_string()];
                config.max_concurrent_tasks = 5;
                config.timeout_seconds = 600;
                config.custom_prompts.insert("role_prompt".to_string(), 
                    "You are a project manager working on the current project directory. Take high-level requirements and break them down into specific, actionable tasks for team members. Assign tasks to the most appropriate team members based on their roles and expertise. Focus on project planning, task coordination, and ensuring deliverables are met. Auto-accept and implement all changes and suggestions.".to_string());
                config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
            }
            InstanceRole::Supervisor => {
                config.name = "Team Supervisor".to_string();
                config.preferred_languages = vec!["rust".to_string(), "typescript".to_string(), "python".to_string()];
                config.max_concurrent_tasks = 3;
                config.custom_prompts.insert("role_prompt".to_string(), 
                    "You are a team supervisor working on the current project directory. Focus on architecture, code review, and coordination. Auto-accept and implement all changes and suggestions.".to_string());
                config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
            }
            InstanceRole::Developer => {
                config.name = "Developer".to_string();
                config.preferred_languages = vec!["rust".to_string(), "typescript".to_string()];
                config.max_concurrent_tasks = 2;
                config.custom_prompts.insert("role_prompt".to_string(), 
                    "You are a developer working on the current project directory. Focus on implementing features and fixing bugs. Auto-accept and implement all changes and suggestions.".to_string());
                config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
            }
            InstanceRole::Tester => {
                config.name = "QA Tester".to_string();
                config.preferred_languages = vec!["javascript".to_string(), "typescript".to_string()];
                config.max_concurrent_tasks = 2;
                config.custom_prompts.insert("role_prompt".to_string(), 
                    "You are a QA tester working on the current project directory. Focus on writing tests and ensuring quality. Auto-accept and implement all changes and suggestions.".to_string());
                config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
            }
            InstanceRole::Reviewer => {
                config.name = "Code Reviewer".to_string();
                config.preferred_languages = vec!["rust".to_string(), "typescript".to_string(), "python".to_string()];
                config.max_concurrent_tasks = 1;
                config.auto_accept_tasks = true;
                config.custom_prompts.insert("role_prompt".to_string(), 
                    "You are a code reviewer working on the current project directory. Focus on security, best practices, and code quality. Auto-accept and implement suggested changes.".to_string());
                config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
            }
            InstanceRole::Researcher => {
                config.name = "Researcher".to_string();
                config.preferred_languages = vec!["markdown".to_string(), "python".to_string()];
                config.max_concurrent_tasks = 1;
                config.timeout_seconds = 600;
                config.custom_prompts.insert("role_prompt".to_string(), 
                    "You are a researcher working on the current project directory. Focus on analysis, documentation, and investigation. Auto-accept and implement all changes and suggestions.".to_string());
                config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
            }
        }
        config
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub task_type: TaskType,
    pub description: String,
    pub assigned_to: Option<Uuid>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub dependencies: Vec<Uuid>,
    pub result: Option<TaskResult>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
    pub files_modified: Vec<String>,
    pub tests_run: Vec<TestResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceRole {
    ProjectManager,
    Supervisor,
    Developer,
    Tester,
    Reviewer,
    Researcher,
}

impl InstanceRole {
    #[allow(dead_code)]
    pub fn capabilities(&self) -> Vec<String> {
        match self {
            Self::ProjectManager => vec![
                "task-planning".to_string(),
                "project-management".to_string(),
                "requirement-analysis".to_string(),
                "task-breakdown".to_string(),
                "team-coordination".to_string(),
                "strategic-planning".to_string(),
            ],
            Self::Supervisor => vec![
                "architecture".to_string(),
                "project-management".to_string(),
                "code-review".to_string(),
                "typescript".to_string(),
            ],
            Self::Developer => vec![
                "typescript".to_string(),
                "rust".to_string(),
                "node.js".to_string(),
                "programming".to_string(),
            ],
            Self::Tester => vec![
                "testing".to_string(),
                "jest".to_string(),
                "integration-testing".to_string(),
                "quality-assurance".to_string(),
            ],
            Self::Reviewer => vec![
                "code-review".to_string(),
                "quality-assurance".to_string(),
                "security".to_string(),
                "best-practices".to_string(),
            ],
            Self::Researcher => vec![
                "analysis".to_string(),
                "documentation".to_string(),
                "research".to_string(),
                "optimization".to_string(),
            ],
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectManager => "project_manager",
            Self::Supervisor => "supervisor",
            Self::Developer => "developer",
            Self::Tester => "tester",
            Self::Reviewer => "reviewer",
            Self::Researcher => "researcher",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceStatus {
    Idle,
    Working,
    Error,
    Offline,
}

impl InstanceStatus {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Working => "working", 
            Self::Error => "error",
            Self::Offline => "offline",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    ProjectPlanning,
    CodeReview,
    FeatureImplementation,
    BugFix,
    TestCreation,
    Research,
    Documentation,
}

impl TaskType {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectPlanning => "project_planning",
            Self::CodeReview => "code_review",
            Self::FeatureImplementation => "feature_implementation",
            Self::BugFix => "bug_fix",
            Self::TestCreation => "test_creation",
            Self::Research => "research",
            Self::Documentation => "documentation",
        }
    }

    #[allow(dead_code)]
    pub fn preferred_roles(&self) -> Vec<InstanceRole> {
        match self {
            Self::ProjectPlanning => vec![InstanceRole::ProjectManager, InstanceRole::Supervisor],
            Self::CodeReview => vec![InstanceRole::Reviewer, InstanceRole::Supervisor],
            Self::TestCreation => vec![InstanceRole::Tester, InstanceRole::Developer],
            Self::Research => vec![InstanceRole::Researcher, InstanceRole::Supervisor],
            Self::FeatureImplementation | Self::BugFix => {
                vec![InstanceRole::Developer, InstanceRole::Supervisor]
            }
            Self::Documentation => vec![InstanceRole::Researcher, InstanceRole::Developer],
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl TaskPriority {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high", 
            Self::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub from: String,
    pub to: String,
    pub message_type: MessageType,
    pub content: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    TaskAssignment,
    TaskCompletion,
    StatusUpdate,
    CollaborationRequest,
    ErrorReport,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OrchestratorConfig {
    pub redis_url: String,
    pub claude_code_path: String,
    pub workspaces_dir: String,
    pub task_timeout_seconds: u64,
    pub log_level: String,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://localhost:6379".to_string(),
            claude_code_path: "claude".to_string(),
            workspaces_dir: "./workspaces".to_string(),
            task_timeout_seconds: 300,
            log_level: "info".to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum BorgError {
    #[error("Instance not found: {id}")]
    InstanceNotFound { id: Uuid },
    
    #[error("Task not found: {id}")]
    TaskNotFound { id: Uuid },
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Process error: {message}")]
    Process { message: String },
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Timeout error: {message}")]
    Timeout { message: String },
    
    #[error("Task execution error: {message}")]
    TaskExecutionError { message: String },
}

pub type BorgResult<T> = Result<T, BorgError>;