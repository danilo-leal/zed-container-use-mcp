use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use zed_extension_api::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

#[derive(Debug, Deserialize, JsonSchema)]
struct ContainerUseMcpSettings {
    /// Path to the cu binary (optional - will download if not provided)
    cu_path: Option<String>,
}

struct ContainerUseMcpExtension {
    cached_binary_path: Option<String>,
}

impl ContainerUseMcpExtension {
    fn context_server_binary_path(
        &mut self,
        _context_server_id: &ContextServerId,
    ) -> Result<String> {
        // First check if user provided a custom path
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // Try to find cu in PATH
        let output = zed::process::Command::new("which")
            .arg("cu")
            .output()
            .map_err(|e| format!("Error finding cu in PATH: {}", e))?;

        let path = String::from_utf8(output.stdout)
            .map_err(|e| format!("Error parsing cu path: {}", e))?;
        let path = path.trim().to_string();

        if !path.is_empty() {
            self.cached_binary_path = Some(path.clone());
            Ok(path)
        } else {
            Err("Failed to find `cu` path. Follow the instructions in README.md".to_string())
        }
    }
}

impl zed::Extension for ContainerUseMcpExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("container-use-mcp", project)?;

        if let Some(settings) = settings.settings {
            if let Ok(custom_settings) = serde_json::from_value::<ContainerUseMcpSettings>(settings)
            {
                if let Some(cu_path) = custom_settings.cu_path {
                    self.cached_binary_path = Some(cu_path);
                }
            }
        }

        Ok(Command {
            command: self.context_server_binary_path(context_server_id)?,
            args: vec!["stdio".into()],
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(ContainerUseMcpSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ContainerUseMcpExtension);
