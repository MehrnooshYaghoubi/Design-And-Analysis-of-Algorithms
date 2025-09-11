pub fn dfs(graph: &Vec<Vec<usize>>, start: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
    visited[start] = true;
    order.push(start);

    for &neighbor in &graph[start] {
        if !visited[neighbor] {
            dfs(graph, neighbor, visited, order);
        }
    }
}
