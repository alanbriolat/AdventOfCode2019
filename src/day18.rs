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
    doors: HashSet<char>,
}

impl Edge {
    fn new() -> Edge {
        Edge {
            cost: 0,
            doors: HashSet::new(),
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

#[derive(Clone,Debug)]
struct PathDB(HashMap<Node, Vec<(Node, Edge)>>);
deref!(PathDB, HashMap<Node, Vec<(Node, Edge)>>);

impl PathDB {
    fn new() -> PathDB {
        PathDB(HashMap::new())
    }

    fn add_edge(&mut self, a: Node, b: Node, e: Edge) {
        self.0.entry(a).or_insert(Vec::new()).push((b, e.clone()));
        self.0.entry(b).or_insert(Vec::new()).push((a, e.clone()));
    }

    /// Simplify the graph by removing `n`, combining paths and removing doors if `n` is a `Key`
    fn remove_node(&mut self, n: Node) {
        // TODO: implement me
    }
}

impl From<&Map> for PathDB {
    fn from(map: &Map) -> Self {
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
                        queue.push_back((next, Edge{cost: edge.cost + 1, doors: edge.doors.clone()}, pos.clone(), from_node));
                    },
                    // Door: add to the set of doors, advance one step
                    Some(door) if 'A' <= door && door <= 'Z' => {
                        let mut doors = edge.doors.clone();
                        doors.insert(door);
                        queue.push_back((next, Edge{cost: edge.cost + 1, doors}, pos.clone(), from_node));
                    },
                    // Key: end path and record it, start new path
                    Some(key) if 'a' <= key && key <= 'z' => {
                        let node = Node::Key(key);
                        paths.add_edge(from_node, node, Edge{cost: edge.cost + 1, doors: edge.doors.clone()});
                        queue.push_back((next, Edge::new(), pos.clone(), node));
                    },
                    unknown => panic!(format!("unknown tile: {:?}", unknown)),
                }
            }
        }
        return paths;
    }
}

fn shortest_path(filename: &str) -> usize {
    let map = Map::from_data_file(filename);
    let paths = PathDB::from(&map);
    println!("paths: {:?}", paths);
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
        assert_eq!(shortest_path("day18_example1.txt"), 8);
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
