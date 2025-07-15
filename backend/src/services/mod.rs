pub mod git_service;
pub mod github_service;
pub mod notification_service;
pub mod pr_monitor;
pub mod process_service;

pub use git_service::{GitService, GitServiceError};
pub use github_service::{CreatePrRequest, GitHubRepoInfo, GitHubService, GitHubServiceError};
pub use notification_service::{NotificationConfig, NotificationService};
pub use pr_monitor::PrMonitorService;
pub use process_service::ProcessService;

/// Returns a dummy user ID
pub fn generate_user_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
