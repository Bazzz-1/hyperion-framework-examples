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
use common_messages::messages::example_message::ExampleMessage;
use tokio::time::{sleep, Duration};


pub async fn process_message(message: ExampleMessage, increment_from_config: u8) -> ExampleMessage {
    log::info!("Processing ExampleMessage...  {:?}", message);
    sleep(Duration::from_secs(1)).await;

    log::info!("Incrementing value and building new message to send to ComponentA");
    ExampleMessage::new(
        "Message from ComponentB!".into(),
        message.value + increment_from_config as u64
    )
}
