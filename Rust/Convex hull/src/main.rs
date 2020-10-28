use rand::Rng;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::process::exit;
use std::time::Duration;
use rand::seq::SliceRandom;
use clap::{App, Arg};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point {
            x,
            y,
        }
    }

    pub fn print(self) {
        println!("{} {}", self.x, self.y);
    }

    pub fn get_point(self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}

#[derive(Debug, Copy, Clone)]
enum Algorithm {
    Incremental,
    Gift,
    Chan,
    MBC,
}

#[derive(Copy, Clone)]
pub struct TestData {
    right_turn: i64,
    left_turn: i64,
    no_turn: i64,
    time_start: std::time::Duration,
    time_end: std::time::Duration,
}

impl TestData {
    fn new() -> Self {
        TestData {
            right_turn: 0,
            left_turn: 0,
            no_turn: 0,
            time_start: Default::default(),
            time_end: Default::default(),
        }
    }
    fn increment_turn(&mut self, direction: i64) {
        if direction == 0 { self.increment_no_turn() } else if direction > 0 { self.increment_right_turn() } else { self.increment_left_turn() }
    }
    fn increment_right_turn(&mut self) { self.right_turn += 1 }
    fn increment_left_turn(&mut self) { self.left_turn += 1 }
    fn increment_no_turn(&mut self) { self.no_turn += 1 }
    fn set_start_time(&mut self) { self.time_start = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Time went backwards") }
    fn set_end_time(&mut self) { self.time_end = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Time went backwards") }
    fn time_elapsed(self) -> Duration { self.time_end - self.time_start }
    fn print_data(&mut self, algorithm: Algorithm, input_size: usize, output_size: usize) {
        println!("------- Method: {:?} statistics -------", algorithm);
        println!("Input length: {}, Output length: {}", input_size, output_size);
        println!("Ran in {} nanoseconds", self.time_elapsed().as_nanos());
        println!("Right turns {}", self.right_turn);
        println!("Left turns {}", self.left_turn);
        println!("No turns {}", self.no_turn);
        println!("Total turns {}", self.right_turn + self.left_turn + self.no_turn);
        println!("-------------------------------------");
        println!();
    }
    fn print_data_csv(&mut self, algorithm: Algorithm, input_size: usize, output_size: usize) {
        println!("{:?},{},{},{},{},{},{},{}",
                 algorithm,
                 input_size, output_size,
                 self.time_elapsed().as_nanos(),
                 self.right_turn, self.left_turn, self.no_turn,
                 self.right_turn + self.left_turn + self.no_turn
        );
    }
}

struct MBCTestData {
    time_start: std::time::SystemTime,
    time_end: std::time::SystemTime,
    bridges: u64,
    recursions: u64,
}

impl MBCTestData {
    fn new() -> Self {
        Self {
            time_start: std::time::SystemTime::now(),
            time_end: std::time::SystemTime::now(),
            bridges: 0,
            recursions: 0,
        }
    }
    fn time_elapsed(&self) -> Duration { self.time_end.duration_since(self.time_start).unwrap() }
    fn print_data(&mut self, input_size: usize, output_size: usize) {
        println!("------- Method: {:?} statistics -------", Algorithm::MBC);
        println!("Input length: {}, Output length: {}", input_size, output_size);
        println!("Ran in {} nanoseconds", self.time_elapsed().as_nanos());
        println!("Executed {} recursions", self.recursions);
        println!("Executed bridges {} times", self.bridges);
        println!("-------------------------------------");
        println!();
    }
    fn print_data_csv(&mut self, input_size: usize, output_size: usize) {
        println!("{:?},{},{},{},{},{}",
                 Algorithm::MBC,
                 input_size, output_size,
                 self.time_elapsed().as_nanos(),
                 self.recursions, self.bridges
        );
    }
}

#[derive(Debug, Copy, Clone)]
enum PointGeneratorStrategy {
    Square,
    Circle,
    Exp,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Convex Hull Algorithms")
        .about("Tests different convex hull algorithms")
        .arg(Arg::with_name("points")
            .short("n")
            .long("points")
            .help("Amount of points to generate")
            .default_value("1000"))
        .arg(Arg::with_name("samples")
            .short("s")
            .long("samples")
            .help("Amount of times to run algorithms on dataset")
            .default_value("1"))
        .arg(Arg::with_name("generator")
            .short("g")
            .long("generator")
            .help("The point generator to use.")
            .possible_values(&vec!["square", "circle", "exp"])
            .default_value("square"))
        .arg(Arg::with_name("all")
            .short("a")
            .long("all")
            .help("Runs all algorithms."))
        .arg(Arg::with_name("csv")
            .long("csv")
            .help("Print CSV instead of human-friendly."))
        .arg(Arg::with_name("incremental")
            .long("incremental")
            .help("Runs the incremental algorithm."))
        .arg(Arg::with_name("gift")
            .long("gift")
            .help("Runs the gift wrapping algorithm."))
        .arg(Arg::with_name("chan")
            .long("chan")
            .help("Runs Chan's algorithm."))
        .arg(Arg::with_name("mbc")
            .long("mbc")
            .help("Runs the marriage-before-conquest algorithm."))
        .get_matches();
    let point_count = matches.value_of("points").unwrap().parse::<i64>().unwrap();
    let sample_count = matches.value_of("samples").unwrap().parse::<usize>().unwrap();
    let point_generator_strategy = match matches.value_of("generator").unwrap() {
        "square" => PointGeneratorStrategy::Square,
        "circle" => PointGeneratorStrategy::Circle,
        "exp" => PointGeneratorStrategy::Exp,
        g => panic!("{} is not a known generator.", g),
    };
    let points = generate_points(point_count, point_generator_strategy);
    let csv = matches.is_present("csv");
    if !csv {
        println!("Point count: {:?}", point_count);
        println!("Sample count: {:?}", sample_count);
        println!("Point generator: {:?}", point_generator_strategy);
        println!("Deduped point count: {:?}", points.len());
    }
    let run_all = matches.is_present("all");

    if run_all || matches.is_present("incremental") {
        run_algorithm(&points, Algorithm::Incremental, sample_count, csv);
    }

    if run_all || matches.is_present("gift") {
        run_algorithm(&points, Algorithm::Gift, sample_count, csv);
    }

    if run_all || matches.is_present("chan") {
        run_algorithm(&points, Algorithm::Chan, sample_count, csv);
    }

    if run_all || matches.is_present("mbc") {
        run_algorithm(&points, Algorithm::MBC, sample_count, csv);
    }

    Ok(())
}

fn run_algorithm(points: &Vec<Point>, algorithm: Algorithm, sample_count: usize, csv: bool) {
    if !csv {
        println!("------- Running {:?} {} times -------", algorithm, sample_count);
    }

    for _ in 0..sample_count {
        match algorithm {
            Algorithm::Incremental => {
                let mut inc_ch_test_data = TestData::new();
                let inc_ch_output_ch = inc_ch(&points, &mut inc_ch_test_data);
                if csv {
                    inc_ch_test_data.print_data_csv(algorithm, points.len(), inc_ch_output_ch.len());
                } else {
                    inc_ch_test_data.print_data(algorithm, points.len(), inc_ch_output_ch.len());
                }
            }
            Algorithm::Gift => {
                let mut gift_ch_test_data = TestData::new();
                let hull = gift_ch(&points, &mut gift_ch_test_data);
                if csv {
                    gift_ch_test_data.print_data_csv(algorithm, points.len(), hull.len());
                } else {
                    gift_ch_test_data.print_data(algorithm, points.len(), hull.len());
                }
            }
            Algorithm::Chan => {
                let mut ch_ch_test_data = TestData::new();
                let test = ch_ch(&points, &mut ch_ch_test_data);
                if csv {
                    ch_ch_test_data.print_data_csv(algorithm, points.len(), test.len());
                } else {
                    ch_ch_test_data.print_data(algorithm, points.len(), test.len());
                }
            }
            Algorithm::MBC => {
                let mut mbc_ch_test_data = MBCTestData::new();
                let mbc_ch = mbc_ch(points.clone(), &mut mbc_ch_test_data);
                if csv {
                    mbc_ch_test_data.print_data_csv(points.len(), mbc_ch.len());
                } else {
                    mbc_ch_test_data.print_data(points.len(), mbc_ch.len());
                }
            }
        };
    }
}

fn generate_circle_point() -> Point {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0f64, 360f64);
    let r = 4000f64 * f64::sqrt(rng.gen_range(0f64, 1f64));
    let xx = (r * f64::cos(a)) as i64 + 3000;
    let y = (r * f64::sin(a)) as i64 + 3000;

    Point::new(xx, y)
}

fn generate_square_point() -> Point {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0, 3600);
    let y = rng.gen_range(0, 3600);

