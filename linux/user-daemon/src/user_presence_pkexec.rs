use std::collections::HashMap;
use std::process::Command;
use std::io;

use async_trait::async_trait;
use lazy_static::lazy_static;
use tracing::debug;
use u2f_core::{try_reverse_app_id, AppId, UserPresence};

pub struct PolkitUserPresence;

impl PolkitUserPresence {
    pub fn new() -> Self {
        PolkitUserPresence
    }

    async fn test_user_presence(&self, message: String) -> Result<bool, io::Error> {
        debug!(%message, "test_user_presence_pkexec");

        let output = Command::new("pkexec")
            .arg("echo")
            .output()
            .expect("Failed to execute command");

        let user_present = output.status.success();

        Ok(user_present)
    }
}

#[async_trait]
impl UserPresence for PolkitUserPresence {
    async fn approve_registration(&self, application: &AppId) -> Result<bool, io::Error> {
        let site_name = try_reverse_app_id(application).unwrap_or(String::from("site"));
        let message = format!("Register {}", site_name);
        self.test_user_presence(message).await
    }

    async fn approve_authentication(&self, application: &AppId) -> Result<bool, io::Error> {
        let site_name = try_reverse_app_id(application).unwrap_or(String::from("site"));
        let message = format!("Authenticate {}", site_name);
        self.test_user_presence(message).await
    }

    async fn wink(&self) -> Result<(), io::Error> {
        Ok(())
    }
}
