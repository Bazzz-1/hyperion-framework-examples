// -------------------------------------------------------------------------------------------------
// Hyperion Framework Example 1
// https://github.com/Bazzz-1/hyperion-framework-examples
// https://github.com/Bazzz-1/hyperion-framework
//
// A lightweight Rust framework for building modular, component-based systems
// with built-in TCP messaging and CLI control.
//
// Example written by Robert Hannah 2025
// -------------------------------------------------------------------------------------------------

// Standard
use std::collections::HashMap;

// Package
use hyperion_framework::containerisation::traits::{ContainerIdentidy, LogLevel};
use serde::Deserialize;


// Top level configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub container: Container,
    pub logging: Logging,
    pub component: Component
}

// Container configuration
#[derive(Debug, Deserialize)]
pub struct Container {
    pub name: String,
    pub version: String,
    pub version_title: String,
    pub software_collection: String
}

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub level: String
}

// Traits
impl ContainerIdentidy for Config {
    fn container_identity(&self) -> HashMap<String, String> {
        let mut identity = HashMap::new();
        identity.insert("name".to_string(), self.container.name.clone());
        identity.insert("version".to_string(), self.container.version.clone());
        identity.insert("version_title".to_string(), self.container.version_title.clone());
        identity.insert("software_collection".to_string(), self.container.software_collection.clone());
        identity
    }
}

impl LogLevel for Config {
    fn log_level(&self) -> &str {
        &self.logging.level
    }
}

// Component configuration
#[derive(Debug, Deserialize)]
pub struct Component {
    pub increment: u8
}
