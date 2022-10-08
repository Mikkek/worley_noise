use image::{ImageBuffer, Rgb};
use rand::{rngs::StdRng, SeedableRng, RngCore};

const SEED: u64 = 0x5EED;

const HEIGHT: usize = 512;
const WIDTH : usize = 512;

const POINTS: usize = 64;

fn main() {
    info();
    let mut prng = StdRng::seed_from_u64(SEED);

    let mut n_map = NoiseMap::new(HEIGHT, WIDTH);

    let mut rand_points = Vec::with_capacity(POINTS);
    for _ in 0..POINTS {
        rand_points.push( Point::with_max(HEIGHT, &mut prng) );
    }

    n_map.values_from_distance(&rand_points, 0);

    for point in rand_points {
        n_map.set_colour(point.row, point.column, Rgb([255, 0, 255]));
    }

    n_map.save_image("Distances");
}

struct NoiseMap {
    height: usize,
    width: usize,
    values: Vec<Vec<NoisePoint>>,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl NoiseMap {
    fn new(height: usize, width: usize) -> Self {
        let values = vec![vec![NoisePoint::new(); height]; width];
        let image
            = ImageBuffer::from_fn(
                width as u32,
                height as u32,
                |_, _| Rgb([0, 0, 0]),
            );

        NoiseMap {
            height,
            width,
            values,
            image,
        }
    }

    fn from_random_noise(rng: &mut StdRng) -> NoiseMap {
        let mut n_map = NoiseMap::new(
            HEIGHT,
            WIDTH,
        );
    
        for row in 0..n_map.height {
            for column in 0..n_map.width {
                let value = normalize(rng.next_u64());
                
                n_map.set(row, column, value);
            }
        }
    
        n_map
    }

    fn values_from_distance(&mut self, points: &Vec<Point>, n: usize) {
        let mut l_dist = 0.0;
        let mut s_dist = 100.0;
        
        for row in 0..self.height {
            if row % 100 == 0 { println!("Row: {}", row) }
            for column in 0..self.width {
                let mut distances = Vec::with_capacity( 512 * POINTS );
                for &point in points.iter() {

                    let dist = euclidean_dist(
                        Point { row, column },
                        point
                    );
                    
                    if l_dist < dist { l_dist = dist }
                    if s_dist > dist { s_dist = dist }

                    distances.push(dist);
                }
                distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

                self.set(
                    row,
                    column,
                    distances[n],
                );
            }
        }

        println!("Done!");

        println!("Largest distance: {}", l_dist);
        println!("Smallest distance: {}", s_dist);
    }

    fn set(&mut self, row: usize, column: usize, value: f64) {
        self
            .values[row][column]
            .value = value;

        let val_u8 = (value) as u8;
        let colour = Rgb([val_u8; 3]);
        self.set_colour(row, column, colour);
    }

    fn set_colour(&mut self, row: usize, column: usize, colour: Rgb<u8>) {
        self.values[row][column].colour = colour;

        self.image.put_pixel(
            row as u32,
            column as u32,
            colour
        );
    }

    fn save_image(&self, filename: &str) {
        let filepath = format!("images/{}.png", filename);

        let res = self.image.save(filepath);

        match res {
            Ok(_) => println!("Successfully saved image! :D"),
            Err(e) => println!("Oh no!\n{}", e),
        }
    }
}

fn euclidean_dist(p: Point, q: Point) -> f64 {
    let p1 = p.row as f64;
    let p2 = p.column as f64;

    let q1 = q.row as f64;
    let q2 = q.column as f64;

    ((q1 - p1).powi(2) + (q2 - p2).powi(2)).sqrt()
}

fn manhattan_dist(p: Point, q: Point) -> f64 {
    let p1 = p.row as f64;
    let p2 = p.column as f64;

    let q1 = q.row as f64;
    let q2 = q.column as f64;

    (p1 - q1).abs() + (p2 - q2).abs()
}

#[derive(Debug, Copy, Clone)]
struct NoisePoint {
    value: f64,
    colour: Rgb<u8>,
}

impl NoisePoint {
    fn new() -> Self {
        NoisePoint {
            value: 0.0,
            colour: Rgb([0, 0, 0]),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn with_max(max: usize, rng: &mut StdRng) -> Self {
        let max = max as f64;
        
        let row = (normalize(rng.next_u64()) * max) as usize;
        let column = (normalize(rng.next_u64()) * max) as usize;

        Point{ row, column }
    }
}

fn normalize(x: u64) -> f64 {
    let numerator = x as f64 - 0.0;
    let denominator = u64::MAX as f64 - 0.0;

    numerator / denominator
}

fn normalize_list(values: Vec<f64>) -> Vec<f64> {
    let mut normalized_list = Vec::new();

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for &value in values.iter() {
        if min > value { min = value }
        if max < value { max = value }
    }

    for value in values {
        let numerator = value - min;
        let denominator = max - min;

        normalized_list.push(numerator / denominator);
    }

    normalized_list
}

fn info() {
    println!("
        \tu32::MIN = {}
        \tu32::MAX = {}
        ",
        0, "4.294.967.295"
    );
}
