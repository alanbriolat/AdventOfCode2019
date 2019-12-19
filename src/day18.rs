/*
Notes
=====

The only points of interest for path-finding (for part 1) are the starting point and the keys. The
map can be simplified to a graph of these nodes, with edges that consist of a path length - number
of steps - and a set of requirements - keys that must have been acquired already to traverse the
edge.

The 1-tile-wide tunnels, and apparent lack of cycles, constrain the problem such that if B is
adjacent to A and C, we must travel through B to get from A to C, and there is only ever one path
between 2 nodes. These properties should also allow converting the map to a graph of node
connectivity with a flood-fill algorithm.

The solution looks like a variant of the Travelling Salesman Problem: we must visit every node, with
the minimum total cost. However, the key/door behaviour adds a dependency tree aspect. In theory,
the dependency tree should constrain the TSP to a more reasonable set of possibilities than O(n!).

*/
use std::collections::{HashSet, VecDeque, HashMap};
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{self, Deref, DerefMut};
use crate::util::{self, BoundingBox2D, Point2D, Vector2D};

const TILE_WALL: char = '#';
const TILE_FLOOR: char = '.';
const TILE_ENTRANCE: char = '@';
const DIRECTIONS: [Vector2D; 4] = [vector!(0, -1), vector!(1, 0), vector!(0, 1), vector!(-1, 0)];


/// Node: a point of interest in the map
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
enum Node {
    Entrance,
    Key(char),
}

/// Route: a sequence of nodes to visit, without specifics of adjacency, cost, etc.
#[derive(Clone,Debug)]
struct Route(Vec<Node>);
deref!(Route, Vec<Node>);

impl Route {
    fn new() -> Route {
        Route(Vec::new())
    }

    /// Iterate over `(from, to)` pairs along the route.
    fn segments<'a>(&'a self) -> impl Iterator<Item=(Node, Node)> + 'a {
        self.0.windows(2).map(|w| (w[0], w[1]))
    }
}


/// Edge: a connection between adjacent nodes
#[derive(Clone,Debug)]
struct Edge {
    /// Cost: the number of steps to get between the two nodes
    cost: usize,
    /// Requirements: the keys that must be held (i.e. nodes that must have been visited) to use the edge
    requirements: HashSet<Node>,
}

impl Edge {
    fn new() -> Edge {
        Edge {
            cost: 0,
            // TODO: optimisation: use a u32, use 1 bit per key, check requirements with bitmasks
            requirements: HashSet::new(),
        }
    }
}


/// Map: the 2D tile representation of the input map
#[derive(Debug)]
struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
    bbox: BoundingBox2D,
    entrance: Point2D,
}

impl Map {
    /// Construct the map from an input file
    fn from_data_file(filename: &str) -> Map {
        let lines = util::read_lines(filename);
        let height = lines.len();
        let width = lines[0].len();
        let mut bbox = BoundingBox2D::new(&point!(0, 0));
        bbox.include(&point!(width as i32 - 1, height as i32 - 1));
        let mut data = Vec::new();
        data.reserve(width * height);
        for line in lines {
            data.extend(line.chars())
        }
        let mut map = Map {data, width, height, bbox, entrance: point!(0, 0)};
        map.entrance = map.bbox.iter()
            .find(|p| map.get(p).unwrap() == TILE_ENTRANCE)
            .unwrap();
        return map;
    }

    /// Get the tile character at `p`
    fn get(&self, p: &Point2D) -> Option<char> {
        if !self.bbox.contains(p) {
            None
        } else {
            Some(self.data[p.y as usize * self.width + p.x as usize])
        }
    }
}


/// Node graph: an acyclic graph representation of the input map, containing only information
/// relating to nodes and moving between them.
#[derive(Clone,Debug)]
struct NodeGraph {
    /// Adjacency map for traversing between nodes
    adjacent: HashMap<Node, HashMap<Node, Edge>>,
    /// Requirements (nodes visited AKA keys held) that must be met to visit a node for the first
    /// time, i.e. the sum of all edge requirements to get to each node from Entrance
    requirements: HashMap<Node, HashSet<Node>>,
}

impl NodeGraph {
    fn new() -> NodeGraph {
        NodeGraph {
            adjacent: HashMap::new(),
            requirements: HashMap::from(vec![(Node::Entrance, HashSet::new())].into_iter().collect()),
        }
    }

