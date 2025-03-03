# HTTP Server in Rust

This is a simple HTTP server implemented in Rust. It handles basic HTTP requests such as GET and POST, and supports routes for serving files, echoing requests, and providing user-agent information.

## Features

- **Root Route**: Responds with a simple 200 OK response.
- **Echo Route**: Returns the path segment provided in the request.
- **User-Agent Route**: Returns the User-Agent header from the request.
- **Files Route**:
    - **GET**: Serves files from the `/tmp/usercontent/` directory.
    - **POST**: Accepts file uploads and saves them to the `/tmp/usercontent/` directory.
- **404 Handling** - For undefined routes

## Usage

1. **Clone the repository**:
   ```bash
   git clone https://github.com/zxasc/rust-server rust-server 
   cd rust-server
   ```
   
2. **Run the server**:
    ```bash
    cargo run
    ```
   
3. **Send requests to the server**:
    ```bash
    # Root endpoint
    curl http://localhost:4221/

    # Echo endpoint
    curl http://localhost:4221/echo/hello-world

    # User-Agent endpoint
    curl -H "User-Agent: my-client" http://localhost:4221/user-agent

    # File operations
    echo "test content" > /tmp/usercontent/test.txt
    curl http://localhost:4221/files/test.txt
    curl -X POST --data-binary @localfile.txt http://localhost:4221/files/uploaded.txt 
    ```
   
## Project Structure

- `main.rs`: Core server implementation
  - TCP listener
  - Request parsing
  - Connection handling
  
- `router.rs`: Routing logic
  - Route mapping 
  - Request handlers
  - File operations
  
## Implementation Notes
- Uses standard library networking (no external dependencies)
- Basic HTTP header parsing
- Thread-per-connection model (no thread pool)
- Simple content-type handling (text/plain and application/octet-stream)

## License
MIT License - See [LICENSE](./LICENSE) for details.