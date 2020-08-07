use std::collections::{HashMap, VecDeque, HashSet, BTreeSet};
extern crate pathfinding;
use pathfinding::prelude::{astar,idastar};
use crate::util::{self, Grid2D, Point2D, Vector2D};
use std::cmp::Ordering;
use std::iter::{FromIterator};
use self::pathfinding::directed::fringe::fringe;
use self::pathfinding::directed::bfs::bfs;

const TILE_WALL: char = '#';
const TILE_FLOOR: char = '.';
const TILE_ENTRANCE: char = '@';
const DIRECTIONS: [Vector2D; 4] = [vector!(0, -1), vector!(1, 0), vector!(0, 1), vector!(-1, 0)];


/// Node: a point of interest in the map
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
enum Node {
    Entrance,
    Key(char),
    Door(char),
}


impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Node::Entrance, Node::Entrance) => Ordering::Equal,
            (Node::Entrance, Node::Key(_)) => Ordering::Less,
            (Node::Entrance, Node::Door(_)) => Ordering::Less,
            (Node::Key(a), Node::Key(b)) => a.cmp(b),
            (Node::Key(_), Node::Entrance) => Ordering::Greater,
            (Node::Key(_), Node::Door(_)) => Ordering::Less,
            (Node::Door(a), Node::Door(b)) => a.cmp(b),
            (Node::Door(_), Node::Entrance) => Ordering::Greater,
            (Node::Door(_), Node::Key(_)) => Ordering::Greater,
        }
    }
}


impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/// Map: the 2D tile representation of the input map.
#[derive(Debug)]
struct Map {
    grid: Grid2D<char>,
    nodes: HashMap<Node, Point2D>,
    adjacent: HashMap<Node, HashMap<Node, usize>>,
}

impl Map {
    /// Construct the map from an input file.
    fn from_data_file(filename: &str) -> Map {
        let lines = util::read_lines(filename);
        let height = lines.len();
        let width = lines[0].len();
        let grid: Grid2D<char> = Grid2D::new(width, height, lines.iter().flat_map(|s| s.chars()));
        let nodes: HashMap<Node, Point2D> = grid.iter()
            .filter_map(|(p, c)| {
                Self::char_to_node(*c).and_then(|n| Some((n, p)))
            })
            .collect();
        let mut map = Map { grid, nodes, adjacent: Default::default() };
        map.adjacent = map.nodes.keys().cloned()
            .map(|n| (n, map.find_adjacent(n)))
            .collect();
        return map;
    }

    /// Get the tile character at `p`.
    fn get(&self, p: &Point2D) -> Option<char> {
        self.grid.get(p).cloned()
    }

    /// Try to convert `c` to a Node value.
    fn char_to_node(c: char) -> Option<Node> {
        match c {
            TILE_FLOOR | TILE_WALL => None,
            TILE_ENTRANCE => Some(Node::Entrance),
            door if 'A' <= door && door <= 'Z' => Some(Node::Door(door.to_ascii_lowercase())),
            key if 'a' <= key && key <= 'z' => Some(Node::Key(key)),
            _ => None,
        }
    }

    /// Get the node at `p`, if that tile is a node
    fn get_node(&self, p: &Point2D) -> Option<Node> {
        self.get(p).and_then(Self::char_to_node)
    }

    /// Find nodes that can be reached from `n` without going via another node,
    /// along with the associated shortest path costs (number of steps).
    fn find_adjacent(&self, n: Node) -> HashMap<Node, usize> {
        let mut adjacent: HashMap<Node, usize> = HashMap::new();
        let mut visited: HashSet<Point2D> = HashSet::new();
        let mut queue: VecDeque<(Point2D, usize)> = VecDeque::new();
        let initial = self.nodes[&n];
        queue.push_back((initial, 0));
        visited.insert(initial);

        // Use flood fill to find adjacent nodes by not continuing past any node when found.
        // Queue-based algorithm is guaranteed to find the shortest path to each adjacent node.
        while let Some((p, cost)) = queue.pop_front() {
            for d in DIRECTIONS.iter().cloned() {
                let next = p + d;
                // Don't visit a tile more than once
                if !visited.insert(next) {
                    continue;
                }
                match self.get(&next) {
                    // Floor: keep going
                    Some(TILE_FLOOR) => {
                        queue.push_back((next, cost + 1));
                    },
                    // Wall or out of bounds: stop
                    Some(TILE_WALL) | None => {},
                    // Something else, should be a node
                    Some(c) => match Self::char_to_node(c) {
                        // A node: record the node and stop, because we're only looking for adjacent
                        // nodes. (Other nodes may be reachable by avoiding this one.)
                        Some(n) => {
                            adjacent.insert(n, cost + 1);
                        },
                        // Shouldn't be possible
                        None => panic!(format!("unknown node: {:?}", c)),
                    }
                }
            }
        }

        return adjacent;
    }

