// Integration tests for the DAG Manager API
// Note: These tests require a running PostgreSQL database

#[cfg(test)]
mod tests {
    use serde_json::json;
    use uuid::Uuid;

    // Helper function to create a test DAG
    fn create_dag_payload(name: &str) -> serde_json::Value {
        json!({
            "name": name
        })
    }

    // Helper function to create a test node
    fn create_node_payload(dag_id: Uuid, label: &str) -> serde_json::Value {
        json!({
            "dag_id": dag_id,
            "label": label
        })
    }

    // Helper function to create a test edge
    fn create_edge_payload(source: Uuid, target: Uuid, dag_id: Uuid) -> serde_json::Value {
        json!({
            "source": source,
            "target": target,
            "dag_id": dag_id
        })
    }

    #[test]
    fn test_create_dag_payload() {
        let payload = create_dag_payload("Test DAG");
        assert_eq!(payload["name"], "Test DAG");
    }

    #[test]
    fn test_create_node_payload() {
        let dag_id = Uuid::new_v4();
        let payload = create_node_payload(dag_id, "Test Node");
        assert_eq!(payload["dag_id"], dag_id.to_string());
        assert_eq!(payload["label"], "Test Node");
    }

    #[test]
    fn test_create_edge_payload() {
        let source = Uuid::new_v4();
        let target = Uuid::new_v4();
        let dag_id = Uuid::new_v4();
        let payload = create_edge_payload(source, target, dag_id);
        assert_eq!(payload["source"], source.to_string());
        assert_eq!(payload["target"], target.to_string());
        assert_eq!(payload["dag_id"], dag_id.to_string());
    }

    // Note: Full integration tests require database setup and a running server
    // These can be added with proper test database configuration
}