    Point::new(x, y)
}

fn generate_exp_point(x: i64) -> Point {
    Point::new(x, x.pow(2))
}

fn generate_points(count: i64, strategy: PointGeneratorStrategy) -> Vec<Point> {
    let points: HashSet<_> = (0..count)
        .map(|i| (match strategy {
            PointGeneratorStrategy::Circle => generate_circle_point(),
            PointGeneratorStrategy::Square => generate_square_point(),
            PointGeneratorStrategy::Exp => generate_exp_point(i),
        }))
        .collect();

    let points: Vec<_> = points.into_iter().collect();
    points
}

/// These methods calculate at which side of the line point c is
/// The line is created between a and b.
/// There are 3 different outcomes
/// < 0  -> left turn
/// == 0 -> on the line
/// > 0  -> right turn
fn dir(a: Point, b: Point, c: Point) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn inc_ch(points: &Vec<Point>, test_struct: &mut TestData) -> Vec<Point> {
    test_struct.set_start_time();
    if points.len() <= 2 {
        let set: HashSet<Point> = points.clone().drain(..).collect();
        test_struct.set_end_time();
        return Vec::from_iter(set.iter().map(|p| p.clone()));
    }

    let mut cloned_points = points.clone();
    cloned_points.sort();
    let mut asc_sorted = cloned_points.clone();
    cloned_points.reverse();
    let mut desc_sorted = cloned_points;
    let mut lh: Vec<Point> = asc_sorted.drain(0..2).collect();
    let mut uh: Vec<Point> = desc_sorted.drain(0..2).collect();

    generate_upper_hull(test_struct, &mut asc_sorted, &mut lh);
    generate_upper_hull(test_struct, &mut desc_sorted, &mut uh);

    uh.pop();
    uh.drain(0..1);
    lh.append(&mut uh);
    test_struct.set_end_time();
    lh
}

