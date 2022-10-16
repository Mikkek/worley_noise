
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
fn main() {

}
