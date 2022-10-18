use std::fmt::Debug;

use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng, seq::SliceRandom,
};
use rand_pcg::Lcg64Xsh32;

/// Increasing the table size would increase the quality of randomness we generate, 
/// however, the creation of a PermutationTable is rather expensive and the size of 
/// the table is the main contributing factor to this complexity.
/// 
/// We also don't want the table to be too small, as this would decrease the quality 
/// of randomness. 256 Strikes a nice balance, giving us acceptable performance and 
/// randomness. Also this is what i saw everyone else use so... Ken Perlin also used 
/// a PermutationTable of this size for his perlin noise and if it's good enough for 
/// Ken it's good enough for me :) 
const TABLE_SIZE: usize = 256;

/// A pseudo-random permutation of the numbers [0; 256[
pub struct PermutationTable {
    // Maybe just use a Vec<u8>?
    values: [u8; TABLE_SIZE],
}

impl PermutationTable {

    /// Generates a new, random, PermutationTable from the given seed.
    pub fn new(seed: u64) -> Self {
        println!("Generating permutation table...");
        let mut prng = Lcg64Xsh32::seed_from_u64(seed);
        let res = prng.gen();
        println!("Permutation table generated ✔\n");
        res
    }

    /// Hash into the PermutationTable with an arbitrary amount of points.
    /// 
    /// This is a simple Pearson Hash. Except the Pearson hash i found was wrong, or i understood it wrong lol. 
    /// originally i was reducing like `values[a ^ b]`, but this give TERRIBLE results. The code `values [a] ^ b` 
    /// works a lot better, and makes more sense.
    /// 
    /// # Panics
    /// 
    /// If i made a very big mistake
    pub fn hash(&self, input: &[i32]) -> u8 {
        let index = input
            .iter()
            .map(|&a| a as usize)   // Cast to usize
            .map(|a| a & 0xff ) // Reduce values to the last 8 bits
            .reduce(|a, b| (self.values[a] as usize ^ b))
            .unwrap();
        self.values[index]
    }

    /// Returns the permutation table originally used by Ken Perlin when implementing Perlin Noise.
    /// 
    /// Debug type shit.
    pub fn ken_table() -> Self {
        PermutationTable { 
            values: [151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7,
            225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247,
            120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33,
            88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134,
            139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220,
            105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80,
            73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
            164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38,
            147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189,
            28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101,
            155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
            178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12,
            191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181,
            199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236,
            205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180] }
    }
}

/// Define how a standard distribution should produce a PermutationTable.
/// 
/// This means that anything implementing the [Rng] trait can produce a randomized PermutationTable, 
/// but since a PermutationTable mut be created though the new() method, we only use [SplitMix64].
/// 
/// ## Note
/// It might be an idea to come back to this and find a better implementation for randomizing the PermutationTable. 
/// As it is right now "bad" PermutationTables could be created giving repetetive results, but it's probably not a big deal.
impl Distribution<PermutationTable> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PermutationTable {
        // The sorted table
        let mut seq: Vec<u8> = (0..TABLE_SIZE)
            .into_iter()
            .map(|val| val as u8)
            .collect();
        
        // Shuffle. Now it's a permutation
        seq.shuffle(rng);

        let mut perm_table = PermutationTable { values: [0; TABLE_SIZE] };

        // Insert the values into a PermutationTable and return it
        seq.into_iter()
            .zip(perm_table.values.iter_mut())
            .for_each( |(seq_val, perm_val)| {
                *perm_val = seq_val;
            });

        perm_table
    }
}

/// A pretier output than had we simply used a derive macro
impl Debug for PermutationTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::from("PermutationTable {\n");
        for i in 0..TABLE_SIZE/16 {
            text.push_str("\t");
            for j in 0..16 {
                let mut val = self.values[ i * 16 + j ].to_string();
                while val.len() < 3 {
                    val = format!(" {}", val);
                }
                text.push_str( &format!( "{}| ", val ) );
            }
            text.push_str("\n");
        }
        text.push_str("}");
        
        write!(f, "{}", text)
    }
}
