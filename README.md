# Submodule User Example Client

This repository provides a **gRPC client** written in **Rust** using the **Tokio**, **Tonic**, and **Prost** libraries for asynchronous operations and communication with gRPC servers. It handles requests via gRPC and interacts with a server through the provided protocol files.

### Submodules
This repository employs a submodule, `submodule-provider-example`, which contains the necessary **gRPC** definition files. These files are essential for the server to interact with other services using gRPC.

- **Submodule:** [submodule-provider-example](https://github.com/eyyyyyyy3/submodule-provider-example)

The submodule is already added in this repository, and all you need to do is **initialize** it locally.

### How to Work with Git Submodules

To work with Git submodules, follow these steps:

1. **Cloning the Repository with Submodules:**

   If you clone the repository and want to include the submodule, use:

   ```bash
   git clone --recurse-submodules https://github.com/eyyyyyyy3/submodule-user-example-client.git
   ```

   If you have already cloned the repository without the `--recurse-submodules` flag, you can initialize the submodule with the following commands:

   ```bash
   git submodule init
   git submodule update
   ```

2. **Updating Submodules:**

   To update the submodule to the latest commit from the upstream repository, use:

   ```bash
   git submodule update --remote
   ```

   This will fetch the latest changes from the submodule's repository.

### How to Start the Client

To run the gRPC client, follow these simple steps:

1. **Navigate to the project directory:**
   ```bash
   cd submodule-user-example-client
   ```

2. **Build and run the client using Cargo:**
   ```bash
   cargo run
   ```

### Running the Server

To interact with the client, you also need to run the server locally. The server can be found in the **submodule-user-example-server** repository, which is hosted at the following link:

- [submodule-user-example-server GitHub Repository](https://github.com/eyyyyyyy3/submodule-user-example-server)

Make sure the server is running before executing the client. The client will communicate with the server using gRPC, so both must be operational for the interaction to work.
