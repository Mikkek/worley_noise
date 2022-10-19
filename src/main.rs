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
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);  // An image we can save, rn it's empty
    
    // It's pretty important to generate as few permutation tables as possible, as it is fairly expensive.
    let perm_table = PermutationTable::new(SEED);

    println!("Drawing image...");

    // For every pixel in our image
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // The current pixel as a point
            let point = Vec2{
                x: x as f64 / SCALE,    // Scaling the values is important, as the amount of feature points / subdivisions is determined by this
                y: y as f64 / SCALE,    // Every integer defines a new subdivision, so the greater the scale, the less feature points
            };  // If you have a very low scale or none, you get some pretty crazy patterns. They are not realy usefull though.
            
            // The default variable name.
            let mut res = worley(point, &perm_table);
            
            // "De-scale" (ew) the point, so it's x, y values properlay translate on the image
            res.x *= SCALE;
            res.y *= SCALE;

            // Painting time. 
            // Here we set the colour of the pixel to a representation of the distance returned from the worley noise, 
            // but this could also be a value determined by the feature point itself, this would create a Voronoi Tessellation.
            image.put_pixel(
                x as u32,
                y as u32,
                Luma([ (res.dist * 175.0) as u8 ]).to_rgb(),
            );

            // Draw the feature points contained within the image
            if (0..WIDTH as i32).contains(&(res.x as i32)) && (0..HEIGHT as i32).contains(&(res.y as i32)) {
                image.put_pixel(
                    res.x as u32,
                    res.y as u32,
                    Rgb([255, 0, 255]),
                );
            }
        }
    }
    // Looping like this is VERY inefficient, as we have to generate the feature points for every pixel. 
    // This means, that for a 512x512 image every pixel generates 9 feature points that's 512 * 512 * 9 = 2.359.296 times total.
    //
    // I would fix this, by making a seperate noise function to generate an area of noise. This way the function can use a buffer 
    // to keep all the feature points in; this would drasticly reduce the amount of calls to the feature point generator. 
    // For the same 512x512 image, with a scale of 64, a function like this would only have to generate ((512/64) + 2)x((512/64) + 2) = 100 total feature points.
    //
    // TL;DR: this could be improved slightly.
    //
    // Although i don't know how mutch this contributes to the runtime 2.359.296 -> 100 can't be a bad improvement.
    //
    // Furthermore, holding all feature points in a vector is not actually necessary, as we only need to keep 3 rows saved at once if we move with rows then columns

    println!("Image drawn ✔\n");

    println!("Saving image...");

    let filename = "new_distance";
    let filepath = format!("images/{}.png", filename);

    // Save the image in the images folder with the name in 'filename'
    let res = image.save(filepath);

    // Any errors that occur when saving an image, we have to handle ourselves. We just panic.
    // Could use unwrap, but how would we know anything went wrong, if there is no epic UTF-8 emoji to tell us that?
    match res {
        Ok(_) => println!("Image saved, we good ✔"),
        Err(e) => panic!("Failed to save image ❌\n{}", e),
    }
}

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
fn worley(eval_point: Vec2, perm_table: &PermutationTable) -> FPoint{
// Find unit square:
    let unit_sq = eval_point.floor();

// Relative evaluation point:
    let r_point = eval_point.frac();

    let mut dists = Vec::with_capacity(9);

    for i in -1..=1 {   // Repeats for all relevant unit squares
        for j in -1..=1 {                   // We repeat for the grid relative to our unit square:
            let mut sq = unit_sq;           // (-1,  1) | ( 0,  1) | ( 1,  1)
            sq[0] += j;                     // (-1,  0) | ( 0,  0) | ( 1,  0)
            sq[1] += i;                     // (-1, -1) | ( 0, -1) | ( 1, -1)
            
            // Get the feature point of the current square
            let mut f_point = rand_vec2( perm_table.hash( &sq ) );

            // Add the current offset, as the feature point is generated relative to it's own unit square
            f_point.x += j as f64;
            f_point.y += i as f64;

            // Calculate the distance from the evaluation point to the feature point
            f_point.dist = euclidean_dist(&r_point, &f_point);

            // Rescale the feature point to its actual values (before this they were relative to the unit square "unit_sq")
            f_point.x += (sq[0] - j) as f64;
            f_point.y += (sq[1] - i) as f64;

            // All feature points are added to a vec to later be evaluated
            dists.push( f_point );
        }
    }
    // Evaluate the feature points by sorting them by their distance
    dists.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    // After sorting, the closest feature point is the first in the vector
    dists[0]
    // NOTE that while we can be sure this is the closest feature point, we cannot be so sure for the other points. 
    // It is perfectly possible for the second closest point to be outside the 3x3 grid we use here. 
    // Although it is unlikely, it is possible and would cause artifacts. 
    // 
    // If we want to generate based on the second closest feature point, we would have to evaluate in a larger, 4x4, grid. 
    // 
    // Limiting the area in which feature points can apear might also solve this, making it possible to also determine the 
    // second closest feature point from a 3x3 'evaluation grid'.
}

/// Here it is fine to use an rng, as we dictate the order in which it is used.
/// 
/// I don't know how efficient this is, the doc says "very fast" so good? If we managed to get 
/// the points from a [`PermutationTable`] it would probably be faster, but by how mutch? And 
/// will there be enough variance(probably). This should not add too mutch to the runtime, even 
/// though it is, without a doubt, overkill for this purpose.

/// # Possible changes
/// - At the moment, it is still possible for small "regions" to appear. This could be solved by limiting the 
///     range at which feature points can generate, as it is right now, feature points generate with no limits.
/// 
/// - I am not sure about the use of an RNG for the purpose of getting the feature points. I would imagine simply getting 
///     the points from hashing into the [`PermutationTable`] and doing some math magic. For now this should be fine, but 
///     it's something to consider.
fn rand_vec2(seed: u8) -> FPoint {
    let mut rng = SplitMix64::seed_from_u64(seed as u64);
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();

    FPoint {
        x,
        y,
        dist: 0.0,
    }
}

/// It's euclidean distance... not a lot more to say.
/// 
/// Though, it might be a good idea to generalize this, but that's too mutch work for me rn.
fn euclidean_dist(p: &Vec2, q: &FPoint) -> f64 {
    ((p.x - q.x).powi(2) + (p.y - q.y).powi(2)).sqrt()
}


/// A feature point type. The point is to make returning from a worley function easier.
#[derive(Debug, Copy, Clone)]
struct FPoint {
    x: f64,
    y: f64,
    dist: f64,
}
