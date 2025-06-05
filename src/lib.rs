use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use zed_extension_api::{self as zed, settings::ContextServerSettings};

struct RoverContextServer;

#[derive(Debug, Deserialize, JsonSchema, Serialize, Default)]
struct RoverContextServerSettings {
    api_key: String,
    host: Option<String>,
}

const DEFAULT_HOST: &str = "https://api.getrover.com";
const PACKAGE_NAME: &str = "@getrover/mcp-cli";
const SERVER_PATH: &str = "node_modules/@getrover/mcp-cli/dist/index.js";

impl zed::Extension for RoverContextServer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed_extension_api::ContextServerId,
        project: &zed::Project,
    ) -> zed_extension_api::Result<zed::Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let settings = ContextServerSettings::for_project("rover", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `api_key` setting".into());
        };
        let settings: RoverContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                std::env::current_dir()
                    .unwrap()
                    .join(SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
            ],
            env: vec![
                ("ROVER_API_KEY".into(), settings.api_key),
                (
                    "ROVER_HOST".into(),
                    settings.host.unwrap_or(DEFAULT_HOST.to_string()),
                ),
            ],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &zed_extension_api::ContextServerId,
        _project: &zed_extension_api::Project,
    ) -> zed_extension_api::Result<Option<zed_extension_api::ContextServerConfiguration>> {
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(RoverContextServerSettings))
                .map_err(|e| e.to_string())?;
        Ok(Some(zed_extension_api::ContextServerConfiguration {
            settings_schema,
            default_settings: serde_json::to_string(&RoverContextServerSettings::default())
                .map_err(|e| e.to_string())?,
            installation_instructions: "You'll need to provide your Rover API key. You can generate a key in your user or organization settings.".to_string()
        }))
    }
}

zed::register_extension!(RoverContextServer);
