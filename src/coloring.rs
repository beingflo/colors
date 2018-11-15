use std::collections::HashMap;
use std::collections::HashSet;
use graph::Graph;

/// Coloring type.
/// This maps from vertices to colors.
pub type Coloring = HashMap<usize, usize>;

/// Check whether coloring defines a color for all vertices that exist in the graph.
pub fn compatible_coloring(graph: &Graph, coloring: &Coloring) -> bool {
    for u in graph.vertices() {
        if !coloring.contains_key(u) {
            return false;
        }
    }

    return true;
}

/// Check whether no adjacent vertices are in conflict.
/// 'false' indicates either a color conflict or no color define for at least
/// one of the vertices in the graph.
pub fn check_coloring(graph: &Graph, coloring: &Coloring) -> bool {
    if !compatible_coloring(graph, coloring) {
        return false;
    }

    for (u,v) in graph.edges() {
        if coloring[u] == coloring[v] {
            return false;
        }
    }

    return true;
}

/// Returns the number of colors used in the coloring.
pub fn num_colors(coloring: &Coloring) -> usize {
    let mut colors: HashSet<usize> = HashSet::new();

    for &val in coloring.values() {
        colors.insert(val);
    }

    colors.len()
}



#[cfg(test)]
mod tests {
    use graph::Graph;
    use coloring::Coloring;
    use coloring::check_coloring;
    use coloring::compatible_coloring;
    use coloring::num_colors;

    #[test]
    fn creation_empty() {
        let g = Graph::new();
        let c = Coloring::new();

        assert!(check_coloring(&g, &c));
    }

    #[test]
    fn creation_fail() {
        let mut g = Graph::new();
        let c = Coloring::new();

        g.add_edge(1,2);

        assert!(!check_coloring(&g, &c));
        assert!(!compatible_coloring(&g, &c));
    }

    #[test]
    fn creation_success() {
        let mut g = Graph::new();
        let mut c = Coloring::new();

        g.add_edge(1,2);

        c.insert(1, 0);
        c.insert(2, 1);

        assert!(check_coloring(&g, &c));
    }

    #[test]
    fn creation_large() {
        let mut g = Graph::new();
        let mut c = Coloring::new();

        for u in 0..100 {
            for v in u..100 {
                g.add_edge(u,v);
            }
        }

        for u in 0..100 {
            c.insert(u, u);
        }

        assert!(compatible_coloring(&g, &c));
        assert!(check_coloring(&g, &c));

        c.insert(4, 5);

        assert!(compatible_coloring(&g, &c));
        assert!(!check_coloring(&g, &c));
    }

    #[test]
    fn colors() {
        let mut c = Coloring::new();

        for u in 0..100 {
            c.insert(u, u % 11);
        }

        assert_eq!(num_colors(&c), 11);
    }
}
