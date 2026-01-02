# Axum DAG Manager

A high-performance REST API service for managing Directed Acyclic Graphs (DAGs) built with Rust, Axum, and PostgreSQL.

## Features

- ✅ Full CRUD operations for DAGs, Nodes, and Edges
- ✅ **Automatic cycle detection** - prevents creation of cycles in the graph
- ✅ Input validation for all endpoints
- ✅ Proper error handling with meaningful error messages
- ✅ UUID-based identifiers for distributed systems
- ✅ Async/await with Tokio runtime
- ✅ Type-safe database queries with SQLx
- ✅ Modular architecture with separation of concerns
- ✅ Unit and integration tests

## Technology Stack

- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) 0.6
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) 0.6
- **Runtime**: [Tokio](https://tokio.rs) (async)
- **Serialization**: Serde + serde_json
- **IDs**: UUID v4

## Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 12+ ([Install PostgreSQL](https://www.postgresql.org/download/))

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd Axum_DAG_Manager
```

2. Set up the database:
```bash
# Create database
createdb dag_service

# Run migrations
psql -d dag_service -f db_schema_migration.sql
```

3. Configure environment:
```bash
# Create .env file
echo 'DATABASE_URL=postgres://postgres:password@localhost:5432/dag_service' > .env
```

4. Build and run:
```bash
cargo build --release
cargo run
```

The server will start at `http://127.0.0.1:3000`

## API Documentation

### DAG Endpoints

#### Create DAG
```http
POST /dags
Content-Type: application/json

{
  "name": "My DAG"
}
```

**Response**: `201 Created`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "My DAG"
}
```

#### List All DAGs
```http
GET /dags
```

**Response**: `200 OK`
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "My DAG"
  }
]
```

#### Get DAG by ID
```http
GET /dags/:id
```

**Response**: `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "My DAG"
}
```

#### Get DAG with Full Details
```http
GET /dags/:id/details
```

**Response**: `200 OK`
```json
{
  "dag": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "My DAG"
  },
  "nodes": [
    {
      "id": "...",
      "dag_id": "550e8400-e29b-41d4-a716-446655440000",
      "label": "Node A"
    }
  ],
  "edges": [
    {
      "id": "...",
      "source": "...",
      "target": "...",
      "dag_id": "550e8400-e29b-41d4-a716-446655440000"
    }
  ]
}
```

#### Update DAG
```http
PUT /dags/:id
Content-Type: application/json

{
  "name": "Updated DAG Name"
}
```

**Response**: `200 OK`

#### Delete DAG
```http
DELETE /dags/:id
```

**Response**: `204 No Content`

**Note**: Deletes all associated nodes and edges.

---

### Node Endpoints

#### Create Node
```http
POST /nodes
Content-Type: application/json

{
  "dag_id": "550e8400-e29b-41d4-a716-446655440000",
  "label": "Node A"
}
```

**Response**: `201 Created`
```json
{
  "id": "660e8400-e29b-41d4-a716-446655440001",
  "dag_id": "550e8400-e29b-41d4-a716-446655440000",
  "label": "Node A"
}
```

**Validation**: DAG must exist.

#### List All Nodes
```http
GET /nodes
```

#### Get Node by ID
```http
GET /nodes/:id
```

#### Update Node
```http
PUT /nodes/:id
Content-Type: application/json

{
  "label": "Updated Label"
}
```

#### Delete Node
```http
DELETE /nodes/:id
```

**Response**: `204 No Content`

**Note**: Deletes all associated edges.

---

### Edge Endpoints

#### Create Edge
```http
POST /edges
Content-Type: application/json

{
  "source": "660e8400-e29b-41d4-a716-446655440001",
  "target": "660e8400-e29b-41d4-a716-446655440002",
  "dag_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response**: `201 Created`
```json
{
  "id": "770e8400-e29b-41d4-a716-446655440003",
  "source": "660e8400-e29b-41d4-a716-446655440001",
  "target": "660e8400-e29b-41d4-a716-446655440002",
  "dag_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Validation**:
- ✅ DAG must exist
- ✅ Source and target nodes must exist
- ✅ Both nodes must belong to the specified DAG
- ✅ Cannot create self-loops
- ✅ **Cannot create cycles** (enforced via graph algorithm)

**Error Response** (if cycle detected):
```json
{
  "error": "Adding this edge would create a cycle in the DAG"
}
```

#### List All Edges
```http
GET /edges
```

#### Get Edge by ID
```http
GET /edges/:id
```

#### Delete Edge
```http
DELETE /edges/:id
```

**Response**: `204 No Content`

---

## Error Responses

All errors return a JSON object with an `error` field:

```json
{
  "error": "Error message here"
}
```

**Status Codes**:
- `400 Bad Request` - Validation error or cycle detected
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Database or server error

## Project Structure

```
src/
├── main.rs              # Application entry point & routing
├── db.rs                # Database connection setup
├── error.rs             # Error types and handling
├── graph.rs             # DAG cycle detection algorithms
├── models.rs            # Data models and request/response types
├── validation.rs        # Input validation functions
└── handlers/
    ├── mod.rs
    ├── dag.rs           # DAG CRUD handlers
    ├── node.rs          # Node CRUD handlers
    └── edge.rs          # Edge CRUD handlers

tests/
└── api_tests.rs         # Integration tests

db_schema_migration.sql  # Database schema
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_detects_cycle
```

### Database Schema

The application uses three tables:

**dags**
- `id` (UUID, PRIMARY KEY)
- `name` (TEXT, NOT NULL)

**nodes**
- `id` (UUID, PRIMARY KEY)
- `dag_id` (UUID, FOREIGN KEY → dags.id)
- `label` (TEXT, NOT NULL)

**edges**
- `id` (UUID, PRIMARY KEY)
- `source` (UUID, FOREIGN KEY → nodes.id)
- `target` (UUID, FOREIGN KEY → nodes.id)
- `dag_id` (UUID, FOREIGN KEY → dags.id)

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check without building
cargo check
```

## Architecture Highlights

### Cycle Detection Algorithm

The service implements a **DFS-based cycle detection** algorithm that:
1. Builds an adjacency list from existing edges
2. Simulates adding the new edge
3. Checks if there's a path from the target back to the source
4. Rejects the edge if it would create a cycle

See `src/graph.rs` for implementation.

### Error Handling

Custom `AppError` enum with automatic conversion to HTTP responses:
- Database errors → 500 Internal Server Error (details hidden)
- Not found errors → 404 Not Found
- Validation errors → 400 Bad Request
- Cycle detection → 400 Bad Request

### Input Validation

All user inputs are validated:
- Names and labels must be non-empty and ≤ 255 characters
- UUIDs are validated by the type system
- Foreign key relationships are verified before insertion

## Roadmap

- [ ] Add pagination for list endpoints
- [ ] Implement topological sorting
- [ ] Add graph traversal endpoints (DFS/BFS)
- [ ] Add filtering and search capabilities
- [ ] OpenAPI/Swagger documentation
- [ ] Docker containerization
- [ ] Authentication & authorization
- [ ] Rate limiting
- [ ] Observability (logging, metrics, tracing)
- [ ] GraphQL API option

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Apache License 2.0 - see [LICENSE](LICENSE) file for details.

## Performance

Built with Rust for maximum performance:
- **Zero-cost abstractions** - no runtime overhead
- **Memory safe** - no garbage collection pauses
- **Async I/O** - handles thousands of concurrent connections
- **Type-safe SQL** - compile-time query validation

## Support

For issues, questions, or contributions, please open an issue on GitHub.