    /// Add an edge from `a` to `b` specified by `e`
    ///
    /// Also adds the reverse edge, but the `a -> b` direction is used to determine the dependency
    /// graph.
    fn add_edge(&mut self, a: Node, b: Node, e: Edge) {
        // Add edge from a to b
        self.adjacent.entry(a).or_insert(HashMap::new()).insert(b, e.clone());
        // Add same edge from b to a
        self.adjacent.entry(b).or_insert(HashMap::new()).insert(a, e.clone());

        // Record the dependencies for getting to b:
        // 1) Must have been to every node in the edge's requirements (i.e. picked up the relevant keys)
        let mut b_deps: HashSet<Node> = e.requirements.clone();
        // 2) Must have satisfied the requirements to get to a first
        if let Some(a_deps) = self.requirements.get(&a) {
            b_deps.extend(a_deps);
        }
        // (Update the dependency set)
        self.requirements.entry(b).or_insert(HashSet::new()).extend(b_deps);
    }

    /// Get set of nodes adjacent to `n`, excluding `from`
    fn get_adjacent_nodes(&self, n: Node, from: Node) -> HashSet<Node> {
        let mut adjacent: HashSet<Node> = self.adjacent.get(&n)
            .map(|x| x.keys().cloned().collect())
            .unwrap_or(HashSet::new());
        adjacent.remove(&from);
        return adjacent;
    }

    /// Get valid extensions of `route`, taking into account dependencies and nodes already visited
    fn continue_route(&self, route: &Route) -> Vec<Route> {
        let mut routes: Vec<Route> = Vec::new();
        let keys = HashSet::from_iter(route.iter().cloned());
        for (next, deps) in self.requirements.iter() {
            if !keys.contains(next) && deps.difference(&keys).count() == 0 {
                let mut next_route = Route::new();
                next_route.extend_from_slice(route);
                next_route.push(*next);
                routes.push(next_route);
            }
        }
        return routes;
    }
}

impl From<&Map> for NodeGraph {
    fn from(map: &Map) -> Self {
        // TODO: check than the acyclic graph assumption holds true - should only see each node once
        let mut paths = NodeGraph::new();
        let mut queue: VecDeque<(Point2D, Edge, Point2D, Node)> = VecDeque::new();
        queue.push_back((map.entrance.clone(), Edge::new(), map.entrance.clone(), Node::Entrance));

        while let Some((pos, edge, from_pos, from_node)) = queue.pop_front() {
            for d in DIRECTIONS.iter().cloned() {
                let next = pos + d;
                // Don't backtrack
                if next == from_pos {
                    continue;
                }
                match map.get(&next) {
                    // Shouldn't re-visit entrance position in flood fill, but let's have an exhaustive match here
                    Some(TILE_ENTRANCE) => panic!("revisited entrance location!?!?"),
                    // Wall or out of bounds: do nothing
                    Some(TILE_WALL) | None => {},
                    // Floor: just advance one step
                    Some(TILE_FLOOR) => {
                        queue.push_back((next, Edge{cost: edge.cost + 1, requirements: edge.requirements.clone()}, pos.clone(), from_node));
                    },
                    // Door: add to the set of requirements, advance one step
                    Some(door) if 'A' <= door && door <= 'Z' => {
                        let mut requirements = edge.requirements.clone();
                        // Convert door to the required key
                        requirements.insert(Node::Key(door.to_ascii_lowercase()));
                        queue.push_back((next, Edge{cost: edge.cost + 1, requirements}, pos.clone(), from_node));
                    },
                    // Key: end path and record it, start new path
                    Some(key) if 'a' <= key && key <= 'z' => {
                        let node = Node::Key(key);
                        paths.add_edge(from_node, node, Edge{cost: edge.cost + 1, requirements: edge.requirements.clone()});
                        queue.push_back((next, Edge::new(), pos.clone(), node));
                    },
                    unknown => panic!(format!("unknown tile: {:?}", unknown)),
                }
            }
        }
        return paths;
    }
}

/// Depth-first-search iteration of valid node visit orderings
#[derive(Clone,Debug)]
struct RouteGenerator<'a> {
    nodegraph: &'a NodeGraph,
    stack: Vec<Route>,
}

impl<'a> RouteGenerator<'a> {
    fn new(nodegraph: &'a NodeGraph) -> RouteGenerator<'a> {
        RouteGenerator {
            nodegraph,
            stack: vec![Route(vec![Node::Entrance])],
        }
    }
}

impl<'a> Iterator for RouteGenerator<'a> {
    type Item = Route;

    fn next(&mut self) -> Option<Self::Item> {
        // Loop until we return something
        loop {
            if let Some(route) = self.stack.pop() {
                let next_routes = self.nodegraph.continue_route(&route);
                if next_routes.is_empty() {
                    break Some(route);
                } else {
                    self.stack.extend(next_routes);
                }
            } else {
                break None;
            }
        }
    }
}


/// A caching calculator of paths, reachable sets, etc.
#[derive(Debug)]
struct PathCache<'a> {
    /// The node graph this path cache relates to (assumes that the node graph is fully populated).
    nodegraph: &'a NodeGraph,
    /// See get_reachable().
    reachable: HashMap<(Node, Node), HashSet<Node>>,
    /// See get_path().
    paths: HashMap<(Node, Node), Path>,
}

