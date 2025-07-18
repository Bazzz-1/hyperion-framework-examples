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
use hyperion_framework::messages::container_directive::ContainerDirective;
use hyperion_framework::messages::component_directive::ComponentDirective;
use hyperion_framework::containerisation::traits::HyperionContainerDirectiveMessage;
use serde::{Serialize, Deserialize};

// Local
use crate::messages::example_message::ExampleMessage;
use crate::messages::another_example_message::AnotherExampleMessage;

// This will be the only message that is sent between containers
// Container and component directives are essential for the Hyperion Network
// The rest are specific to your project


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerMessage {
    ContainerDirectiveMsg       (ContainerDirective),
    ComponentDirectiveMsg       (ComponentDirective),
    ExampleMessage              (ExampleMessage),
    AnotherExampleMessage       (AnotherExampleMessage),
    // Add more messages as needed
}

impl HyperionContainerDirectiveMessage for ContainerMessage {
    // Gets ContainerDirective if is instance
    fn get_container_directive_message(&self) -> Option<&ContainerDirective> {
        if let ContainerMessage::ContainerDirectiveMsg(directive) = self {
            Some(directive)
        } else {
            None
        }
    }
}
