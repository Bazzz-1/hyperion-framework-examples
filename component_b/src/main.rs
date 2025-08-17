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

//! Main entry point for the Hyperion Framework example application.

// Standard
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc as StdArc;

// Package
use common_messages::messages::container_message::ContainerMessage;
use hyperion_framework::messages::container_directive::ContainerDirective;
use hyperion_framework::messages::component_directive::ComponentDirective;
use hyperion_framework::containerisation::container_state::ContainerState;
use hyperion_framework::containerisation::hyperion_container::HyperionContainer;
use hyperion_framework::containerisation::hyperion_container_factory;
use hyperion_framework::utilities::tx_sender::add_to_tx_with_retry;

use tokio::io::{AsyncBufReadExt, BufReader, stdin};
use tokio::sync::{mpsc, Notify};
use tokio::time::{sleep, Duration};
use tokio::task;

// Local
mod component_logic;
mod component;
mod config;

use component::Component;
use config::Config;

/// Main entry point for the application.
/// Sets up and runs a Hyperion container with CLI control interface.
/// Generally speaking, very little should ever need to change in this file for implementation
#[tokio::main]
async fn main() {
    // Initialize container state management
    // Uses atomic operations for thread-safe state changes
    let container_state: StdArc<AtomicUsize> = StdArc::new(AtomicUsize::new(ContainerState::Running as usize));
    let container_state_notify: StdArc<Notify> = StdArc::new(Notify::new());

    // Set up message channel for container communication
    // Channel buffer size of 32 messages to prevent blocking
    let (main_tx, main_rx) = mpsc::channel::<ContainerMessage>(32);
    
    // Initialize and configure the Hyperion container
    let mut container: HyperionContainer<ContainerMessage> =
        hyperion_container_factory::create::<Component, Config, ContainerMessage>(
            "component_b/config/configuration.xml",    // Container configuration file
            "component_b/config/network_topology.xml", // Network topology configuration
            container_state.clone(),
            container_state_notify.clone(),
            main_rx
        ).await;

    // Spawn the container in a separate task
    task::spawn(async move {
        container.run().await;
    });

    // Initialize CLI interface with async input handling
    let mut reader = BufReader::new(stdin()).lines();
    log::info!("Enter a command or press h for help.");
    
    // Main command processing loop
    loop {
        tokio::select! {
            // Handle CLI commands
            Ok(Some(command)) = reader.next_line() => {
                let command = command.trim();
                match command {
                    "h" => {
                        // Display available commands
                        println!("Commands:");
                        println!("start         - Component state set to active");
                        println!("suspend       - Component state set to dormant");
                        println!("s             - Graceful container shutdown");
                        println!("s.            - Graceful container network shutdown");
                    }
                    // Command handlers for component state management
                    "start" => {
                        add_to_tx_with_retry(&main_tx, 
                            &ContainerMessage::ComponentDirectiveMsg(ComponentDirective::SetToActive), 
                            "Command line", 
                            "Container main"
                        ).await;
                    }
                    "suspend" => {
                        add_to_tx_with_retry(&main_tx, 
                            &ContainerMessage::ComponentDirectiveMsg(ComponentDirective::SetToDormant), 
                            "Command line", 
                            "Container main"
                        ).await;
                    }
                    // Command handlers for container management
                    "s" => {
                        add_to_tx_with_retry(&main_tx, 
                            &ContainerMessage::ContainerDirectiveMsg(ContainerDirective::Shutdown), 
                            "Command line", 
                            "Container main"
                        ).await;
                    }
                    "s." => {
                        add_to_tx_with_retry(&main_tx, 
                            &ContainerMessage::ContainerDirectiveMsg(ContainerDirective::SystemShutdown), 
                            "Command line", 
                            "Container main"
                        ).await;
                    }
                    _ => {
                        println!("Unknown command: {}", command);
                        println!("\n\nCommands:");
                        println!("start         - Component state set to active");
                        println!("suspend       - Component state set to dormant");
                        println!("s             - Graceful container shutdown");
                        println!("s.            - Graceful container network shutdown");
                    },
                }
            }
            // Monitor container state for shutdown
            _ = container_state_notify.notified() => {
                if container_state.load(Ordering::SeqCst) == ContainerState::Closed as usize {
                    break;
                }
            }
        }
    }

    // Clean shutdown procedure
    drop(reader);
    log::info!("Hyperion Container closed gracefully");
    // Brief delay to allow final messages to be processed
    sleep(Duration::from_millis(500)).await;
    std::process::exit(0);
}