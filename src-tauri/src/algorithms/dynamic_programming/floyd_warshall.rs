def floyd_warshall(graph):
    # Number of vertices in the graph
    n = len(graph)

    # Initialize the distance matrix with the input graph
    dist = [row[:] for row in graph]

    # Update the distance matrix
    for k in range(n):
        for i in range(n):
            for j in range(n):
                # Update the shortest distance from i to j through vertex k
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])

    return dist
