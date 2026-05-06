// common/src/config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallConfig {
    pub app: AppSection,
    pub install: InstallSection,
    #[serde(default)]
    pub ui: UiSection,
    #[serde(default)]
    pub files: Vec<FileSpec>,
    #[serde(default)]
    pub shortcuts: Vec<ShortcutSpec>,
    #[serde(default)]
    pub registry: Vec<RegistryKeySpec>,
    #[serde(default)]
    pub run: Vec<RunSpec>,
}

impl InstallConfig {
    pub fn parse(input: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(input)
    }

    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = Vec::new();

        if self.app.name.trim().is_empty() {
            errors.push(ValidationError::new("app.name", "must not be empty"));
        }
        if self.app.version.trim().is_empty() {
            errors.push(ValidationError::new("app.version", "must not be empty"));
        }
        if self.install.default_dir.trim().is_empty() {
            errors.push(ValidationError::new("install.default_dir", "must not be empty"));
        }
        if self.files.is_empty() {
            errors.push(ValidationError::new("files", "must contain at least one [[files]] entry"));
        }

        for (index, file) in self.files.iter().enumerate() {
            if file.src.trim().is_empty() {
                errors.push(ValidationError::new(format!("files[{index}].src"), "must not be empty"));
            }
            if file.dest.trim().is_empty() {
                errors.push(ValidationError::new(format!("files[{index}].dest"), "must not be empty"));
            }
        }

        for (index, shortcut) in self.shortcuts.iter().enumerate() {
            if shortcut.name.trim().is_empty() {
                errors.push(ValidationError::new(format!("shortcuts[{index}].name"), "must not be empty"));
            }
            if shortcut.target.trim().is_empty() {
                errors.push(ValidationError::new(format!("shortcuts[{index}].target"), "must not be empty"));
            }
        }

        for (index, key) in self.registry.iter().enumerate() {
            if key.key.trim().is_empty() {
                errors.push(ValidationError::new(format!("registry[{index}].key"), "must not be empty"));
            }
            for (value_index, value) in key.values.iter().enumerate() {
                if value.name.trim().is_empty() {
                    errors.push(ValidationError::new(
                        format!("registry[{index}].values[{value_index}].name"),
                        "must not be empty",
                    ));
                }
            }
        }

        for (index, item) in self.run.iter().enumerate() {
            if item.cmd.trim().is_empty() {
                errors.push(ValidationError::new(format!("run[{index}].cmd"), "must not be empty"));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors(errors))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSection {
    pub name: String,
    pub version: String,
    pub publisher: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallSection {
    pub default_dir: String,
    #[serde(default)]
    pub add_to_path: bool,
    #[serde(default)]
    pub create_desktop_shortcut: bool,
    #[serde(default)]
    pub require_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiSection {
    #[serde(default)]
    pub theme: UiTheme,
    pub accent_color: Option<String>,
    pub welcome_text: Option<String>,
    pub license_file: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum UiTheme {
    Dark,
    Light,
    #[default]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSpec {
    pub src: String,
    pub dest: String,
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutSpec {
    pub name: String,
    pub target: String,
    #[serde(default)]
    pub args: String,
    #[serde(default)]
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryKeySpec {
    pub key: String,
    #[serde(default)]
    pub values: Vec<RegistryValueSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryValueSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub value_type: RegistryValueType,
    pub data: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistryValueType {
    String,
    Dword,
    Qword,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSpec {
    pub cmd: String,
    #[serde(default)]
    pub args: String,
    #[serde(default)]
    pub when: RunWhen,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RunWhen {
    #[default]
    After,
    Finish,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug)]
pub struct ValidationErrors(pub Vec<ValidationError>);

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "configuration validation failed:")?;
        for error in &self.0 {
            writeln!(f, "- {}: {}", error.path, error.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}
