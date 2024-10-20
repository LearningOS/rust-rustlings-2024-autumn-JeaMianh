/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

 // 辅助函数：执行递归DFS
 fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
    visited.insert(v); // 标记节点为已访问
    visit_order.push(v); // 记录访问顺序

    // 递归访问所有未访问的邻接节点
    for &neighbor in &self.adj[v] {
        if !visited.contains(&neighbor) {
            self.dfs_util(neighbor, visited, visit_order);
        }
    }
}

// 执行深度优先搜索，并返回访问顺序
fn dfs(&self, start: usize) -> Vec<usize> {
    let mut visited = HashSet::new(); // 记录已访问节点
    let mut visit_order = Vec::new(); // 记录访问顺序
    self.dfs_util(start, &mut visited, &mut visit_order); // 从起始节点开始递归DFS
    visit_order
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}

