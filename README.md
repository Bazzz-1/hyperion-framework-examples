# hyperion-framework-examples


---

## Hyperion Framework Overview
[The Hyperion Framework](https://github.com/Bazzz-1/hyperion-framework) is a lightweight, component-based framework for building distributed Rust applications using TCP messaging. It helps you split large programs into modular, asynchronous services—each wrapped in a HyperionContainer.

Each component becomes a self-contained, event-driven service that:

    Listens and responds to structured TCP messages

    Exposes a CLI for control and inspection

    Handles its own config parsing, logging, and lifecycle state (start/restart/shutdown)

Hyperion is ideal for service-oriented projects where you want clean separation of logic, real-time communication, and
container-like encapsulation within native Rust programs.

The framework makes it simple to bring your project into a fully asynchronous and multithreaded service-based environment,
enabling independent component development, easier debugging, and scalability.

---

## Getting Started


### Repository Structure

```
hyperion-framework-examples
├── common_messages         # Shared messages and utilities used across components
├── component_a             # First example component
│   ├── config              # Contains component XML configurations
│   ├── src                 # Component code
├── component_b             # Second example component
│   ├── config              # Contains component XML configurations
│   ├── src                 # Component code
├── README.md               # This file
└── Cargo.toml              # Rust package configuration
```


---

### User Guide

#### Running the Example

This is a simple example of 2 containers (`component_a` and `component_b`) receiving, processing, and publishing messages.

To run the example:

1. Clone this repo and open a terminal at with `hyperion-framework-examples` as the root

2. Duplicate your terminal and run 
```
cargo run --bin component_a
```
in one, and
```
cargo run --bin component_b
```
in the other.

3. After each component has started, you can type `h` into each terminal to display the menu:
    - **`h`**: Show help with command descriptions.
    - **`start`**: Set the component state to active.
    - **`suspend`**: Put the component into a dormant state.
    - **`s`**: Perform a graceful shutdown of the container.
    - **`s.`**: Shut down the entire container network.

4. component_a has the extra item `run_example`. Type this into component_a's terminal to start the components sending messages between each other.
5. Type `s.` in either terminal to end the example and close the containers gracefully, or `s` in both terminals.


---
## Implementing Your Own Components

This repository's examples can serve as a solid base to start implementing your own components.

You can create your own components and containers by following the same structures for the main.rs, component.rs and config.rs files.

Also, make sure to update the config files for each container accordingly.

---

If you have any questions, issues, or feature requests, feel free to open an issue or reach out through the project ([Hyperion Framework](https://github.com/Bazzz-1/hyperion-framework)) GitHub page.

---

Let me know if you'd like me to adjust or expand on any section further!