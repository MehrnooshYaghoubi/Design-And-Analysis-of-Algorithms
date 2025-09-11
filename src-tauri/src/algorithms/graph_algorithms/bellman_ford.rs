use crate::types::GraphInput;

pub fn bellman_ford(graph: &GraphInput, start: usize) -> Result<Vec<isize>, &'static str> {
    let mut dist = vec![isize::MAX; graph.num_vertices];
    dist[start] = 0;

    // Relax edges repeatedly
    for _ in 0..graph.num_vertices - 1 {
        let mut updated = false;
        for edge in &graph.edges {
            if dist[edge.src] != isize::MAX
                && dist[edge.src] + (edge.weight as isize) < dist[edge.dest]
            {
                dist[edge.dest] = dist[edge.src] + (edge.weight as isize);
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }

    // Check for negative weight cycles
    for edge in &graph.edges {
        if dist[edge.src] != isize::MAX && dist[edge.src] + (edge.weight as isize) < dist[edge.dest]
        {
            return Err("Graph contains a negative weight cycle");
        }
    }

    Ok(dist)
}