fn generate_upper_hull(test_struct: &mut TestData, desc_sorted: &mut Vec<Point>, uh: &mut Vec<Point>) {
    for cur_point in desc_sorted {
        loop {
            if uh.len() < 2 {
                break;
            }
            let direction = dir(uh[uh.len() - 2], uh[uh.len() - 1], *cur_point);
            if direction < 0 {
                break;
            }
            test_struct.increment_turn(direction);
            uh.pop();
        }

        uh.push(*cur_point);
        let d = dir(uh[uh.len() - 2], uh[uh.len() - 1], *cur_point);
        test_struct.increment_turn(d);
    }
}

fn gift_ch(points: &Vec<Point>, test_struct: &mut TestData) -> Vec<Point> {
    test_struct.set_start_time();
    let mut hull = Vec::<Point>::new();
    let mut hull_point: Point = match points.iter().min() {
        Some(p) => *p,
        None => return Vec::new(),
    };
    loop {
        hull.push(hull_point);
        let mut current_candidate: Point = match points.get(0) {
            Some(p) => *p,
            None => exit(8),
        };

        for j in points.clone() {
            let direction = dir(hull[hull.len() - 1], current_candidate, j);
            test_struct.increment_turn(direction);
            if current_candidate == hull_point || direction > 0 {
                current_candidate = j;
            }
        }
        hull_point = current_candidate;
        if current_candidate == hull[0] {
            break;
        }
    }
    test_struct.set_end_time();
    hull
}

fn ch_ch(points: &Vec<Point>, test_struct: &mut TestData) -> Vec<Point> {
    test_struct.set_start_time();
    for i in 1..(log_2(log_2(points.len() as i32) as i32) + 2) {
        let (hull, success) = uh_with_size(points, i64::pow(2, i64::pow(2, i) as u32), test_struct);
        if success {
            test_struct.set_end_time();
            return hull;
        }
    }
    test_struct.set_end_time();
    Vec::<Point>::new()
}

