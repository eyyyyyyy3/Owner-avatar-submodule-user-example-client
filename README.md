Submodule User Example Server

This project is a Rust-based gRPC server that uses Tokio, Tonic, and Prost to handle gRPC requests. It is designed to work with the submodule-user-example-client, which requires the server to be running to function properly. The project utilizes Git submodules to manage its dependencies.
Features

    Async support via Tokio.
    gRPC server implementation using Tonic.
    Protocol buffer serialization with Prost.
    Git submodule integration for managing dependent repositories.

Getting Started
Clone the Repository

When cloning this repository, ensure you initialize and update its submodules:

git clone --recurse-submodules <repository-url>

If you've already cloned the repository without initializing submodules, you can do it manually:

git submodule update --init --recursive

Running the Server

To start the submodule-user-example-server, navigate to the server's directory and run:

cd submodule-user-example-server
cargo run

The server will start listening for gRPC requests.
Git Submodule Management

This project uses Git submodules to include dependencies like submodule-user-example-client.
Adding a Submodule

To add a submodule to the repository:

    Navigate to the root of the repository.

    Use the following command to add the submodule:

git submodule add <submodule-repository-url> <submodule-directory>

For example:

git submodule add https://github.com/eyyyyyyy3/submodule-user-example-client submodule-user-example-client

Commit the changes to include the submodule:

    git commit -m "Add submodule-user-example-client as a submodule"

Updating Submodules

If the submodule repository has updates, pull them into your local repository with the following commands:

# Navigate to the root of the repository
git submodule update --remote

To fetch updates for a specific submodule:

git submodule update --remote submodule-user-example-client

After updating, commit the changes to the superproject:

git commit -m "Update submodule-user-example-client"

# Client Dependency

The submodule-user-example-client depends on the server being up and running. Start the server first before using the client:

    Start the server:

cd submodule-user-example-server
cargo run

Then, run the client in its directory.
