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
pub struct AnotherExampleMessage {
    pub message: String,
    pub data: Vec<u64>
}

impl Default for AnotherExampleMessage {
    fn default() -> AnotherExampleMessage {
        AnotherExampleMessage {
            message: String::new(),
            data: Vec::new()
        }
    }
}
