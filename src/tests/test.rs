#[cfg(test)]

use lib::{generate_noise_map};
mod tests {
    use super::*;

    #[test]
    fn test_generate_noise_map() {
        // Define sample inputs
        let map_width = 100;
        let map_height = 100;
        let seed = 123;
        let scale = 0.1;
        let octaves = 4;
        let persistance = 0.5;
        let lacunarity = 2.0;

        // Call the function
        let result = generate_noise_map(map_width, map_height, seed, scale, octaves, persistance, lacunarity);

        // Log the result to the console
        println!("Result: {}", result);

        // Assert that the result is as expected
        assert_eq!(result, 5); // Adjust the expected result based on your function's logic
    }
}
