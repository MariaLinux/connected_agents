# Connected Agents

Connected Agents is a workflow graph engine designed to connect multiple agents (plugins) together, enabling automated task execution without writing code. By linking triggers, actions, and functions, users can build flexible workflows for embedded Linux and resource-limited devices.

## Features

- **No-code Workflow Creation:** Design automated flows by connecting plugin nodes—no coding required.
- **Trigger Plugins:** Start workflows based on events (e.g., timers, email received, etc.).
- **Action Plugins:** Process data, connect outputs to other actions or functions (e.g., execute shell commands using email content).
- **Function Plugins:** Transform input data (e.g., regex matching, custom processors).
- **Plugin System:** Easily register and enable new plugin types for triggers, actions, or functions.
- **Rust Implementation:** Lightweight and high-performance, ideal for embedded Linux systems.

## How It Works

Workflows are built as directed acyclic graphs (DAGs), connecting nodes that represent triggers, actions, or functions:

- **Triggers** are entry points (start nodes) like timers or event listeners.
- **Actions** receive input, perform operations, and pass outputs to other nodes.
- **Functions** manipulate or transform data and output the result for further processing.

Each workflow consists of:
- **Nodes:** Each with a unique ID, plugin name, type, and parameters.
- **Connections:** Define data flow between nodes.
- **Data Types:** Support for plain text, JSON, XML, HTML, etc.

Example workflow:
1. Timer trigger starts every second.
2. Action node processes input (e.g., receives an email).
3. Function node transforms the data (e.g., applies regex).
4. Output is passed to another action (e.g., executes shell command).

## Architecture

- **Node Registry:** Registers and enables plugins, manages executor factories.
- **Flow Graph:** Uses [`petgraph`](https://github.com/petgraph/petgraph) to build and validate the workflow graph, ensuring correct execution order and acyclicity.
- **Async Execution:** Nodes are executed asynchronously for efficiency.
- **Configurable:** Workflows and settings are managed via YAML configuration files.

## Example Plugins

- **HTTP Action:** Execute HTTP requests and route results based on content type.
- **Function Node:** Processes and tags input data (e.g., marks as processed, adds metadata).

## Getting Started

### Prerequisites

- Rust toolchain (`rustc`, `cargo`)
- Embedded Linux device or standard Linux system

### Installation

Clone the repository:
```sh
git clone https://github.com/MariaLinux/connected_agents.git
cd connected_agents
```

Install dependencies (see `devbox.json` for setup):
```sh
cargo build --release
```

### Usage

Run with default settings:
```sh
./connected_agents
```

Specify workflow and settings file:
```sh
./connected_agents --settings settings.yaml --workflow my_workflow.yaml
```

## License

This project is licensed under the [MIT License](LICENSE).

---

*Connected Agents empowers you to build automated workflows for Linux and embedded devices, connecting powerful plugins using a simple, no-code graph builder!*
