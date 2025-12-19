use aoc_helpers::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Binary;
use std::time::Instant;

// This one nearly broke me. I learned a lot about ray tracing algorithms and optimization. This is probably way overengineered,
// and I'm sure there's a more efficient way to do this (it takes ~11s to run), but I've spent 3 days struggling with this so I'm
// just happy to be here.

// Essentially this puzzle gives a polygon formed by connecting each of the points in the input. And the goal is to go through
// each rectangle created by connecting any two input points together, starting with the largest rectangle and moving down the
// list by descending area, and checks to see if the rectangle is entirely within the polygon.

// My approach, after many rewrites, was to build the polygon, use a modified ray tracing algorithm to find all points internal
// to the polygon (including the border, which was its own can of worms), given as lists of horizontal spans for each row. I also
// get each possible Rect of points as instances of a struct containing x span (x1 -> x2), y span (y1 -> y2), and area for the
// resulting rectangle. Once we've built out the polygon, we check each rectangle, by descending area, and check to see if the x
// spans for the given rectangle intersect with any of the corresponding spans for the polygon for every row in the y span. If so,
// return the area of that rectangle.

fn main() -> Result<(), Box<dyn Error>> {
    let (spans, mut rects) = parse_input()?;

    let now = Instant::now();
    let result = find_best_rect(&spans, &mut rects);
    println!("Result: {}\nElapsed time: {:?}", result, now.elapsed());

    Ok(())
}

fn parse_input() -> Result<(HashMap<usize, Vec<(usize, usize)>>, BinaryHeap<Rect>), Box<dyn Error>>
{
    let input: Vec<String> = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 9)?
    };

    // Parse the points from an input vector of strings to a vector of u32s
    let points: Vec<(usize, usize)> = input
        .iter()
        .map(|s| {
            let spl: Vec<_> = s.split(',').collect();
            let (x, y) = (
                spl[0].parse::<usize>().unwrap(),
                spl[1].parse::<usize>().unwrap(),
            );

            (x, y)
        })
        .collect();

    // Parse all possible rectangles into a binary heap so we can do stuff to them later
    let mut rectangles: BinaryHeap<Rect> = BinaryHeap::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let (x_min, x_max) = (x1.min(x2), x1.max(x2));
            let (y_min, y_max) = (y1.min(y2), y1.max(y2));

            let width = x_max - x_min + 1;
            let height = y_max - y_min + 1;

            let area = width as u64 * height as u64;

            let x_span = (x_min, x_max);
            let y_span = (y_min, y_max);

            rectangles.push(Rect {
                area,
                x_span,
                y_span,
            })
        }
    }

    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut border: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    // Connect each point with the point that follows it
    for window in points.windows(2) {
        let (first, second) = (window[0], window[1]);
        connect_points(first, second, &mut edges, &mut border);
    }

    // Connect the last point with the first point to close the loop
    let last = points.iter().last().unwrap().to_owned();
    let first = points[0];

    connect_points(last, first, &mut edges, &mut border);

    // Combine the polygon with the border to give a single hashmap of spans that define the polygon
    let spans = combine_polygon_with_border(&mut edges, &border);

    Ok((spans, rectangles))
}

// Goes through each Rect of points (i.e., each rectangle) from largest area to smallest, checking to see if the rectangle is
// contained within the polygon. If it is, return its area, otherwise check the next one.
fn find_best_rect(
    polygon_spans: &HashMap<usize, Vec<(usize, usize)>>,
    rects: &mut BinaryHeap<Rect>,
) -> u64 {
    // Get each Rect from the binary heap
    'outer: while let Some(rect) = rects.pop() {
        let (y1, y2) = rect.y_span;

        // Check each row in the rectangle to see if its x span intersects the corresponding polygon x spans
        for y in y1..=y2 {
            let rect_span = rect.x_span;

            if let Some(p_spans) = polygon_spans.get(&y) {
                for p_span in p_spans {
                    // If the rectangle doesn't intersect the polygon for this row, dump this Rect and move to the next
                    if rect_span.0 < p_span.0 || rect_span.1 > p_span.1 {
                        continue 'outer;
                    }
                }
            }
        }
        return rect.area;
    }

    0
}

