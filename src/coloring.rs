use rand::random;
use std::collections::{HashSet, VecDeque};

use graph::StaticGraph;

/// Coloring type.
/// This maps from vertices to colors.
pub type Coloring = Vec<usize>;

/// Coloring heuristics implemented here.
#[derive(Debug, Clone, Copy)]
pub enum ColoringAlgo {
    RS,
    CS,
    LF,
    SL,
    SDO,
}

/// Color the graph with all available methods and return the best coloring.
pub fn color<G: StaticGraph>(graph: &G) -> Coloring {
    let mut colorings = Vec::new();
    colorings.push(rs_coloring(graph));
    colorings.push(cs_coloring(graph));
    colorings.push(lf_coloring(graph));
    colorings.push(sl_coloring(graph));
    colorings.push(sdo_coloring(graph));

    colorings
        .into_iter()
        .map(|c| {
            assert!(check_coloring(graph, &c));
            c
        })
        .min_by_key(|c| num_colors(&c))
        .unwrap()
}

/// Check whether coloring defines a color for all vertices that exist in the graph.
pub fn compatible_coloring<G: StaticGraph>(graph: &G, coloring: &Coloring) -> bool {
    graph.num_vertices() == coloring.len()
}

/// Check whether no adjacent vertices are in conflict.
/// ```false``` indicates either a color conflict or no color defined for at least
/// one of the vertices in the graph.
pub fn check_coloring<G: StaticGraph>(graph: &G, coloring: &Coloring) -> bool {
    if !compatible_coloring(graph, coloring) {
        return false;
    }

    for (u, v) in graph.edges() {
        if coloring[u] == coloring[v] {
            return false;
        }
    }

    true
}

/// Returns the number of colors used in the coloring.
pub fn num_colors(coloring: &Coloring) -> usize {
    let mut colors: HashSet<usize> = HashSet::new();

    for &val in coloring.iter() {
        colors.insert(val);
    }

    colors.len()
}

/// Returns a 2-coloring of the graph if it exists, ```None``` otherwise.
/// Can be used as a check for bipartiteness.
pub fn two_coloring<G: StaticGraph>(graph: &G) -> Option<Coloring> {
    let mut c: Vec<Option<usize>> = vec![None; graph.num_vertices()];
    let mut q = VecDeque::new();

    let first = 0;
    q.push_back(first);
    c[first] = Some(0);

    while let Some(v) = q.pop_front() {
        let color = c[v].unwrap();

        for u in graph.neighbors(v) {
            if let Some(col) = c[u] {
                // Conflict
                if col == color {
                    return None;
                }
            } else {
                // Color neighbors opposite color and put in the frontier
                c[u] = Some(1 - color);
                q.push_back(u);
            }
        }
    }

    // Set any isolated vertices to color '0'
    for v in c.iter_mut() {
        if v.is_none() {
            *v = Some(0);
        }
    }

    let coloring: Option<Coloring> = c.into_iter().collect();
    assert!(coloring.is_some());
    let coloring = coloring.unwrap();

    Some(coloring)
}

/// Greedy coloring algorithm.
/// Colors the vertices in the sequence provided by chosing the
/// smallest color not in conflict.
pub fn greedy_coloring<G: StaticGraph>(
    graph: &G,
    vertices: impl Iterator<Item = usize>,
) -> Coloring {
    // Must be equal to 'vertices.count()'
    // as 'vertices' must be permutation of 'graph.vertices'
    let n = graph.num_vertices();
    let mut c: Vec<Option<usize>> = vec![None; n];

    let mut blocked_colors = vec![false; n];
    for v in vertices {
        for u in graph.neighbors(v) {
            if let Some(color) = c[u] {
                blocked_colors[color] = true;
            }
        }

        for x in 0..n {
            if !blocked_colors[x] {
                c[v] = Some(x);
                break;
            }
        }

        blocked_colors = vec![false; n];
    }

    let coloring: Option<Coloring> = c.into_iter().collect();
    assert!(coloring.is_some());

    coloring.unwrap()
}

