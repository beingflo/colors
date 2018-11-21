use std::collections::{ HashMap, HashSet, VecDeque };
use graph::StaticGraph;

/// Coloring type.
/// This maps from vertices to colors.
pub type Coloring = HashMap<usize, usize>;

/// Coloring heuristics implemented here.
#[derive(Debug, Clone, Copy)]
pub enum ColoringAlgo {
    RS,
    CS,
    LF,
    SL,
}

/// Color using the specified coloring heuristic.
pub fn color<G: StaticGraph>(graph: &G, col: ColoringAlgo) -> Coloring {
    match col {
        ColoringAlgo::RS => rs_coloring(graph),
        ColoringAlgo::CS => cs_coloring(graph),
        ColoringAlgo::LF => lf_coloring(graph),
        ColoringAlgo::SL => sl_coloring(graph),
    }
}

/// Check whether coloring defines a color for all vertices that exist in the graph.
pub fn compatible_coloring<G: StaticGraph>(graph: &G, coloring: &Coloring) -> bool {
    for u in graph.vertices() {
        if !coloring.contains_key(&u) {
            return false;
        }
    }

    return true;
}

/// Check whether no adjacent vertices are in conflict.
/// ```false``` indicates either a color conflict or no color defined for at least
/// one of the vertices in the graph.
pub fn check_coloring<G: StaticGraph>(graph: &G, coloring: &Coloring) -> bool {
    if !compatible_coloring(graph, coloring) {
        return false;
    }

    for (u,v) in graph.edges() {
        if coloring[&u] == coloring[&v] {
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

/// Returns a 2-coloring of the graph if it exists, ```None``` otherwise.
/// Can be used as a check for bipartiteness.
pub fn two_coloring<G: StaticGraph>(graph: &G) -> Option<Coloring> {
    let mut c = Coloring::new();
    let mut q = VecDeque::new();

    let first = graph.vertices().next().unwrap();
    q.push_back(first);
    c.insert(first, 0);

    while let Some(v) = q.pop_front() {
        let &color = c.get(&v).unwrap();

        for u in graph.neighbors(v) {
            if let Some(&col) = c.get(&u) {
                // Conflict
                if col == color {
                    return None;
                }
            } else {
                // Color neighbors opposite color and put in the frontier
                c.insert(u, 1-color);
                q.push_back(u);
            }
        }
    }

    Some(c)
}

/// Greedy coloring algorithm.
/// Colors the vertices in the sequence provided by chosing the
/// smallest color not in conflict.
pub fn greedy_coloring<G: StaticGraph>(graph: &G, vertices: impl Iterator<Item=usize>) -> Coloring {
    // Must be equal to 'vertices.count()'
    // as 'vertices' must be permutation of 'graph.vertices'
    let n = graph.vertices().count();
    let mut c = Coloring::new();

    let mut blocked_colors = vec![false; n];
    for v in vertices {
        for u in graph.neighbors(v) {
            if let Some(&color) = c.get(&u) {
                blocked_colors[color] = true;
            }
        }


        for x in 0..n {
            if !blocked_colors[x] {
                c.insert(v, x);
                break;
            }
        }

        blocked_colors = vec![false; n];
    }

    c
}

/// Returns a random-sequence greedy coloring of the graph where the vertices have
/// been colored in random order.
/// There is no guarantee about the number of colors used.
pub fn rs_coloring<G: StaticGraph>(graph: &G) -> Coloring {
    // No sequence building stage for this algorithm
    // 'graph.vertices()' returns random order as it's implemented as HashSet
    greedy_coloring(graph, graph.vertices())
}

/// Returns a connected-sequence greedy coloring of the graph where the vertices have
/// been colored in an order such that each vertex (except the first) has atleast one
/// neighbor that has already been colored.
pub fn cs_coloring<G: StaticGraph>(graph: &G) -> Coloring {
    // Sequence building stage
    let mut visited = HashSet::new();
    let mut vec: Vec<usize> = Vec::new();

    let n = graph.vertices().count();

    let first = graph.vertices().next().unwrap();
    visited.insert(first);
    vec.push(first);

    for i in 0..n {
        let v = vec[i];

        for u in graph.neighbors(v) {
            if !visited.contains(&u) {
                vec.push(u);
                visited.insert(u);
            }
        }
    }

    greedy_coloring(graph, vec.iter().cloned())
}

/// Returns a largest-first greedy coloring of the graph attained by greedily coloring
/// the vertices in order of decreasing degree.
/// There is no guarantee about the number of colors used.
pub fn lf_coloring<G: StaticGraph>(graph: &G) -> Coloring {
    // Sequence building stage
    let mut vertices: Vec<(usize, usize)> = graph.vertices().map(|u| (u, 0)).collect();

    for (v, d) in &mut vertices {
        *d = graph.neighbors(*v).count();
    }

    vertices.sort_by(|a,b| b.1.cmp(&a.1));

    greedy_coloring(graph, vertices.iter().map(|&(v, _)| v))
}

/// Returns a smallest-last greedy coloring of the graph.
/// This algorithm optimally colors trees, cycles and other types of graphs.
/// For general graphs there is no guarantee about the number of colors used.
pub fn sl_coloring<G: StaticGraph>(graph: &G) -> Coloring {
    // Sequence building stage
    // Inefficient implementation
    let mut k = Vec::new();
    let mut k_set = HashSet::new();

    let n = graph.vertices().count();

    while k.len() < n {
        let mut min_d = std::usize::MAX;
        let mut min_d_idx = 0;
        for v in graph.vertices() {
            // Only look at vertices not in k
            if k_set.contains(&v) {
                continue;
            }

            // Look for min degree of vertices not in k
            let mut degree = 0;
            for u in graph.neighbors(v) {
                if !k_set.contains(&u) {
                    degree += 1;
                }
            }
            if degree < min_d {
                min_d = degree;
                min_d_idx = v;
            }
        }

        k.push(min_d_idx);
        k_set.insert(min_d_idx);
    }

    // Greedy coloring with reversed order of k
    greedy_coloring(graph, k.iter().rev().cloned())
}



#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;

    #[test]
    fn coloring_creation_empty() {
        let g = EdgeList::new();
        let c = Coloring::new();

        assert!(check_coloring(&g, &c));
    }

    #[test]
    fn coloring_creation_fail() {
        let mut g = EdgeList::new();
        let c = Coloring::new();

        g.add_edge(1,2);

        assert!(!check_coloring(&g, &c));
        assert!(!compatible_coloring(&g, &c));
    }

    #[test]
    fn coloring_creation_success() {
        let mut g = EdgeList::new();
        let mut c = Coloring::new();

        g.add_edge(1,2);

        c.insert(1, 0);
        c.insert(2, 1);

        assert!(check_coloring(&g, &c));
    }

    #[test]
    fn coloring_creation_large() {
        let mut g = EdgeList::new();
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
    fn test_num_colors() {
        let mut c = Coloring::new();

        for u in 0..100 {
            c.insert(u, u % 11);
        }

        assert_eq!(num_colors(&c), 11);
    }

    #[test]
    fn rs_color() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);

        let c = rs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn rs_color2() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);
        g.add_edge(1,3);

        let c = rs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn rs_line() {
        let mut g = EdgeList::new();

        for i in 0..10 {
            g.add_edge(i, i+1);
        }

        let c = rs_coloring(&g);

        assert!(check_coloring(&g, &c));

        // Line might not be 2-colored by rs
        // in case of unfortunate vertex ordering
        assert!(num_colors(&c) <= 3);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn rs_random() {
        let g = EdgeList::random(100, 0.5);

        let c = rs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn cs_color() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);

        let c = cs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn cs_color2() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);
        g.add_edge(1,3);

        let c = cs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn cs_line() {
        let mut g = EdgeList::new();

        for i in 0..10 {
            g.add_edge(i, i+1);
        }

        let c = cs_coloring(&g);

        assert!(check_coloring(&g, &c));

        // Line must be 2-colored by cs-coloring
        assert!(num_colors(&c) == 2);
    }

    #[test]
    fn cs_random() {
        let g = EdgeList::random(100, 0.5);

        let c = cs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn lf_color() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);

        let c = lf_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn lf_color2() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);
        g.add_edge(1,3);

        let c = lf_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn lf_line() {
        let mut g = EdgeList::new();

        for i in 0..10 {
            g.add_edge(i, i+1);
        }

        let c = lf_coloring(&g);

        assert!(check_coloring(&g, &c));

        // Line might not be 2-colored by lf coloring
        // in case of unfortunate vertex ordering
        assert!(num_colors(&c) <= 3);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn lf_random() {
        let g = EdgeList::random(100, 0.5);

        let c = lf_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn sl_color() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);

        let c = sl_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn sl_color2() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);
        g.add_edge(1,3);

        let c = sl_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn sl_line() {
        let mut g = EdgeList::new();

        for i in 0..10 {
            g.add_edge(i, i+1);
        }

        let c = sl_coloring(&g);

        assert!(check_coloring(&g, &c));

        // Line must be 2-colored by sl-coloring
        assert!(num_colors(&c) == 2);
    }

    #[test]
    fn sl_random() {
        let g = EdgeList::random(100, 0.5);

        let c = sl_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn tree_coloring() {
        let mut g = EdgeList::new();

        // Binary tree
        for i in 0..127 {
            g.add_edge(i, 2*i+1);
            g.add_edge(i, 2*i+2);
        }

        let c = rs_coloring(&g);
        let c1 = lf_coloring(&g);
        let c2 = sl_coloring(&g);
        let c3 = two_coloring(&g).unwrap();
        let c4 = cs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(check_coloring(&g, &c1));
        assert!(check_coloring(&g, &c2));
        assert!(check_coloring(&g, &c3));
        assert!(check_coloring(&g, &c4));

        assert!(num_colors(&c) <= 4);
        assert!(num_colors(&c1) <= 4);
        assert!(num_colors(&c2) == 2);
        assert_eq!(num_colors(&c3), 2);
        assert_eq!(num_colors(&c4), 2);
    }

    #[test]
    fn even_cycle_coloring() {
        let mut g = EdgeList::new();

        // even cycle
        let n = 128;
        for i in 0..n {
            g.add_edge(i, (i+1)%n);
        }

        let c = rs_coloring(&g);
        let c1 = lf_coloring(&g);
        let c2 = sl_coloring(&g);
        let c3 = two_coloring(&g);
        let c4 = cs_coloring(&g);

        // Even circle => bipartite => 2-colorable
        assert!(c3.is_some());
        let c3 = c3.unwrap();

        assert!(check_coloring(&g, &c));
        assert!(check_coloring(&g, &c1));
        assert!(check_coloring(&g, &c2));
        assert!(check_coloring(&g, &c3));
        assert!(check_coloring(&g, &c4));

        assert_eq!(num_colors(&c), 3);
        assert_eq!(num_colors(&c1), 3);
        assert_eq!(num_colors(&c2), 2);
        assert_eq!(num_colors(&c3), 2);
        assert_eq!(num_colors(&c4), 2);
    }

    #[test]
    fn odd_cycle_coloring() {
        let mut g = EdgeList::new();

        // odd cycle
        let n = 127;
        for i in 0..n {
            g.add_edge(i, (i+1)%n);
        }

        let c = rs_coloring(&g);
        let c1 = lf_coloring(&g);
        let c2 = sl_coloring(&g);
        let c3 = two_coloring(&g);
        let c4 = cs_coloring(&g);

        // Odd circle => not bipartite => not 2-colorable
        assert!(c3.is_none());

        assert!(check_coloring(&g, &c));
        assert!(check_coloring(&g, &c1));
        assert!(check_coloring(&g, &c2));
        assert!(check_coloring(&g, &c4));

        assert_eq!(num_colors(&c), 3);
        assert_eq!(num_colors(&c1), 3);
        assert_eq!(num_colors(&c2), 3);
        assert_eq!(num_colors(&c4), 3);
    }

    #[test]
    fn prism_coloring() {
        // Smallest slightly hard to color graph for SL
        let mut g = EdgeList::new();

        g.add_edge(1,2);
        g.add_edge(1,3);
        g.add_edge(2,3);
        g.add_edge(1,4);
        g.add_edge(2,5);
        g.add_edge(3,6);
        g.add_edge(4,5);
        g.add_edge(4,6);
        g.add_edge(5,6);

        let c = rs_coloring(&g);
        let c1 = lf_coloring(&g);
        let c2 = sl_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(check_coloring(&g, &c1));
        assert!(check_coloring(&g, &c2));

        assert!(num_colors(&c) <= 4);
        assert!(num_colors(&c1) <= 4);
        assert!(num_colors(&c2) <= 4);

        assert!(num_colors(&c) >= 3);
        assert!(num_colors(&c1) >= 3);
        assert!(num_colors(&c2) >= 3);
    }

    #[test]
    fn two_color() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);

        let c = two_coloring(&g);

        assert!(c.is_some());
        let c = c.unwrap();

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn two_color2() {
        let mut g = EdgeList::new();

        g.add_edge(1,2);
        g.add_edge(1,3);

        let c = two_coloring(&g);

        assert!(c.is_some());
        let c = c.unwrap();

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn two_line() {
        let mut g = EdgeList::new();

        for i in 0..10 {
            g.add_edge(i, i+1);
        }

        let c = two_coloring(&g);

        assert!(c.is_some());
        let c = c.unwrap();

        assert!(check_coloring(&g, &c));

        // Line must be 2-colored by two-coloring
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn two_fail() {
        let mut g = EdgeList::new();

        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);

        let c = two_coloring(&g);

        assert!(c.is_none());
    }

    #[test]
    fn random_coloring() {
        let g1 = EdgeList::random(100, 0.5);
        let g2 = AdjMatrix::random(100, 0.5);

        let c = rs_coloring(&g1);
        let c1 = lf_coloring(&g1);
        let c2 = sl_coloring(&g1);
        let c3 = cs_coloring(&g1);

        assert!(check_coloring(&g1, &c));
        assert!(check_coloring(&g1, &c1));
        assert!(check_coloring(&g1, &c2));
        assert!(check_coloring(&g1, &c3));

        let c = rs_coloring(&g2);
        let c1 = lf_coloring(&g2);
        let c2 = sl_coloring(&g2);
        let c3 = cs_coloring(&g2);

        assert!(check_coloring(&g2, &c));
        assert!(check_coloring(&g2, &c1));
        assert!(check_coloring(&g2, &c2));
        assert!(check_coloring(&g2, &c3));
    }
}
