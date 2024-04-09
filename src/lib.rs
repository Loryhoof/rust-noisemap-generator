mod utils;

use js_sys::Array;

use noise::core::perlin;
use noise::{NoiseFn, Perlin, Seedable};

use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn generate_noise_map(
    map_width: usize,
    map_height: usize,
    seed: usize,
    scale: usize,
    octaves: usize,
    persistance: f64,
    lacunarity: f64,
    offset: &JsValue
) -> js_sys::Array {
    
    let mut noise_map: Vec<Vec<f64>> = vec![vec![0.0; map_width]; map_height];

    let offset_array = offset.clone().dyn_into::<Array>().expect("Error converting offset array");
    let mut max_possible_height = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;

    let mut prng = StdRng::seed_from_u64(44);

    let mut octave_offsets: Vec<[f64; 2]> = Vec::new();

    for i in 0..octaves {
        let offset_array_x = offset_array.get(0).as_f64().expect("Failed to get offset index x");
        let offset_array_y = offset_array.get(1).as_f64().expect("Failed to get offset index y");

        let offset_x = prng.gen::<f64>() * 200000.0 - 100000.0 + offset_array_x;
        let offset_y = prng.gen::<f64>() * 200000.0 - 100000.0 + offset_array_y;
        octave_offsets.push([offset_x, offset_y]);

        max_possible_height += amplitude;
        amplitude *= persistance;
    }

    let mut min_local_noise_height = f64::MIN;
    let mut max_local_noise_height = f64::MAX;

    let half_width = map_width / 2;
    let half_height = map_height / 2;

    for x in 0..map_width {
        for y in 0..map_height {
            amplitude = 1.0;
            frequency = 1.0;
            let mut noise_height: f64 = 0.0;

            for i in 0..octaves {
                let sample_x = ((x as f64) - (half_width as f64) + octave_offsets[i][0]) / (scale as f64) * frequency;
                let sample_y = ((y as f64) - (half_height as f64) + octave_offsets[i][1]) / (scale as f64) * frequency;

                let perlin_value = Perlin::default().set_seed(44).get([sample_x, sample_y]);
                noise_height += perlin_value * amplitude;

                amplitude *= persistance;
                frequency *= lacunarity;
            }

            if noise_height > max_local_noise_height {
                max_local_noise_height = noise_height;
            }
            else if noise_height < min_local_noise_height {
                min_local_noise_height = noise_height;
            }

            noise_map[x][y] = noise_height;
        }
    }

    let js_noise_map: js_sys::Array = noise_map
    .into_iter()
    .map(|row| {
        // Convert each f64 element in the row into a JsValue
        let js_row = js_sys::Array::new_with_length(row.len() as u32);
        for (i, value) in row.into_iter().enumerate() {
            js_row.set(i as u32, JsValue::from_f64(value));
        }
        // Return the js_row
        js_row
    }).collect();

    js_noise_map
}