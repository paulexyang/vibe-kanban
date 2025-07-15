use std::{collections::HashMap, sync::Arc, time::Duration};

#[cfg(unix)]
use nix::{sys::signal::Signal, unistd::Pid};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::services::generate_user_id;

#[derive(Debug)]
pub enum ExecutionType {
    SetupScript,
    CodingAgent,
    DevServer,
}

#[derive(Debug)]
pub struct RunningExecution {
    pub task_attempt_id: Uuid,
    pub _execution_type: ExecutionType,
    pub child: command_group::AsyncGroupChild,
}

#[derive(Debug, Clone)]
pub struct AppState {
    running_executions: Arc<Mutex<HashMap<Uuid, RunningExecution>>>,
    pub db_pool: sqlx::SqlitePool,
    config: Arc<tokio::sync::RwLock<crate::models::config::Config>>,
    user_id: String,
}

impl AppState {
    pub async fn new(
        db_pool: sqlx::SqlitePool,
        config: Arc<tokio::sync::RwLock<crate::models::config::Config>>,
    ) -> Self {
        Self {
            running_executions: Arc::new(Mutex::new(HashMap::new())),
            db_pool,
            config,
            user_id: generate_user_id(),
        }
    }

    // Running executions getters
    pub async fn has_running_execution(&self, attempt_id: Uuid) -> bool {
        let executions = self.running_executions.lock().await;
        executions
            .values()
            .any(|exec| exec.task_attempt_id == attempt_id)
    }

    pub async fn get_running_executions_for_monitor(&self) -> Vec<(Uuid, Uuid, bool, Option<i64>)> {
        let mut executions = self.running_executions.lock().await;
        let mut completed_executions = Vec::new();

        for (execution_id, running_exec) in executions.iter_mut() {
            match running_exec.child.try_wait() {
                Ok(Some(status)) => {
                    let success = status.success();
                    let exit_code = status.code().map(|c| c as i64);
                    completed_executions.push((
                        *execution_id,
                        running_exec.task_attempt_id,
                        success,
                        exit_code,
                    ));
                }
                Ok(None) => {
                    // Still running
                }
                Err(e) => {
                    tracing::error!("Error checking process status: {}", e);
                    completed_executions.push((
                        *execution_id,
                        running_exec.task_attempt_id,
                        false,
                        None,
                    ));
                }
            }
        }

        // Remove completed executions from the map
        for (execution_id, _, _, _) in &completed_executions {
            executions.remove(execution_id);
        }

        completed_executions
    }

    // Running executions setters
    pub async fn add_running_execution(&self, execution_id: Uuid, execution: RunningExecution) {
        let mut executions = self.running_executions.lock().await;
        executions.insert(execution_id, execution);
    }

    pub async fn stop_running_execution_by_id(
        &self,
        execution_id: Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut executions = self.running_executions.lock().await;
        let Some(exec) = executions.get_mut(&execution_id) else {
            return Ok(false);
        };

        // hit the whole process group, not just the leader
        #[cfg(unix)]
        {
            use nix::{sys::signal::killpg, unistd::getpgid};

            let pgid = getpgid(Some(Pid::from_raw(exec.child.id().unwrap() as i32)))?;
            for sig in [Signal::SIGINT, Signal::SIGTERM, Signal::SIGKILL] {
                killpg(pgid, sig)?;
                tokio::time::sleep(Duration::from_secs(2)).await;
                if exec.child.try_wait()?.is_some() {
                    break; // gone!
                }
            }
        }

        // final fallback – command_group already targets the group
        exec.child.kill().await.ok();
        exec.child.wait().await.ok(); // reap

        // only NOW remove it
        executions.remove(&execution_id);
        Ok(true)
    }

    // Config getters
    pub async fn get_sound_alerts_enabled(&self) -> bool {
        let config = self.config.read().await;
        config.sound_alerts
    }

    pub async fn get_push_notifications_enabled(&self) -> bool {
        let config = self.config.read().await;
        config.push_notifications
    }

    pub async fn get_sound_file(&self) -> crate::models::config::SoundFile {
        let config = self.config.read().await;
        config.sound_file.clone()
    }

    pub fn get_config(&self) -> &Arc<tokio::sync::RwLock<crate::models::config::Config>> {
        &self.config
    }

    pub async fn update_sentry_scope(&self) {
        let config = self.get_config().read().await;
        let username = config.github.username.clone();
        let email = config.github.primary_email.clone();
        drop(config);

        let sentry_user = if username.is_some() || email.is_some() {
            sentry::User {
                id: Some(self.user_id.clone()),
                username,
                email,
                ..Default::default()
            }
        } else {
            sentry::User {
                id: Some(self.user_id.clone()),
                ..Default::default()
            }
        };

        sentry::configure_scope(|scope| {
            scope.set_user(Some(sentry_user));
        });
    }
}
