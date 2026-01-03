use crate::error::{AppError, Result};
use crate::models::Edge;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Detects if adding a new edge would create a cycle in the DAG
pub fn would_create_cycle(edges: &[Edge], new_source: Uuid, new_target: Uuid) -> bool {
    // Build adjacency list
    let mut graph: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    // Add existing edges
    for edge in edges {
        graph.entry(edge.source).or_default().push(edge.target);
    }

    // Add the proposed new edge
    graph.entry(new_source).or_default().push(new_target);

    // Check if there's a path from new_target back to new_source (which would be a cycle)
    has_path(&graph, new_target, new_source)
}

/// DFS to check if there's a path from start to end
fn has_path(graph: &HashMap<Uuid, Vec<Uuid>>, start: Uuid, end: Uuid) -> bool {
    let mut visited = HashSet::new();
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if node == end {
            return true;
        }

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }

    false
}

/// Validates that edges form a proper DAG (no cycles)
/// Note: Kept for potential future use as an alternative validation method
#[allow(dead_code)]
pub fn validate_dag(edges: &[Edge]) -> Result<()> {
    let mut graph: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    for edge in edges {
        graph.entry(edge.source).or_default().push(edge.target);
    }

    // Detect cycles using DFS
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    for &node in graph.keys() {
        if !visited.contains(&node) && has_cycle(&graph, node, &mut visited, &mut rec_stack) {
            return Err(AppError::CycleDetected);
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn has_cycle(
    graph: &HashMap<Uuid, Vec<Uuid>>,
    node: Uuid,
    visited: &mut HashSet<Uuid>,
    rec_stack: &mut HashSet<Uuid>,
) -> bool {
    visited.insert(node);
    rec_stack.insert(node);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                if has_cycle(graph, neighbor, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&neighbor) {
                return true;
            }
        }
    }

    rec_stack.remove(&node);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycle() {
        let edges = vec![
            Edge {
                id: Uuid::new_v4(),
                source: Uuid::from_u128(1),
                target: Uuid::from_u128(2),
                dag_id: Uuid::new_v4(),
            },
            Edge {
                id: Uuid::new_v4(),
                source: Uuid::from_u128(2),
                target: Uuid::from_u128(3),
                dag_id: Uuid::new_v4(),
            },
        ];

        assert!(validate_dag(&edges).is_ok());
    }

    #[test]
    fn test_detects_cycle() {
        let source = Uuid::from_u128(1);
        let target = Uuid::from_u128(2);

        let edges = vec![Edge {
            id: Uuid::new_v4(),
            source,
            target,
            dag_id: Uuid::new_v4(),
        }];

        // Adding edge from target back to source would create a cycle
        assert!(would_create_cycle(&edges, target, source));
    }

    #[test]
    fn test_no_cycle_on_new_edge() {
        let edges = vec![Edge {
            id: Uuid::new_v4(),
            source: Uuid::from_u128(1),
            target: Uuid::from_u128(2),
            dag_id: Uuid::new_v4(),
        }];

        // Adding edge from 2 to 3 should not create a cycle
        assert!(!would_create_cycle(
            &edges,
            Uuid::from_u128(2),
            Uuid::from_u128(3)
        ));
    }
}