// Takes the edges hashmap which contains spans for all vertical edges for each row, and the border hashmap which contains spans for all horizontal borders
// the given row, and combines them into a single hashmap of spans defining the polygon. The key is the y position, and the value is a vector of x spans.
fn combine_polygon_with_border(
    edges: &mut HashMap<usize, Vec<usize>>,
    border: &HashMap<usize, Vec<(usize, usize)>>,
) -> HashMap<usize, Vec<(usize, usize)>> {
    let mut merged: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    let max_y = edges
        .keys()
        .max()
        .unwrap()
        .max(border.keys().max().unwrap())
        .to_owned();
    for y in 0..=max_y {
        let mut current_spans: Vec<(usize, usize)> = Vec::new();

        let current_border = border.get(&y);
        if let Some(current_edges) = edges.get_mut(&y) {
            current_spans = get_spans(current_edges, current_border);
        } else if let Some(b) = current_border {
            current_spans = b.to_owned();
        }

        merged.insert(y, current_spans.clone());
    }

    merged
}

// Merges two vectors of spans together, combining any overlapping spans, and returns a single vector of spans
fn merge_spans(
    spans1: &mut Vec<(usize, usize)>,
    spans2: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    spans1.extend(spans2);
    spans1.sort_unstable_by_key(|(a, _)| *a);

    let mut merged: Vec<(usize, usize)> = Vec::new();

    for span in spans1 {
        if let Some(last_merged) = merged.last_mut() {
            if span.0 <= last_merged.1 {
                if span.1 > last_merged.1 {
                    last_merged.1 = span.1;
                }
            } else {
                merged.push(*span);
            }
        } else {
            merged.push(*span);
        }
    }

    merged
}

// Combines a single row's vertical edges and horizontal border spans to make a single vector of spans
// which define the polygon for that row
fn get_spans(edges: &mut Vec<usize>, border: Option<&Vec<(usize, usize)>>) -> Vec<(usize, usize)> {
    let mut edge_spans: Vec<(usize, usize)> = Vec::new();

    // The edges start as a vector of x positions where entries 0 and 1, 2 and 3, 4 and 5, etc, each make up a
    // span. We're going to convert them into a vector of spans, so first we need to make sure they're sorted.
    edges.sort_unstable();

    // Take each subsequent pair of edges and convert them into a span. This fails if there's an odd number of
    // edges, but since there should never be an odd number of edges in a polygon, this should be fine. And it
    // compiles, and gives the right answer, so I think we're good.
    for rect in edges.chunks_exact(2) {
        edge_spans.push((rect[0], rect[1]));
    }

    // If there's a border on this row, merge it with the interior of the polygon as defined by the edge span
    if let Some(b) = border {
        return merge_spans(&mut edge_spans, b);

    // If there's no border on this row, we can skip all this nonsense and just return the sorted edge spans
    } else {
        edge_spans.sort_unstable_by_key(|(a, _)| *a);
        return edge_spans;
    }
}

// Take two points and connect them together. If the points are separated vertically, they get put into the edges
// hashmap that defines the polygon's interior for the ray tracing algorithm. If they're separated horizontally,
// the x span gets put into the border hashmap, which will be combined with the interior later to make a single span.
// Honestly there has to be a better way to do this, but we're here now.
fn connect_points(
    p1: (usize, usize),
    p2: (usize, usize),
    edges: &mut HashMap<usize, Vec<usize>>,
    border: &mut HashMap<usize, Vec<(usize, usize)>>,
) {
    let (x_min, x_max) = (p1.0.min(p2.0), p1.0.max(p2.0));
    let (y_min, y_max) = (p1.1.min(p2.1), p1.1.max(p2.1));

    // If the points are separated vertically
    if y_min != y_max {
        for y in y_min..y_max {
            edges.entry(y).or_insert_with(Vec::new).push(x_min);
        }

    // If they're separated horizontally
    } else {
        border
            .entry(y_min)
            .or_insert_with(Vec::new)
            .push((x_min, x_max));
    }
}

#[derive(Debug)]
struct Rect {
    area: u64,
    x_span: (usize, usize),
    y_span: (usize, usize),
}

impl Eq for Rect {}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area.eq(&self.area)
    }
}

impl Ord for Rect {
    fn cmp(&self, other: &Self) -> Ordering {
        self.area.partial_cmp(&other.area).unwrap()
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