fn uh_with_size(points: &Vec<Point>, h: i64, test_struct: &mut TestData) -> (Vec<Point>, bool) {
    let mut cloned_points = points.clone();
    let mut tmp_hull = Vec::<Point>::new();
    let mut upper_hull = Vec::<Point>::new();
    let mut lower_hull = Vec::<Point>::new();

    let mut m = points.len() / (h as usize);
    if m == 0 {
        m = 1
    }
    loop {
        let min = usize::min(cloned_points.len(), m);
        if min == 0 {
            break;
        }
        let mut ps: Vec<Point> = cloned_points.drain(0..min).collect();
        tmp_hull.append(&mut gift_ch(&mut ps, test_struct))
    }


    let min_point = match points.iter().min() {
        Some(point) => *point,
        None => exit(7),
    };
    let max_point = match points.iter().max() {
        Some(point) => *point,
        None => exit(7),
    };
    let mut upper_point: Point = min_point.clone();
    let mut lower_point: Point = max_point.clone();

    for _ in 0..h {
        if upper_hull.len() < 2 || upper_hull[upper_hull.len() - 1] != max_point {
            upper_hull.push(upper_point);
        }
        if lower_hull.len() < 2 || lower_hull[lower_hull.len() - 1] != min_point {
            lower_hull.push(lower_point);
        }

        if (upper_hull.len() > 2 || lower_hull.len() > 2)
            && upper_hull[upper_hull.len() - 1] == max_point
            && lower_hull[lower_hull.len() - 1] == min_point {
            break;
        }

        let mut current_upper_candidate = match tmp_hull.get_mut(0) {
            Some(point) => *point,
            None => {
                exit(5)
            }
        };

        let mut current_lower_candidate = current_upper_candidate.clone();

        for point in tmp_hull.clone() {
            let dir1 = dir(upper_hull[upper_hull.len() - 1], current_upper_candidate, point);
            test_struct.increment_turn(dir1);
            if current_upper_candidate == upper_point || dir1 > 0 {
                current_upper_candidate = point;
            }
            let dir2 = dir(lower_hull[lower_hull.len() - 1], current_lower_candidate, point);
            test_struct.increment_turn(dir2);
            if current_lower_candidate == lower_point || dir2 > 0 {
                current_lower_candidate = point;
            }
        }

        lower_point = current_lower_candidate;
        upper_point = current_upper_candidate;
    }

    let succ = upper_hull[upper_hull.len() - 1] == max_point
        && lower_hull[lower_hull.len() - 1] == min_point;
    lower_hull.pop();
    lower_hull.drain(0..1);
    upper_hull.append(&mut lower_hull);
    (upper_hull, succ)
}

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: i32) -> u32 {
    assert!(x > 0);
    num_bits::<i32>() as u32 - x.leading_zeros() - 1
}

fn bridge(points: &[Point], a: i64, test_data: &mut MBCTestData) -> (Point, Point) {
    test_data.bridges += 1;
    if points.len() == 2 {
        return if points[0].x < points[1].x {
            (points[0], points[1])
        } else {
            (points[1], points[0])
        };
    }
    let mut candidates = Vec::with_capacity(points.len());
    let chunks = points.chunks_exact(2);
    chunks.remainder().iter().for_each(|&p| candidates.push(p));
    let pairs: Vec<_> = chunks
        .map(|p| match p {
            [p1, p2, ..] => if p1.x < p2.x { (*p1, *p2) } else { (*p2, *p1) },
            _ => panic!("Chunks forgot how to chunk"),
        })
        .filter(|(p1, p2)|
            if p1.x == p2.x {
                if p1.y > p2.y {
                    candidates.push(p1.clone());
                } else {
                    candidates.push(p2.clone());
                }
                false
            } else { true }
        )
        .map(|(p1, p2)| (p1, p2, (p1.y as f64 - p2.y as f64) / (p1.x as f64 - p2.x as f64)))
        .collect();

    if pairs.is_empty() {
        return bridge(&candidates, a, test_data);
    }

    let slopes: Vec<_> = pairs.iter().map(|(_, _, s)| s).cloned().collect();
    let slope_median = find_sampled_median(&slopes, 5);

    let small: Vec<_> = pairs
        .iter()
        .filter(|(_, _, s)| *s < slope_median)
        .map(|(p1, p2, _)| (*p1, *p2))
        .collect();
    let equal: Vec<_> = pairs
        .iter()
        .filter(|(_, _, s)| *s == slope_median)
        .map(|(p1, p2, _)| (*p1, *p2))
        .collect();
    let large: Vec<_> = pairs
        .iter()
        .filter(|(_, _, s)| *s > slope_median)
        .map(|(p1, p2, _)| (*p1, *p2))
        .collect();

    let funny_slopes: Vec<_> = points
        .iter()
        .map(|p| p.y as f64 - slope_median * p.x as f64)
        .collect();
    let max_slope = funny_slopes
        .iter()
        .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
        .cloned()
        .unwrap();

    let max = {
        let mut max = Vec::new();
        for i in 0..points.len() {
            if (funny_slopes[i] - max_slope).abs() < 10e-10 {
                max.push(points[i]);
            }
        }
        max
    };

    let pk = max
        .iter()
        .min_by_key(|p| p.x)
        .cloned()
        .unwrap();
    let pm = max
        .iter()
        .max_by_key(|p| p.x)
        .cloned()
        .unwrap();
    if pk.x <= a && pm.x > a {
        return (pk, pm);
    }

    if pm.x <= a {
        large.iter().cloned().for_each(|(_, p)| candidates.push(p));
        equal.iter().cloned().for_each(|(_, p)| candidates.push(p));
        small.iter().cloned().for_each(|(p1, p2)| {
            candidates.push(p1);
            candidates.push(p2);
        });
    }

    if pk.x > a {
        equal.iter().cloned().for_each(|(p, _)| candidates.push(p));
        small.iter().cloned().for_each(|(p, _)| candidates.push(p));
        large.iter().cloned().for_each(|(p1, p2)| {
            candidates.push(p1);
            candidates.push(p2);
        });
    }

    let candidates: Vec<_> = candidates.into_iter().collect();
    bridge(&candidates, a, test_data)
}

