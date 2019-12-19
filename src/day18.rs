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
use std::ops::{Deref, DerefMut};
use crate::util::{self, BoundingBox2D, Point2D, Vector2D};

const TILE_WALL: char = '#';
const TILE_FLOOR: char = '.';
const TILE_START: char = '@';
const DIRECTIONS: [Vector2D; 4] = [vector!(0, -1), vector!(1, 0), vector!(0, 1), vector!(-1, 0)];

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
enum Node {
    Start,
    Key(char),
}

#[derive(Clone,Debug)]
struct Edge {
    cost: usize,
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

#[derive(Debug)]
struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
    bbox: BoundingBox2D,
    start: Point2D,
}

impl Map {
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
        let mut map = Map {data, width, height, bbox, start: point!(0, 0)};
        map.start = map.bbox.iter()
            .find(|p| map.get(p).unwrap() == TILE_START)
            .unwrap();
        return map;
    }

    fn get(&self, p: &Point2D) -> Option<char> {
        if !self.bbox.contains(p) {
            None
        } else {
            Some(self.data[p.y as usize * self.width + p.x as usize])
        }
    }
}

struct QueueOnce<T: Copy + Eq + Hash> {
    queue: VecDeque<T>,
    seen: HashSet<T>,
}

impl<T: Copy + Eq + Hash> QueueOnce<T> {
    fn new() -> QueueOnce<T> {
        QueueOnce {
            queue: VecDeque::new(),
            seen: HashSet::new(),
        }
    }