impl<'a> PathCache<'a> {
    fn new(nodegraph: &'a NodeGraph) -> PathCache<'a> {
        PathCache {
            nodegraph,
            reachable: HashMap::new(),
            paths: HashMap::new(),
        }
    }

    /// Get the set of nodes reachable in the direction of `from -> to`.
    ///
    /// `(from, to)` must be a path segment, i.e. adjacent, not an abstract route segment.
    fn get_reachable(&mut self, from: Node, to: Node) -> HashSet<Node> {
        assert!(
            self.nodegraph.adjacent.get(&from).and_then(|x| x.get(&to)).is_some(),
            "get_reachable: from and to must be adjacent",
        );
        if let Some(set) = self.reachable.get(&(from, to)).cloned() {
            set
        } else {
            let mut set: HashSet<Node> = HashSet::new();
            // Obviously can reach `to` by following `from -> to`
            set.insert(to);
            // Recursively include anything reachable from `to`
            for node in self.nodegraph.get_adjacent_nodes(to, from) {
                let more = self.get_reachable(to, node);
                set.extend(more);
            }
            self.reachable.insert((from, to), set.clone());
            set
        }
    }

    /// Get the path to travel the `from -> to` route segment.
    ///
    /// Every node is connected to the graph, and the graph has no cycles, so there is exactly one
    /// non-backtracking route between any 2 nodes.
    fn get_path(&mut self, from: Node, to: Node) -> Path {
        // TODO: caching
        let mut path = Path {
            route: Route(vec![from]),
            cost: 0,
        };
        let mut prev = from;
        let mut curr = from;
        // Loop until we find the destination
        'outer: while curr != to {
            // Look at possible next nodes
            for next in self.nodegraph.get_adjacent_nodes(curr, prev) {
                // See if destination is reachable via this node
                if self.get_reachable(curr, next).contains(&to) {
                    let edge = self.nodegraph.adjacent.get(&curr).unwrap().get(&next).unwrap();
                    path.route.push(next);
                    path.cost += edge.cost;
                    prev = curr;
                    curr = next;
                    continue 'outer;
                }
            }
            panic!(format!("no route between {:?} and {:?}", from, to));
        }
        return path;
    }

    /// Get the concrete path to travel the abstract `route`.
    fn get_path_from_route(&mut self, route: &Route) -> Path {
        let mut path = Path {
            route: Route(vec![route[0]]),
            cost: 0,
        };
        for (from, to) in route.segments() {
            path += &self.get_path(from, to);
        }
        return path;
    }
}


/// Path: a route where consecutive nodes are always adjacent, with cost calculated too
#[derive(Clone,Debug)]
struct Path {
    route: Route,
    cost: usize,
}

impl Path {
    /// Iterate over `(from, to)` pairs along the path.
    fn segments<'a>(&'a self) -> impl Iterator<Item=(Node, Node)> + 'a {
        self.route.windows(2).map(|w| (w[0], w[1]))
    }
}

impl ops::Add<&Path> for Path {
    type Output = Path;

    fn add(self, rhs: &Path) -> Self::Output {
        assert!(self.route.len() == 0 || rhs.route.len() == 0 || self.route.last() == rhs.route.first());
        let mut path = self.clone();
        path += rhs;
        return path;
    }
}

impl ops::AddAssign<&Path> for Path {
    fn add_assign(&mut self, rhs: &Path) {
        assert!(self.route.len() == 0 || rhs.route.len() == 0 || self.route.last() == rhs.route.first());
        self.route.extend(rhs.route.iter().skip(1));
        self.cost += rhs.cost;
    }
}


fn shortest_path(filename: &str) -> usize {
    let map = Map::from_data_file(filename);
    let node_graph = NodeGraph::from(&map);
    let mut path_cache = PathCache::new(&node_graph);
    let mut route_gen = RouteGenerator::new(&node_graph);
    println!("number of routes: {}", route_gen.clone().count());
    route_gen.map(|r| {
        println!("route: {:?}", &r);
        path_cache.get_path_from_route(&r).cost
    }).min().unwrap()
}

pub fn part1() -> usize {
    shortest_path("day18_example2.txt")
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_example1() {
        assert_eq!(shortest_path("day18_example1.txt"), 8);
    }

    #[test]
    fn test_shortest_path_example2() {
        assert_eq!(shortest_path("day18_example2.txt"), 86);
    }

    #[test]
    fn test_shortest_path_example3() {
        assert_eq!(shortest_path("day18_example3.txt"), 132);
    }

    #[test]
    fn test_shortest_path_example4() {
        assert_eq!(shortest_path("day18_example4.txt"), 136);
    }

    #[test]
    fn test_shortest_path_example5() {
        assert_eq!(shortest_path("day18_example5.txt"), 81);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), unimplemented!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
