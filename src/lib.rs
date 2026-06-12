use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed::{serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result};
use zed_extension_api as zed;

const CONTEXT_SERVER_ID: &str = "atlassian-rovo";
const DEFAULT_ATLASSIAN_MCP_URL: &str = "https://mcp.atlassian.com/v1/mcp/authv2";
const DEFAULT_MCP_REMOTE_PACKAGE: &str = "mcp-remote@latest";

#[derive(Debug, Deserialize, JsonSchema)]
struct AtlassianRovoSettings {
    /// Atlassian Rovo MCP endpoint.
    #[serde(default = "default_atlassian_mcp_url")]
    url: String,

    /// npm package spec for the local remote MCP proxy.
    #[serde(default = "default_mcp_remote_package")]
    mcp_remote_package: String,

    /// Optional extra arguments passed to mcp-remote before the Atlassian URL.
    #[serde(default)]
    extra_args: Vec<String>,

    /// npm cache directory used by npx. A local cache avoids broken user-level npm caches.
    #[serde(default = "default_npm_cache")]
    npm_cache: String,
}

impl Default for AtlassianRovoSettings {
    fn default() -> Self {
        Self {
            url: default_atlassian_mcp_url(),
            mcp_remote_package: default_mcp_remote_package(),
            extra_args: Vec::new(),
            npm_cache: default_npm_cache(),
        }
    }
}

fn default_atlassian_mcp_url() -> String {
    DEFAULT_ATLASSIAN_MCP_URL.to_string()
}

fn default_mcp_remote_package() -> String {
    DEFAULT_MCP_REMOTE_PACKAGE.to_string()
}

fn default_npm_cache() -> String {
    ".npm-cache".to_string()
}

struct AtlassianRovoExtension;

impl AtlassianRovoExtension {
    fn settings(project: &Project) -> Result<AtlassianRovoSettings> {
        let settings = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let Some(settings) = settings.settings else {
            return Ok(AtlassianRovoSettings::default());
        };

        serde_json::from_value(settings).map_err(|e| format!("failed to parse settings: {e}"))
    }
}

impl zed::Extension for AtlassianRovoExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = Self::settings(project)?;

        let mut args = vec![
            "npx".to_string(),
            "-y".to_string(),
            settings.mcp_remote_package,
        ];
        args.extend(settings.extra_args);
        args.push(settings.url);

        let env = if settings.npm_cache.is_empty() {
            vec![]
        } else {
            vec![("NPM_CONFIG_CACHE".to_string(), settings.npm_cache)]
        };

        Ok(Command {
            command: "/usr/bin/env".to_string(),
            args,
            env,
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
        let settings_schema = serde_json::to_string(&schemars::schema_for!(AtlassianRovoSettings))
            .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(AtlassianRovoExtension);
