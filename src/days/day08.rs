use crate::utils;

pub fn solve(input: &str) {
    println!("=== Day 1 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let junc_box_list = parse_input(input);

    let mut edges = build_edges(&junc_box_list);
    edges.sort_unstable_by_key(|e| e.distance);

    // skapa DSU med antal junction boxes
    let mut dsu = Dsu::new(junc_box_list.len());

    // NOTE: ta de 10 eller 1000 kortaste edges och connecta beroende på example eller riktig
    for e in edges.iter().take(1000) {
        dsu.union(e.a, e.b);
    }

    // räkna circuit-storlekar
    use std::collections::HashMap;
    let mut sizes: HashMap<usize, usize> = HashMap::new();

    for i in 0..junc_box_list.len() {
        let root = dsu.find(i);
        *sizes.entry(root).or_insert(0) += 1;
    }

    // ta tre största
    let mut vals: Vec<usize> = sizes.values().copied().collect();
    vals.sort_unstable_by(|a, b| b.cmp(a));

    (vals[0] as i64) * (vals[1] as i64) * (vals[2] as i64)
}

fn part2(input: &str) -> i64 {
    let junc_box_list = parse_input(input);

    let mut edges = build_edges(&junc_box_list);
    edges.sort_unstable_by_key(|e| e.distance);

    // skapa DSU med antal junction boxes
    let mut dsu = Dsu::new(junc_box_list.len());

    for e in edges.iter() {
        // om union faktiskt händer är denna edge "needed"
        if dsu.union(e.a, e.b) {
            // när allt är i en circuit är detta sista kopplingen
            if dsu.components == 1 {
                let xa = junc_box_list[e.a].x as i64;
                let xb = junc_box_list[e.b].x as i64;
                return xa * xb;
            }
        }
    }

    unreachable!()
}

fn parse_input(input: &str) -> Vec<Junction_box> {
    let mut junc_box_list = vec![];
    let parsed_input = utils::parse_lines(input);
    for line in parsed_input {
        junc_box_list.push(Junction_box::from_string(line));
    }
    junc_box_list
}

fn build_edges(points: &[Junction_box]) -> Vec<Edge> {
    let mut edges = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = points[i].distance_sq(&points[j]);
            edges.push(Edge {
                a: i,
                b: j,
                distance: d,
            });
        }
    }

    edges
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }

        // union by size
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.components -= 1;
        true
    }
}

#[derive(Debug)]
struct Junction_box {
    x: i32,
    y: i32,
    z: i32,
}

impl Junction_box {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Junction_box { x, y, z }
    }
    fn from_string(line: &str) -> Self {
        let nums: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();

        let (x, y, z) = (nums[0], nums[1], nums[2]);
        Junction_box { x, y, z }
    }

    // behöver inte sqrt för kan ändå jämföra avstånd
    fn distance_sq(&self, other: &Junction_box) -> i64 {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        let dz = self.z as i64 - other.z as i64;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    a: usize,      // index i listan
    b: usize,      // index i listan
    distance: i64, // distance_sq
}