fn find_sampled_median_x(points: &[Point], sample_size: usize) -> i64 {
    let mut sample: Vec<i64> = points
        .choose_multiple(&mut rand::thread_rng(), sample_size)
        .map(|p| p.x)
        .collect();
    sample.sort();
    sample[sample.len() / 2]
}

fn find_sampled_median<T: PartialOrd + Copy>(points: &[T], sample_size: usize) -> T {
    let mut sample: Vec<T> = points
        .choose_multiple(&mut rand::thread_rng(), sample_size)
        .cloned()
        .collect();
    sample.sort_by(|f1, f2| f1.partial_cmp(f2).unwrap());
    sample[sample.len() / 2]
}

fn mbc_ch(points: Vec<Point>, test_data: &mut MBCTestData) -> Vec<Point> {
    test_data.time_start = std::time::SystemTime::now();
    let lh_points: Vec<_> = points
        .iter()
        .map(|p| Point { x: p.x, y: -p.y })
        .collect();
    let lh = mbc_ch_inner(lh_points, test_data);
    let mut real_lh: Vec<_> = lh
        .iter()
        .skip(1)
        .take(lh.len() - 2)
        .rev()
        .map(|p| Point { x: p.x, y: -p.y })
        .collect();

    let mut hull = mbc_ch_inner(points, test_data);
    hull.append(&mut real_lh);
    test_data.time_end = std::time::SystemTime::now();
    hull
}

fn mbc_ch_inner(points: Vec<Point>, test_data: &mut MBCTestData) -> Vec<Point> {
    test_data.recursions += 1;
    if points.len() < 2 {
        return points;
    } else if points.len() == 2 {
        return if points[0].x < points[1].x {
            points
        } else {
            vec![points[1], points[0]]
        };
    }

    let median_x = find_sampled_median_x(&points, 5);

    let (left_point, right_point) = bridge(&points, median_x, test_data);

    let left = {
        let mut left: Vec<_> = points
            .iter()
            .filter(|p| p.x < left_point.x)
            .cloned()
            .collect();
        left.push(left_point);
        left
    };
    let right = {
        let mut right: Vec<Point> = points
            .into_iter()
            .filter(|p| p.x > right_point.x)
            .collect();
        right.push(right_point);
        right
    };

    let mut hull = mbc_ch_inner(left, test_data);
    hull.append(&mut mbc_ch_inner(right, test_data));
    hull
}

#[cfg(test)]
mod tests {
    use crate::{Point, mbc_ch, draw, bridge, MBCTestData};
    use std::fs::read;
    use rand::Rng;

    #[test]
    fn mbc_ch_test() {
        let points = vec![
            Point { x: 41, y: -6 },
            Point { x: -24, y: -74 },
            Point { x: -51, y: -6 },
            Point { x: 73, y: 17 },
            Point { x: -30, y: -34 },
        ];
        let mut test_data = MBCTestData::new();
        assert_eq!(mbc_ch(points, &mut test_data), [Point { x: -51, y: -6 }, Point { x: 73, y: 17 }, Point { x: -24, y: -74 }]);
    }

