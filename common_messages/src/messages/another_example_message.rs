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
