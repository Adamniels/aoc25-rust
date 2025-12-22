use crate::utils;

pub fn solve(input: &str) {
    println!("=== Day 1 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mut parsed_input = Vec::new();

    for line in utils::parse_lines(input) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (x_str, y_str) = line.split_once(',').expect("Each line must be 'x,y'");

        let x: i64 = x_str.trim().parse().expect("x must be an integer");
        let y: i64 = y_str.trim().parse().expect("y must be an integer");

        parsed_input.push((x, y));
    }

    parsed_input
}

fn part1(input: &str) -> i64 {
    let pts = parse_input(input);

    let mut best: i64 = 0;

    for i in 0..pts.len() {
        let (x1, y1) = pts[i];
        for j in (i + 1)..pts.len() {
            let (x2, y2) = pts[j];

            let dx = (x2 - x1).abs();
            let dy = (y2 - y1).abs();
            let area = (dx + 1) * (dy + 1);

            best = best.max(area);
        }
    }

    best
}

fn part2(input: &str) -> i64 {
    let pts = parse_input(input);
    let n = pts.len();

    // Polygon edges are consecutive pairs (wrapping)
    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::with_capacity(n);
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        edges.push((a, b));
    }

    let mut best: i64 = 0;

    for i in 0..n {
        let (x1, y1) = pts[i];
        for j in (i + 1)..n {
            let (x2, y2) = pts[j];

            if x1 == x2 || y1 == y2 {
                continue; // no rectangle
            }

            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);

            let area = (xmax - xmin + 1) * (ymax - ymin + 1);
            if area <= best {
                continue;
            }

            // Other two corners of the rectangle
            let c3 = (x1, y2);
            let c4 = (x2, y1);

            // Must be inside or on boundary (red/green area)
            if !point_in_poly_inclusive(c3, &edges) {
                continue;
            }
            if !point_in_poly_inclusive(c4, &edges) {
                continue;
            }

            // Boundary must not pass through the strict interior of the rectangle
            if boundary_crosses_rectangle_interior(xmin, xmax, ymin, ymax, &edges) {
                continue;
            }

            best = area;
        }
    }

    best
}

fn on_segment_inclusive(p: (i64, i64), a: (i64, i64), b: (i64, i64)) -> bool {
    let (px, py) = p;
    let (x1, y1) = a;
    let (x2, y2) = b;

    if x1 == x2 {
        if px != x1 {
            return false;
        }
        let lo = y1.min(y2);
        let hi = y1.max(y2);
        py >= lo && py <= hi
    } else if y1 == y2 {
        if py != y1 {
            return false;
        }
        let lo = x1.min(x2);
        let hi = x1.max(x2);
        px >= lo && px <= hi
    } else {
        false
    }
}

// kollar så alla hörn ligger inne i polygonen
fn point_in_poly_inclusive(p: (i64, i64), edges: &[((i64, i64), (i64, i64))]) -> bool {
    for &(a, b) in edges {
        if on_segment_inclusive(p, a, b) {
            return true;
        }
    }

    let (px, py) = p;
    let mut inside = false;

    for &(a, b) in edges {
        let (x1, y1) = a;
        let (x2, y2) = b;

        if x1 == x2 {
            let x = x1;
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);

            if py >= ymin && py < ymax {
                if x > px {
                    inside = !inside;
                }
            }
        } else {
            let _ = (x2, y2);
        }
    }

    inside
}

// även om hörnen ligger inne så kan den sticka ut men då måste det finnas en edge som går igenom
// rektangeln
fn boundary_crosses_rectangle_interior(
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    edges: &[((i64, i64), (i64, i64))],
) -> bool {
    if xmax - xmin < 2 || ymax - ymin < 2 {
        return false;
    }

    let ix_lo = xmin + 1;
    let ix_hi = xmax - 1;
    let iy_lo = ymin + 1;
    let iy_hi = ymax - 1;

    for &(a, b) in edges {
        let (x1, y1) = a;
        let (x2, y2) = b;

        if x1 == x2 {
            let x = x1;
            if x < ix_lo || x > ix_hi {
                continue;
            }
            let lo = y1.min(y2);
            let hi = y1.max(y2);

            let o_lo = lo.max(iy_lo);
            let o_hi = hi.min(iy_hi);
            if o_lo <= o_hi {
                return true;
            }
        } else if y1 == y2 {
            let y = y1;
            if y < iy_lo || y > iy_hi {
                continue;
            }
            let lo = x1.min(x2);
            let hi = x1.max(x2);

            let o_lo = lo.max(ix_lo);
            let o_hi = hi.min(ix_hi);
            if o_lo <= o_hi {
                return true;
            }
        } else {
            panic!("should not be possible")
        }
    }

    false
}