    #[test]
    fn mbc_ch_test2() {
        let points = vec![
            Point { x: 4, y: 11 },
            Point { x: 14, y: 13 },
            Point { x: 21, y: 10 },
            Point { x: 18, y: 11 },
            Point { x: 10, y: 11 },
            Point { x: 13, y: 6 },
            Point { x: 7, y: 6 },
            Point { x: 19, y: 5 },
            Point { x: 1, y: 7 },
        ];
        let mut test_data = MBCTestData::new();
        println!("{:?}", mbc_ch(points, &mut test_data));
    }

    #[test]
    fn bridge_test_two_points() {
        let points = bridge(&vec![
            Point { x: 4, y: 11 },
            Point { x: 14, y: 13 },
        ], 4, &mut MBCTestData::new());
        assert_eq!(points, (Point { x: 4, y: 11 }, Point { x: 14, y: 13 }));
        let points = bridge(&vec![
            Point { x: 14, y: 13 },
            Point { x: 4, y: 11 },
        ], 4, &mut MBCTestData::new());
        assert_eq!(points, (Point { x: 4, y: 11 }, Point { x: 14, y: 13 }));
    }

    #[test]
    fn bridge_test_three_points() {
        let points = bridge(&vec![
            Point { x: 19, y: 5 },
            Point { x: 21, y: 10 },
            Point { x: 13, y: 6 },
        ], 13, &mut MBCTestData::new());
        assert_eq!(points, (Point { x: 13, y: 6 }, Point { x: 21, y: 10 }));
        let points = bridge(&vec![
            Point { x: 21, y: 10 },
            Point { x: 19, y: 5 },
            Point { x: 13, y: 6 },
        ], 13, &mut MBCTestData::new());
        assert_eq!(points, (Point { x: 13, y: 6 }, Point { x: 21, y: 10 }));
        let points = bridge(&vec![
            Point { x: 13, y: 6 },
            Point { x: 19, y: 5 },
            Point { x: 21, y: 10 },
        ], 13, &mut MBCTestData::new());
        assert_eq!(points, (Point { x: 13, y: 6 }, Point { x: 21, y: 10 }));
    }

    #[test]
    fn bridge_test1() {
        let points = bridge(&vec![
            Point { x: 4, y: 11 },
            Point { x: 14, y: 13 },
            Point { x: 21, y: 10 },
            Point { x: 18, y: 11 },
            Point { x: 10, y: 11 },
            Point { x: 13, y: 6 },
            Point { x: 7, y: 6 },
            Point { x: 19, y: 5 },
            Point { x: 1, y: 7 },
        ], 10, &mut MBCTestData::new());
        assert_eq!(points, (Point { x: 4, y: 11 }, Point { x: 14, y: 13 }));
    }

    #[test]
    fn bridge_test2() {
        let points = bridge(&vec![
            Point { x: 4, y: 11 },
            Point { x: 21, y: 10 },
            Point { x: 18, y: 11 },
            Point { x: 10, y: 11 },
            Point { x: 13, y: 6 },
            Point { x: 7, y: 6 },
            Point { x: 19, y: 5 },
            Point { x: 1, y: 7 },
        ], 10, &mut MBCTestData::new());
        assert_eq!(points, (Point { x: 4, y: 11 }, Point { x: 18, y: 11 }));
    }

    #[test]
    fn that_damn_float_precision_bug() {
        let points = vec![
            Point { x: 804, y: 2271 }, Point { x: -136, y: 2382 }, Point { x: 2686, y: 4952 }, Point { x: -289, y: 3189 }, Point { x: 6560, y: 1644 }
        ];
        assert_eq!(bridge(&points, 804, &mut MBCTestData::new()), (Point { x: -289, y: 3189 }, Point { x: 2686, y: 4952 }));
    }

    #[test]
    fn mbc_perf() {
        let mut rng = rand::thread_rng();
        let mut points = std::vec::Vec::<Point>::new();

        for _ in 1..10000 {
            let a = rng.gen_range(0f64, 360f64);
            let r = 4000f64 * f64::sqrt(rng.gen_range(0f64, 1f64));
            let xx = (r * f64::cos(a)) as i64 + 3000;
            let y = (r * f64::sin(a)) as i64 + 3000;

            points.push(Point::new(xx, y))
        }
        let mut test_data = MBCTestData::new();
        mbc_ch(points, &mut test_data);
        println!("{:?}", test_data.time_elapsed().as_nanos())
    }
}