    fn push_back(&mut self, x: T) {
        if self.seen.insert(x) {
            self.queue.push_back(x);
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}

#[derive(Clone,Debug)]
struct PathDB {
    /// Adjacency map for traversing between nodes
    adjacent: HashMap<Node, HashMap<Node, Edge>>,
    /// Dependencies (key ownership) that must be satisfied to get from Start to a particular node
    dependencies: HashMap<Node, HashSet<Node>>,
    /// Nodes that are reachable by following a particular edge without backtracking
    reachable: HashMap<(Node, Node), HashSet<Node>>,
}

impl PathDB {
    fn new() -> PathDB {
        PathDB {
            adjacent: HashMap::new(),
            dependencies: HashMap::from(vec![(Node::Start, HashSet::new())].into_iter().collect()),
            reachable: HashMap::new(),
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
        if let Some(a_deps) = self.dependencies.get(&a) {
            b_deps.extend(a_deps);
        }
        // (Update the dependency set)
        self.dependencies.entry(b).or_insert(HashSet::new()).extend(b_deps);
    }

    fn get_adjacent_nodes(&self, n: Node, from: Node) -> HashSet<Node> {
        self.adjacent
            .get(&n).unwrap()
            .iter()
            .filter_map(|(&k, _v)| if k == from { None } else { Some(k) })
            .collect()
    }

    /// Get valid extensions of `path`, taking into account dependencies and nodes already visited
    fn continue_path(&self, path: &[Node]) -> Vec<Vec<Node>> {
        let mut paths: Vec<Vec<Node>> = Vec::new();
        let keys = HashSet::from_iter(path.iter().cloned());
        for (next, deps) in self.dependencies.iter() {
            if !keys.contains(next) && deps.difference(&keys).count() == 0 {
                let mut next_path = Vec::new();
                next_path.extend_from_slice(path);
                next_path.push(*next);
                paths.push(next_path);
            }
        }
        return paths;
    }
}

impl From<&Map> for PathDB {
    fn from(map: &Map) -> Self {
        // TODO: check than the acyclic graph assumption holds true - should only see each node once
        let mut paths = PathDB::new();
        let mut queue: VecDeque<(Point2D, Edge, Point2D, Node)> = VecDeque::new();
        queue.push_back((map.start.clone(), Edge::new(), map.start.clone(), Node::Start));

        while let Some((pos, edge, from_pos, from_node)) = queue.pop_front() {
            for d in DIRECTIONS.iter().cloned() {
                let next = pos + d;
                // Don't backtrack
                if next == from_pos {
                    continue;
                }
                match map.get(&next) {
                    // Shouldn't re-visit start position in flood fill, but let's have an exhaustive match here
                    Some(TILE_START) => panic!("revisited start location!?!?"),
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
struct PathGenerator<'a> {
    path_db: &'a PathDB,
    stack: Vec<Vec<Node>>,
}

impl<'a> PathGenerator<'a> {
    fn new(path_db: &'a PathDB) -> PathGenerator<'a> {
        PathGenerator {
            path_db,
            stack: vec![vec![Node::Start]],
        }
    }
}

impl<'a> Iterator for PathGenerator<'a> {
    type Item = Vec<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        // Loop until we return something
        loop {
            if let Some(path) = self.stack.pop() {
                let next_paths = self.path_db.continue_path(&path);
                if next_paths.is_empty() {
                    break Some(path);
                } else {
                    self.stack.extend(next_paths);
                }
            } else {
                break None;
            }
        }
    }
}

#[derive(Debug)]
struct Reachable<'a> {
    path_db: &'a PathDB,
    cache: HashMap<(Node, Node), HashSet<Node>>,
}

/// A caching calculator of reachable node sets for each `from -> to` edge
impl<'a> Reachable<'a> {
    fn new(path_db: &'a PathDB) -> Reachable<'a> {
        Reachable {
            path_db,
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, from: Node, to: Node) -> HashSet<Node> {
        if let Some(set) = self.cache.get(&(from, to)).cloned() {
            set
        } else {
            let mut set: HashSet<Node> = HashSet::new();
            // Obviously can reach `to` by following `from -> to`
            set.insert(to);
            // Recursively include anything reachable from `to`
            for node in self.path_db.get_adjacent_nodes(to, from) {
                let more = self.get(to, node);
                set.extend(more);
            }
            self.cache.insert((from, to), set.clone());
            set
        }
    }

    /// Because of the DAG nature of our map, there's only one possible route between 2 nodes
    fn get_path(&mut self, from: Node, to: Node) -> Path {
        let mut path = Path {
            start: from,
            route: Vec::new(),
            cost: 0,
            requirements: HashSet::new(),
        };
        let mut prev = from;
        let mut curr = from;
        // Loop until we find the destination
        'outer: while curr != to {
            // Look at possible next nodes
            for next in self.path_db.get_adjacent_nodes(curr, prev) {
                // See if destination is reachable via this node
                if self.get(curr, next).contains(&to) {
                    let edge = self.path_db.adjacent.get(&curr).unwrap().get(&next).unwrap();
                    path.route.push(next);
                    path.cost += edge.cost;
                    path.requirements.extend(edge.requirements.iter());
                    prev = curr;
                    curr = next;
                    continue 'outer;
                }
            }
            panic!(format!("no route between {:?} and {:?}", from, to));
        }
        return path;
    }
}

#[derive(Debug)]
struct Path {
    start: Node,
    route: Vec<Node>,
    cost: usize,
    requirements: HashSet<Node>,
}

fn shortest_path(filename: &str) -> usize {
    let map = Map::from_data_file(filename);
    let paths = PathDB::from(&map);
    println!("paths: {:?}", paths);
//    let path = paths.topological_sort();
//    println!("path: {:?}", path);
    let mut reachable = Reachable::new(&paths);
    println!("Start -> Key('a') reachable: {:?}", reachable.get(Node::Start, Node::Key('a')));
    let path_f_to_c = reachable.get_path(Node::Key('f'), Node::Key('c'));
    println!("Key('f') -> Key('c') route: {:?}", path_f_to_c);
    let mut pathgen = PathGenerator::new(&paths);
    for p in pathgen {
        println!("path: {:?}", p);
    }
    0
}

pub fn part1() -> i32 {
    0
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_example1() {
//        assert_eq!(shortest_path("day18_example1.txt"), 8);
    }

    #[test]
    fn test_shortest_path_example2() {
        assert_eq!(shortest_path("day18_example2.txt"), 86);
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