    fn find_path(&self, start: &SearchState, goal: Node) -> Option<(Vec<Node>, usize)> {
        let successors = |state: &SearchState| -> Vec<(SearchState, i32)> {
            let mut output: Vec<(SearchState, i32)> = Vec::new();
            for (node, cost) in self.adjacent.get(&state.position).unwrap_or(&HashMap::new()).iter() {
                if let Node::Door(c)  = node {
                    if !state.visited.contains(&Node::Key(*c)) {
//                        println!("skipping door: {:?}", node);
                        continue;
                    }
                }
                let mut visited = state.visited.clone();
                visited.insert(*node);
                output.push((SearchState{visited, position: *node}, *cost as i32));
            }
//            println!("successors for {:?}: {:?}", state, output);
            return output;
        };

        let heuristic = |state: &SearchState| -> i32 {
            (self.nodes[&state.position] - self.nodes[&goal]).manhattan_length()
        };

        let success = |state: &SearchState| -> bool {
            state.position == goal
        };

        return astar(start, successors, heuristic, success)
            .map(|(path, cost)| {
                (path.iter().map(|s| s.position).collect(), cost as usize)
            });
    }
}


#[derive(Clone,Debug,Eq,PartialEq,Hash)]
struct SearchState {
    visited: BTreeSet<Node>,
    position: Node,
}


impl SearchState {
    fn from_starting_node(n: Node) -> SearchState {
        SearchState {
            position: n,
            visited: BTreeSet::from_iter(vec![n]),
        }
    }
}


fn shortest_path(filename: &str) -> usize {
    let map = Map::from_data_file(filename);
    let all_keys: BTreeSet<Node> = map.nodes.keys().cloned()
        .filter(|n| if let Node::Key(_) = n { true } else { false })
        .collect();

    let mut path_cache: HashMap<(SearchState, Node), (Vec<Node>, usize)> = HashMap::new();

    // "Successor states": travelling to unvisited keys that are reachable
    let successors = |state: &SearchState| -> Vec<(SearchState, usize)> {
        let mut output: Vec<(SearchState, usize)> = Vec::new();
        // Iterate over unvisited keys only
        for key in all_keys.difference(&state.visited) {
            // Is there a path to the key?
            if let Some((path, cost)) = path_cache.get(&(state.clone(), *key)) {
                let visited = state.visited.union(&BTreeSet::from_iter(path.iter().cloned())).cloned().collect();
                output.push((SearchState{visited, position: *key}, *cost));
            } else if let Some((path, cost)) = map.find_path(state, *key) {
                path_cache.insert((state.clone(), *key), (path.clone(), cost));
                let visited = state.visited.union(&BTreeSet::from_iter(path)).cloned().collect();
                output.push((SearchState{visited, position: *key}, cost));
            }
        }
        return output;
    };

    // "Distance to goal" heuristic: the number of keys uncollected
    let heuristic = |state: &SearchState| -> usize {
        all_keys.difference(&state.visited).count()
    };

    // "Success": no keys unvisited
    let success = |state: &SearchState| -> bool {
        heuristic(state) == 0
    };

    let result = bfs(&SearchState::from_starting_node(Node::Entrance), successors, success);
    println!("visit all keys: {:?}", result);
//    result.unwrap().1
    0
}


pub fn part1() -> usize {
    shortest_path("day18_input.txt")
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
