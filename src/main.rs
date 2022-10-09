use image::{ImageBuffer, Rgb};
use rand::{rngs::StdRng, SeedableRng, RngCore};

const SEED: u64 = 0x5EED;

const HEIGHT: usize = 512;
const WIDTH : usize = 512;

const POINTS: usize = 64;

type ValueType = Rgb< u8 >;

fn main() {
    let mut prng = StdRng::seed_from_u64(SEED);
    let mut prng2 = StdRng::seed_from_u64(SEED);
    let mut n_map = WorleyMap::new(HEIGHT, WIDTH);
    let mut rand_points = Vec::with_capacity(POINTS);
    for _ in 0..POINTS {
        rand_points.push( (Point::from_rng(HEIGHT, &mut prng), random_colour(&mut prng2)) );
    }

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>>
            = ImageBuffer::from_fn(
                WIDTH as u32,
                HEIGHT as u32,
                |_, _| Rgb([0, 0, 0]),
            );

    n_map.dist_noise(&rand_points, Distance::Euclidean, 0);

    // Fill image with Worley values
    // for row in 0..n_map.height {
    //     for column in 0..n_map.width {
    //         let u8_noise = n_map.get(row, column) as u8;
    //         image.put_pixel(
    //             column as u32,
    //             row as u32,
    //             Rgb([u8_noise; 3])
    //         );
    //     }
    // }

    for row in 0..n_map.height {
        for column in 0..n_map.width {
            image.put_pixel(
                row as u32,
                column as u32,
                n_map.get(column, row),
            );
        }
    }

    // Draw the random points
    for (point, _) in rand_points {
        image.put_pixel(
            point.column as u32,
            point.row as u32,
            Rgb([0; 3])
        );
    }

    save_img(image, "Regions");
}

struct WorleyMap {
    height: usize,
    width: usize,
    values: Vec< ValueType >,
}

impl WorleyMap {
    fn new(height: usize, width: usize) -> Self {
        let values = vec![Rgb([0; 3]); height * width];
        WorleyMap {
            height,
            width,
            values,
        }
    }

    fn dist_noise(&mut self, regions: &Vec<(Point, Rgb< u8 >)>, dist_fn: Distance, n: usize) {
        for row in 0..self.height {
            if row % 100 == 0 { println!("Row: {}", row) }
            for column in 0..self.width {
                let mut distances = Vec::with_capacity( 512 * POINTS );
                for &(point, region_colour) in regions.iter() {

                    let dist = dist_fn.dist(
                        Point::from(column, row),
                        point
                    );
                    distances.push((dist, region_colour));
                }
                distances.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

                self.set(
                    row,
                    column,
                    distances[n].1,
                );
            }
        }
        println!("Done!");
    }

    fn index(&self, column: usize, row: usize) -> usize {
        self.height * column + row
    }

    fn get(&self, column: usize, row: usize) -> ValueType {
        self.values[ self.index(column, row) ]
    }

    fn set(&mut self, column: usize, row: usize, value: ValueType) {
        let index = self.index(column, row);
        self
            .values[ index ] = value;
    }
}

fn save_img(image: ImageBuffer<Rgb<u8>, Vec<u8>>, filename: &str) {
    let filepath = format!("images/{}.png", filename);
    let res = image.save(filepath);

    match res {
        Ok(_) => println!("Image successfully saved :D"),
        Err(e) => println!("Oh no!\n{}", e),
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    column: usize,
    row: usize,
}

impl Point {
    fn from_rng(max: usize, rng: &mut StdRng) -> Self {
        let max = max as f64;
        
        let row = (normalize(rng.next_u64()) * max) as usize;
        let column = (normalize(rng.next_u64()) * max) as usize;

        Point {
            column,
            row,
        }
    }

    fn from(column: usize, row: usize) -> Self {
        Point {
            column,
            row,
        }
    }
}

fn random_colour(rng: &mut StdRng) -> Rgb< u8 > {
    let r = (normalize(rng.next_u64()) * 255.0) as u8;
    let g = (normalize(rng.next_u64()) * 255.0) as u8;
    let b = (normalize(rng.next_u64()) * 255.0) as u8;
    Rgb([r, g, b])

}

fn normalize(x: u64) -> f64 {
    let numerator = x as f64 - 0.0;
    let denominator = u64::MAX as f64 - 0.0;
    
    numerator / denominator
}

enum Distance {
    Euclidean,
    Manhattan,
}

impl Distance {
    pub fn dist(&self, p: Point, q: Point) -> f64{
        match self {
            Self::Euclidean => Self::euclidean_dist(p, q),
            Self::Manhattan => Self::manhattan_dist(p, q),
        }
    }
}

impl Distance {
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
}