/// Returns a random-sequence greedy coloring of the graph where the vertices have
/// been colored in random order.
/// There is no guarantee about the number of colors used.
pub fn rs_coloring<G: StaticGraph>(graph: &G) -> Coloring {
    // No sequence building stage for this algorithm
    greedy_coloring(graph, graph.vertices())
}

/// Returns a connected-sequence greedy coloring of the graph where the vertices have
/// been colored in an order such that each vertex (except the first) has atleast one
/// neighbor that has already been colored.
pub fn cs_coloring<G: StaticGraph>(graph: &G) -> Coloring {
    // Sequence building stage
    let mut visited = vec![false; graph.num_vertices()];
    let mut vec: Vec<usize> = Vec::new();

    let n = graph.vertices().count();

    let first = graph.vertices().next().unwrap();
    visited[first] = true;
    vec.push(first);

    let mut i = 0;
    while i < vec.len() {
        let v = vec[i];

        for u in graph.neighbors(v) {
            if !visited[u] {
                vec.push(u);
                visited[u] = true;
            }
        }
        i += 1;
    }

    for v in 0..n {
        if !visited[v] {
            vec.push(v);
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

    vertices.sort_by(|a, b| b.1.cmp(&a.1));

    greedy_coloring(graph, vertices.iter().map(|&(v, _)| v))
}

/// Returns a smallest-last greedy coloring of the graph.
/// This algorithm optimally colors trees, cycles and other types of graphs.
/// For general graphs there is no guarantee about the number of colors used.
pub fn sl_coloring<G: StaticGraph>(graph: &G) -> Coloring {
    // Sequence building stage
    // Inefficient implementation
    let n = graph.num_vertices();
    let mut k_set = vec![false; n];
    let mut k = Vec::with_capacity(n);
    let mut notk = graph.vertices().collect::<HashSet<usize>>();

    while k.len() < n {
        let mut min_d = std::usize::MAX;
        let mut min_d_idx = 0;
        for &v in notk.iter() {
            // Only look at vertices not in k
            if k_set[v] {
                continue;
            }

            // Look for min degree of vertices not in k
            let mut degree = 0;
            for u in graph.neighbors(v) {
                if !k_set[u] {
                    degree += 1;
                }
            }
            if degree < min_d {
                min_d = degree;
                min_d_idx = v;
            }
        }

        k.push(min_d_idx);
        k_set[min_d_idx] = true;
        notk.remove(&min_d_idx);
    }

    // Greedy coloring with reversed order of k
    greedy_coloring(graph, k.iter().rev().cloned())
}

/// Returns a saturation degree ordered coloring of the graph.
/// The SDO is defined by the number of distinct colors in the neighborhood -
/// vertices with a high saturation degree are colored first.
/// For general graphs there is no guarantee about the number of colors used.
pub fn sdo_coloring<G: StaticGraph>(graph: &G) -> Coloring {
    let n = graph.num_vertices();
    let mut c = vec![None; n];

    let mut left = graph.vertices().collect::<HashSet<usize>>();

    while !left.is_empty() {
        // Find vertex with highest saturation degree
        let mut colors = HashSet::new();
        let mut max_sd = 0;
        let mut max_sd_idx = 0;
        for &v in left.iter() {
            for u in graph.neighbors(v) {
                if let Some(color) = c[u] {
                    colors.insert(color);
                }
            }

            if colors.len() > max_sd || (colors.len() == max_sd && graph.neighbors(v).count() > graph.neighbors(max_sd_idx).count()) {
                max_sd = colors.len();
                max_sd_idx = v;
            }

            colors.clear();
        }

        // Reacquire blocking colors for chosen vertex
        for u in graph.neighbors(max_sd_idx) {
            if let Some(color) = c[u] {
                colors.insert(color);
            }
        }

        // Color vertex
        for x in 0..n {
            if !colors.contains(&x) {
                c[max_sd_idx] = Some(x);
                break;
            }
        }

        colors.clear();
        left.remove(&max_sd_idx);
    }

    let coloring: Option<Coloring> = c.into_iter().collect();
    assert!(coloring.is_some());

    coloring.unwrap()
}

/// Perform provided coloring method on graph ```n``` times and return the coloring with
/// minimal number of colors. This is useful for randomized coloring methods such as
/// ```sl_coloring``` and ```sdo_coloring``` to get more robust results.
pub fn repeat_coloring<G: StaticGraph>(g: &G, c: fn(&G) -> Coloring, n: usize) -> Coloring {
    let mut best_c = c(g);
    let mut nc_best_c = num_colors(&best_c);

    for _ in 1..n {
        let new_c = c(g);
        let nc_new_c = num_colors(&new_c);
        if nc_new_c < nc_best_c {
            best_c = new_c;
            nc_best_c = nc_new_c;
        }
    }

    best_c
}

/// Fixes a potentially wrong coloring by choosing the lowest available color
/// for the vertex with lower saturation degree of any conflicting edge.
pub fn fix_coloring<G: StaticGraph>(g: &G, c: &mut Coloring) {
    for (u, v) in g.edges() {
        // Conflict
        if c[u] == c[v] {
            let u_colors = g.neighbors(u).map(|x| c[x]).collect::<HashSet<usize>>();
            let v_colors = g.neighbors(v).map(|x| c[x]).collect::<HashSet<usize>>();

            let mut min_u_color = g.num_vertices();
            let mut min_v_color = g.num_vertices();
            for x in 0..g.num_vertices() {
                if !u_colors.contains(&x) {
                    min_u_color = x;
                    break;
                }
            }
            for x in 0..g.num_vertices() {
                if !v_colors.contains(&x) {
                    min_v_color = x;
                    break;
                }
            }

            if min_u_color < min_v_color {
                c[u] = min_u_color;
            } else {
                c[v] = min_v_color;
            }
        }
    }
}

pub fn genetic_coloring<G: StaticGraph>(g: &G) -> Coloring {
    let n = 100;
    let gen = 100;
    let n_vert = g.num_vertices();
    let mut colorings: Vec<Coloring> = Vec::new();

    // Random initialization
    for _ in 0..n {
        colorings.push(sl_coloring(g));
    }

    for _ in 0..gen {
        colorings.sort_by(|a, b| num_colors(a).cmp(&num_colors(b)));

        for i in n / 2..n {
            let mom = random::<usize>() % (n / 2);
            let dad = random::<usize>() % (n / 2);

            let split = random::<usize>() % n_vert;

            for j in 0..n_vert {
                if j < split {
                    colorings[i][j] = colorings[mom][j];
                } else {
                    colorings[i][j] = colorings[dad][j];
                }
            }

            fix_coloring(g, &mut colorings[i]);
        }
    }

    colorings.remove(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;

    #[test]
    fn coloring_creation_empty() {
        let g = AdjList::new();
        let c = Coloring::new();

        assert!(check_coloring(&g, &c));
    }

    #[test]
    fn coloring_creation_fail() {
        let mut g = AdjList::new();
        let c = Coloring::new();

        g.add_edge(0, 1);

        assert!(!check_coloring(&g, &c));
        assert!(!compatible_coloring(&g, &c));
    }

    #[test]
    fn coloring_creation_success() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);

        let c = vec![0, 1];

        assert!(check_coloring(&g, &c));
    }

    #[test]
    fn coloring_creation_large() {
        let n = 100;
        let mut g = AdjList::new();
        let mut c = vec![0; n];

        for u in 0..n {
            for v in u..n {
                g.add_edge(u, v);
            }
        }

        for u in 0..100 {
            c[u] = u;
        }

        assert!(compatible_coloring(&g, &c));
        assert!(check_coloring(&g, &c));

        c[4] = 5;

        assert!(compatible_coloring(&g, &c));
        assert!(!check_coloring(&g, &c));
    }

    #[test]
    fn test_num_colors() {
        let n = 100;
        let mut c = vec![0; n];

        for u in 0..100 {
            c[u] = u % 11;
        }

        assert_eq!(num_colors(&c), 11);
    }

    #[test]
    fn rs_color() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);

        let c = rs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn rs_color2() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);

        let c = rs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn rs_line() {
        let mut g = AdjList::new();

        for i in 0..10 {
            g.add_edge(i, i + 1);
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
        let g = AdjList::random(100, 0.5);

        let c = rs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn cs_color() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);

        let c = cs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn cs_color2() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);

        let c = cs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn cs_line() {
        let mut g = AdjList::new();

        for i in 0..10 {
            g.add_edge(i, i + 1);
        }

        let c = cs_coloring(&g);

        assert!(check_coloring(&g, &c));

        // Line must be 2-colored by cs-coloring
        assert!(num_colors(&c) == 2);
    }

    #[test]
    fn cs_random() {
        let g = AdjList::random(100, 0.5);

        let c = cs_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn lf_color() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);

        let c = lf_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn lf_color2() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);

        let c = lf_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn lf_line() {
        let mut g = AdjList::new();

        for i in 0..10 {
            g.add_edge(i, i + 1);
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
        let g = AdjList::random(100, 0.5);

        let c = lf_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn sl_color() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);

        let c = sl_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn sl_color2() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);

        let c = sl_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn sl_line() {
        let mut g = AdjList::new();

        for i in 0..10 {
            g.add_edge(i, i + 1);
        }

        let c = sl_coloring(&g);

        assert!(check_coloring(&g, &c));

        // Line must be 2-colored by sl-coloring
        assert!(num_colors(&c) == 2);
    }

    #[test]
    fn sl_random() {
        let g = AdjList::random(100, 0.5);

        let c = sl_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn sdo_color() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);

        let c = sdo_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn sdo_color2() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);

        let c = sdo_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn sdo_line() {
        let mut g = AdjList::new();

        for i in 0..10 {
            g.add_edge(i, i + 1);
        }

        let c = sdo_coloring(&g);

        assert!(check_coloring(&g, &c));

        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn sdo_random() {
        let g = AdjList::random(100, 0.5);

        let c = sdo_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(num_colors(&c) <= g.vertices().count());
        assert!(num_colors(&c) >= 2);
        assert!(num_colors(&c) <= g.max_degree() + 1);
    }

    #[test]
    fn tree_coloring() {
        let mut g = AdjList::new();

        // Binary tree
        for i in 0..127 {
            g.add_edge(i, 2 * i + 1);
            g.add_edge(i, 2 * i + 2);
        }

        let c = rs_coloring(&g);
        let c1 = lf_coloring(&g);
        let c2 = sl_coloring(&g);
        let c3 = two_coloring(&g).unwrap();
        let c4 = cs_coloring(&g);
        let c5 = sdo_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(check_coloring(&g, &c1));
        assert!(check_coloring(&g, &c2));
        assert!(check_coloring(&g, &c3));
        assert!(check_coloring(&g, &c4));
        assert!(check_coloring(&g, &c5));

        assert!(num_colors(&c) <= 4);
        assert!(num_colors(&c1) <= 4);
        assert!(num_colors(&c2) == 2);
        assert_eq!(num_colors(&c3), 2);
        assert_eq!(num_colors(&c4), 2);
        assert_eq!(num_colors(&c5), 2);
    }

    #[test]
    fn even_cycle_coloring() {
        let mut g = AdjList::new();

        // even cycle
        let n = 128;
        for i in 0..n {
            g.add_edge(i, (i + 1) % n);
        }

        let c = rs_coloring(&g);
        let c1 = lf_coloring(&g);
        let c2 = sl_coloring(&g);
        let c3 = two_coloring(&g);
        let c4 = cs_coloring(&g);
        let c5 = sdo_coloring(&g);

        // Even circle => bipartite => 2-colorable
        assert!(c3.is_some());
        let c3 = c3.unwrap();

        assert!(check_coloring(&g, &c));
        assert!(check_coloring(&g, &c1));
        assert!(check_coloring(&g, &c2));
        assert!(check_coloring(&g, &c3));
        assert!(check_coloring(&g, &c4));
        assert!(check_coloring(&g, &c5));

        assert!(num_colors(&c) >= 2 && num_colors(&c) <= 3);
        assert!(num_colors(&c1) >= 2 && num_colors(&c) <= 3);
        assert_eq!(num_colors(&c2), 2);
        assert_eq!(num_colors(&c3), 2);
        assert_eq!(num_colors(&c4), 2);
        assert_eq!(num_colors(&c5), 2);
    }

    #[test]
    fn odd_cycle_coloring() {
        let mut g = AdjList::new();

        // odd cycle
        let n = 127;
        for i in 0..n {
            g.add_edge(i, (i + 1) % n);
        }

        let c = rs_coloring(&g);
        let c1 = lf_coloring(&g);
        let c2 = sl_coloring(&g);
        let c3 = two_coloring(&g);
        let c4 = cs_coloring(&g);
        let c5 = sdo_coloring(&g);

        // Odd circle => not bipartite => not 2-colorable
        assert!(c3.is_none());

        assert!(check_coloring(&g, &c));
        assert!(check_coloring(&g, &c1));
        assert!(check_coloring(&g, &c2));
        assert!(check_coloring(&g, &c4));
        assert!(check_coloring(&g, &c5));

        assert_eq!(num_colors(&c), 3);
        assert_eq!(num_colors(&c1), 3);
        assert_eq!(num_colors(&c2), 3);
        assert_eq!(num_colors(&c4), 3);
        assert_eq!(num_colors(&c5), 3);
    }

    #[test]
    fn prism_coloring() {
        // Smallest slightly hard to color graph for SL
        let mut g = AdjList::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(0, 3);
        g.add_edge(1, 4);
        g.add_edge(2, 5);
        g.add_edge(3, 4);
        g.add_edge(3, 5);
        g.add_edge(4, 5);

        let c = rs_coloring(&g);
        let c1 = lf_coloring(&g);
        let c2 = sl_coloring(&g);
        let c3 = cs_coloring(&g);
        let c4 = sdo_coloring(&g);

        assert!(check_coloring(&g, &c));
        assert!(check_coloring(&g, &c1));
        assert!(check_coloring(&g, &c2));
        assert!(check_coloring(&g, &c3));
        assert!(check_coloring(&g, &c4));

        assert!(num_colors(&c) <= 4);
        assert!(num_colors(&c1) <= 4);
        assert!(num_colors(&c2) <= 4);
        assert!(num_colors(&c3) <= 4);
        assert!(num_colors(&c4) <= 4);

        assert!(num_colors(&c) >= 3);
        assert!(num_colors(&c1) >= 3);
        assert!(num_colors(&c2) >= 3);
        assert!(num_colors(&c3) >= 3);
        assert!(num_colors(&c4) >= 3);
    }

    #[test]
    fn two_color() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);

        let c = two_coloring(&g);

        assert!(c.is_some());
        let c = c.unwrap();

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn two_color2() {
        let mut g = AdjList::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);

        let c = two_coloring(&g);

        assert!(c.is_some());
        let c = c.unwrap();

        assert!(check_coloring(&g, &c));
        assert_eq!(num_colors(&c), 2);
    }

    #[test]
    fn two_line() {
        let mut g = AdjList::new();

        for i in 0..10 {
            g.add_edge(i, i + 1);
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
        let mut g = AdjList::new();

        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);

        let c = two_coloring(&g);

        assert!(c.is_none());
    }

    #[test]
    fn random_coloring() {
        let g1 = AdjList::random(100, 0.5);
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
