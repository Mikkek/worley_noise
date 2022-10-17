use image::{ImageBuffer, Luma, Rgb, Pixel};
use rand::{SeedableRng, Rng};
use rand_xoshiro::SplitMix64;
use math::Vec2;
use permutation_table::PermutationTable;

mod math;
mod permutation_table;

const SEED: u64 = 0x5EED;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

const SCALE: f64 = 64.0;

fn main() {
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let perm_table = PermutationTable::new(SEED);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let point = Vec2{
                x: x as f64 / SCALE,
                y: y as f64 / SCALE,
            };
            let mut res = worley(point, &perm_table);
            res.x *= SCALE;
            res.y *= SCALE;

            image.put_pixel(
                x as u32,
                y as u32,
                Luma([ (res.dist * 255.0) as u8 ]).to_rgb(),
            );

            if (0..WIDTH as i32).contains(&(res.x as i32)) && (0..HEIGHT as i32).contains(&(res.y as i32)) {
                image.put_pixel(
                    res.x as u32,
                    res.y as u32,
                    Rgb([255, 0, 255]),
                );
            }
        }
    }

    let filename = "new_distance";
    let filepath = format!("images/{}.png", filename);

    let res = image.save(filepath);

    match res {
        Ok(_) => println!("Image saved, we good"),
        Err(e) => println!("Failed to save image!\n{}", e),
    }
}

/// # Todo
/// - Clean up code.
/// 
/// # Personal Notes
/// - There are many things that could be done to improve performance, but i don't think i will do that here. 
///     I feel a lot of improvements depend on the way in which the noise will be used.
/// 
/// - The return type of this function could be very different depending on what one might want from it. 
///     At the moment, that has all been wraped in [`WPoint`], but i am not sure that is the most appropriate 
///     solution.
/// 
/// # Plan
/// My plan for making the worley noise (not sure if it will work).
/// 
/// - Determine the unit square (like with perlin noise, use floor).
/// 
/// - Generate a reproducable feature point for the square. I think a simple LCG can accomplish this just fine. 
///     The seed for the LCG can be hashed from the unit square coords.
/// 
/// - Calculate distance between feature point and the evaluation point.
/// 
/// - Repeat distance calculation for all neighbouring squares.
/// 
/// - Return some info. I'm not sure what to exactly return yet, it could be the distance or a grouping. 
///     What the function exactly returns is not realy important though and can be easily changed.
fn worley(eval_point: Vec2, perm_table: &PermutationTable) -> WPoint{
// Find unit square:
    let unit_sq = eval_point.floor();

// Relative evaluation point:
    let r_point = eval_point.frac();

    let mut dists = Vec::with_capacity(9);

    for i in -1..=1 {   // Repeats for all relevant unit squares
        for j in -1..=1 {
            let mut sq = unit_sq;
            sq[0] += j;
            sq[1] += i;

            let mut f_point = rand_vec2( perm_table.hash( &sq ) );  // feature point generated pseudo-randomly
            f_point.x += j as f64;
            f_point.y += i as f64;

            f_point.dist = euclidean_dist(&r_point, &f_point);

            f_point.x += (sq[0] - j) as f64;
            f_point.y += (sq[1] - i) as f64;

            dists.push( f_point ); // calculate distance from evaluation point to the feature point
        }
    }
    dists.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());    // sort the distances. Index 0 is now closest

    dists[0]
}

/// Here it is fine to use an rng, as we dictate the order in which it is used.
/// 
/// I don't know how efficient this is, the doc says "very fast" so good? If we managed to get 
/// the points from a [`PermutationTable`] it would probably be faster, but by how mutch? And 
/// will there be enough variance(probably). This should not add too mutch to the runtime, even 
/// though it is, without a doubt, overkill for this purpose.
/// 
/// It is not cryptographically secure, but why would we need that? We don't use the [`SmallRng`] 
/// provided by the rand crate, as it is not reproducable across systems due to its use of [`usize`]/[`isize`].
/// 
/// At the moment, it is still possible for small "regions" to appear. This could be solved by limiting the 
/// range at which feature points can generate. At the moment, feature points can be placed at any point in 
/// the unit square.
/// 
/// [`SmallRng`]: rand::rngs::SmallRng
fn rand_vec2(seed: u8) -> WPoint {
    let mut rng = SplitMix64::seed_from_u64(seed as u64);
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();

    WPoint {
        x,
        y,
        dist: 0.0,
    }
}

fn euclidean_dist(p: &Vec2, q: &WPoint) -> f64 {
    ((p.x - q.x).powi(2) + (p.y - q.y).powi(2)).sqrt()
}

#[derive(Debug, Copy, Clone)]
struct WPoint {
    x: f64,
    y: f64,
    dist: f64,
}
