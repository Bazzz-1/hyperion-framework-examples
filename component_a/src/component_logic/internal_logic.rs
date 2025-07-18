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
use common_messages::messages::example_message::ExampleMessage;
use tokio::time::{sleep, Duration};


pub async fn process_message(message: ExampleMessage, increment_from_config: u8) -> ExampleMessage {
    log::info!("Processing ExampleMessage...  {:?}", message);
    sleep(Duration::from_secs(1)).await;

    log::info!("Incrementing value and building new message to send to ComponentB");
    ExampleMessage::new(
        "Message from ComponentA!".into(),
        message.value + increment_from_config as u64
    )
}
