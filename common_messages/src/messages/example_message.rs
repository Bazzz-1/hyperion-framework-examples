// -------------------------------------------------------------------------------------------------
// Hyperion Framework Example 1
// https://github.com/Bazzz-1/hyperion-framework-examples
// https://github.com/Bazzz-1/hyperion-framework
//
// A lightweight component-based TCP framework for building service-oriented Rust applications with
// CLI control, async messaging, and lifecycle management.
//
// Example written by Robert Hannah 2025
// -------------------------------------------------------------------------------------------------

// Package
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExampleMessage {
    pub message: String,
    pub value: u64
}

impl Default for ExampleMessage {
    fn default() -> ExampleMessage {
        ExampleMessage {
            message: String::new(),
            value: 0
        }
    }
}

impl ExampleMessage {
    pub fn new(message: String, value: u64) -> ExampleMessage {
        ExampleMessage {
            message,
            value
        }
    }
}
