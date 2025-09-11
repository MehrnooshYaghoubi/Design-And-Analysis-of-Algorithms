use crate::types::Point;
use std::f64;

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub fn closest_pair(points: &[Point]) -> Option<(Point, Point, f64)> {
    if points.len() < 2 {
        return None;
    }

    let mut points_sorted_x = points.to_vec();
    points_sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    Some(closest_pair_recursive(&points_sorted_x))
}

fn closest_pair_recursive(points: &[Point]) -> (Point, Point, f64) {
    let n = points.len();
    if n <= 3 {
        return brute_force_closest_pair(points);
    }

    let mid = n / 2;
    let mid_point = points[mid];

    let (p1_l, p2_l, d_l) = closest_pair_recursive(&points[..mid]);
    let (p1_r, p2_r, d_r) = closest_pair_recursive(&points[mid..]);

    let (mut min_pair, mut min_dist) = if d_l < d_r {
        ((p1_l, p2_l), d_l)
    } else {
        ((p1_r, p2_r), d_r)
    };

    let strip: Vec<Point> = points
        .iter()
        .filter(|p| (p.x - mid_point.x).abs() < min_dist)
        .cloned()
        .collect();

    let (strip_pair, strip_dist) = closest_pair_in_strip(&strip, min_dist);
    if strip_dist < min_dist {
        min_pair = strip_pair;
        min_dist = strip_dist;
    }

    (min_pair.0, min_pair.1, min_dist)
}

fn brute_force_closest_pair(points: &[Point]) -> (Point, Point, f64) {
    let mut min_dist = f64::INFINITY;
    let mut pair = (points[0], points[1]);

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = points[i].distance(&points[j]);
            if dist < min_dist {
                min_dist = dist;
                pair = (points[i], points[j]);
            }
        }
    }

    (pair.0, pair.1, min_dist)
}

fn closest_pair_in_strip(strip: &[Point], min_dist: f64) -> ((Point, Point), f64) {
    let mut sorted_strip = strip.to_vec();
    sorted_strip.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let mut min_dist = min_dist;
    let mut min_pair = (sorted_strip[0], sorted_strip[1]);

    for i in 0..sorted_strip.len() {
        for j in i + 1..sorted_strip.len() {
            if (sorted_strip[j].y - sorted_strip[i].y) >= min_dist {
                break;
            }

            let dist = sorted_strip[i].distance(&sorted_strip[j]);
            if dist < min_dist {
                min_dist = dist;
                min_pair = (sorted_strip[i], sorted_strip[j]);
            }
        }
    }

    (min_pair, min_dist)
}
