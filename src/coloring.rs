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


/// Return a greedy coloring of the graph.
/// There is no guaranteed about the number of colors used.
pub fn greedy_coloring(graph: &Graph) -> Coloring {
    let mut c = Coloring::new();
    let n = graph.vertices().count();

    for &v in graph.vertices() {
        let mut blocked_colors = HashSet::new();
        for u in graph.neighbors(v) {
            if let Some(color) = c.get(u) {
                blocked_colors.insert(*color);
            }
        }


        for x in 0..n {
            if !blocked_colors.contains(&x) {
                c.insert(v, x);
                break;
            }
        }
    }

    c
}



#[cfg(test)]
mod tests {
    use graph::Graph;
    use coloring:: { Coloring, check_coloring, compatible_coloring, num_colors, greedy_coloring };

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

    #[test]
    fn greedy_color() {
        let mut g = Graph::new();

        g.add_edge(1,2);

        let c = greedy_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn greedy_color2() {
        let mut g = Graph::new();

        g.add_edge(1,2);
        g.add_edge(1,3);

        let c = greedy_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn greedy_line() {
        let mut g = Graph::new();

        for i in 0..10 {
            g.add_edge(i, i+1);
        }

        let c = greedy_coloring(&g);

        assert!(check_coloring(&g, &c));

        // Line might not be 2-colored by greedy
        // in case of unfortunate vertex ordering
        assert!(num_colors(&c) <= 3);
    }
}
