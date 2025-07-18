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
use std::sync::Arc as StdArc;
use std::sync::atomic::{AtomicUsize, Ordering};

// Package
use common_messages::messages::container_message::ContainerMessage;
use common_messages::messages::example_message::ExampleMessage;
use hyperion_framework::containerisation::traits::{Initialisable, Run};
use hyperion_framework::containerisation::component_state::ComponentState;
use hyperion_framework::containerisation::container_state::ContainerState;
use hyperion_framework::messages::client_broker_message::ClientBrokerMessage;
use hyperion_framework::messages::component_directive::ComponentDirective;
use hyperion_framework::utilities::tx_sender::add_to_tx_with_retry;

use async_trait::async_trait;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::Notify;

// Local
use crate::config::Config;
use crate::component_logic::internal_logic::process_message;


/// Represents a component within the Hyperion Framework.
/// 
/// The component is the top level entry point into your specific implementation. Think of it as the
/// point where Hyperion hands all the specific logic processing over to you.
#[derive(Clone, Debug)]
pub struct Component {
    /// Current state of the top level HyperionContainer (which wraps this component)
    /// This can be set to shutdown through various methods and will auto-kill the component
    container_state: StdArc<AtomicUsize>,
    /// Notification mechanism for container state changes
    container_state_notify: StdArc<Notify>,
    /// Current state of this component (Active, Dormant, Dead)
    component_state: ComponentState,
    /// Specific configuration for this component
    config: StdArc<Config>
}

// Hyperion Network Containerisation - Component Initialization
impl Initialisable for Component {
    type ConfigType = Config;
    
    /// Initializes a new component instance.
    /// 
    /// # Arguments
    /// 
    /// * `container_state` - Atomic reference to container state
    /// * `container_state_notify` - Notification mechanism for state changes
    /// * `config` - Component configuration
    fn initialise(container_state: StdArc<AtomicUsize>, container_state_notify: StdArc<Notify>, config: StdArc<Self::ConfigType>) -> Self {
        Component::new(container_state, container_state_notify, config)
    }
}

#[async_trait]
impl Run for Component {
    type Message = ContainerMessage;
    
    /// Main run loop for the component.
    /// 
    /// Handles incoming messages and container state changes until the component
    /// is either killed or the container is shutting down.
    /// 
    /// # Arguments
    /// 
    /// * `comp_in_rx` - Receiver for incoming component messages
    /// * `comp_out_tx` - Sender for outgoing broker messages
    async fn run(mut self, mut comp_in_rx: Receiver<Self::Message>, comp_out_tx: Sender<ClientBrokerMessage<Self::Message>>) {
        log::info!("{} has started successfully", self.config.container.name);
        
        loop {
            if self.component_state == ComponentState::Dead { break; }
            
            tokio::select! {
                // Handle incoming messages
                Some(message) = comp_in_rx.recv() => {
                    log::trace!("{} received message: {:?}", self.config.container.name, message);
                    if let Some(result) = self.process_incoming_message(message).await {
                        let from_location = format!("{} main loop", self.config.container.name);
                        let to_location = format!("{} Container", self.config.container.name);
                        add_to_tx_with_retry(&comp_out_tx, &result, &from_location, &to_location).await;
                    }
                }
                // Handle container state notifications
                _ = self.container_state_notify.notified() => {
                    // Check for container shutdown
                    if self.container_state.load(Ordering::SeqCst) == ContainerState::ShuttingDown as usize {
                        self.component_state = ComponentState::Dormant;
                        break;
                    }
                }
            }
        }
        log::info!("{} task has closed", self.config.container.name);
    }
}

impl Component {
    /// Creates a new Component instance
    fn new(container_state: StdArc<AtomicUsize>, container_state_notify: StdArc<Notify>, config: StdArc<Config>) -> Self {
        Self {
            container_state,
            container_state_notify,
            component_state: ComponentState::Active,
            config: config.clone()
        }
    }

    /// Processes incoming messages and returns an optional response message
    /// 
    /// # Arguments
    /// 
    /// * `message` - The incoming container message to process
    /// 
    /// # Returns
    /// 
    /// * `Option<ClientBrokerMessage<ContainerMessage>>` - Optional response message
    async fn process_incoming_message(&mut self, message: ContainerMessage) -> Option<ClientBrokerMessage<ContainerMessage>> {
        match message {
            ContainerMessage::ComponentDirectiveMsg(comp_directive) => {
                log::trace!("{} is processing a component directive: {:?}", self.config.container.name, comp_directive);
                match comp_directive {
                    ComponentDirective::SetToActive => {
                        self.component_state = ComponentState::Active;
                        log::info!("{} received a SetToActive directive, but this hasn't been configured for your component yet.",
                            self.config.container.name);
                    }
                    ComponentDirective::SetToDormant => {
                        // You can decide what Dormant means for your specific component
                        self.component_state = ComponentState::Dormant;
                        log::info!("{} received a SetToDormant directive, but this hasn't been configured for your component yet.",
                            self.config.container.name);
                    }
                    ComponentDirective::WriteToFile => {
                        // You can decide what WriteToFile means for your specific component
                        log::debug!("{} received a WriteToFile directive, but this hasn't been configured for your component yet.",
                            self.config.container.name);
                    }
                }
            }
            ContainerMessage::ExampleMessage(example_message) => {
                log::debug!("{} has received an ExampleMessage", self.config.container.name);
                // Process message using component's logic. This can be as complex as you like
                let response: ExampleMessage = process_message(example_message, self.config.component.increment).await;
                // Take response and put it into a ClientBrokerMessage for Hyperion to process and send to relevant containers
                // The target_clients correlate to the names given in network_topology.xml
                return Some(ClientBrokerMessage::new(vec!["ComponentA"], ContainerMessage::ExampleMessage(response)));
            }
            ContainerMessage::AnotherExampleMessage(another_example_message) => {
                log::debug!("{} has received AnotherExampleMessage - not logic implemented",
                    self.config.container.name);
            }

            // Add more cases for Component specific messages here!

            _ => {
                if self.component_state == ComponentState::Active  {
                    log::debug!("{} has received an unmapped message: {:?}", self.config.container.name, message);
                }
            }
        }
        // Do nothing
        None
    }
}
