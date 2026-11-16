use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    io::{self, Cursor, Read, Write},
    path::Path,
    time::Instant,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelGrid {
    pub model_name: String,
    pub parameter: String,
    pub unit: String,
    pub ref_time: String,
    pub nx: usize,
    pub ny: usize,
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
    pub dx: f64,
    pub dy: f64,
    pub values: Vec<f32>,
}

impl ModelGrid {
    /// Bilinear interpolation of physical parameter at (lat, lon)
    pub fn sample_bilinear(&self, lat: f64, lon: f64) -> f32 {
        if lat < self.lat_min || lat > self.lat_max || lon < self.lon_min || lon > self.lon_max {
            return -9999.0;
        }

        let gx = (lon - self.lon_min) / self.dx;
        let gy = (self.lat_max - lat) / self.dy;

        let x0 = (gx.floor() as usize).min(self.nx - 1);
        let x1 = (x0 + 1).min(self.nx - 1);
        let y0 = (gy.floor() as usize).min(self.ny - 1);
        let y1 = (y0 + 1).min(self.ny - 1);

        let fx = (gx - x0 as f64) as f32;
        let fy = (gy - y0 as f64) as f32;

        let v00 = self.values[y0 * self.nx + x0];
        let v10 = self.values[y0 * self.nx + x1];
        let v01 = self.values[y1 * self.nx + x0];
        let v11 = self.values[y1 * self.nx + x1];

        let top = v00 * (1.0 - fx) + v10 * fx;
        let bottom = v01 * (1.0 - fx) + v11 * fx;

        top * (1.0 - fy) + bottom * fy
    }

    /// Render standard Web Mercator 256x256 RGBA weather tile
    pub fn render_mercator_tile(&self, z: u32, x: u32, y: u32) -> Vec<u8> {
        let tile_size = 256usize;
        let mut rgba = vec![0u8; tile_size * tile_size * 4];
        let n = 2.0f64.powi(z as i32);

        for ty in 0..tile_size {
            let py = y as f64 + (ty as f64 / tile_size as f64);
            let lat_rad = ((1.0 - 2.0 * py / n) * std::f64::consts::PI).sinh().atan();
            let lat_deg = lat_rad.to_degrees();

            for tx in 0..tile_size {
                let px = x as f64 + (tx as f64 / tile_size as f64);
                let lon_deg = (px / n) * 360.0 - 180.0;

                let val = self.sample_bilinear(lat_deg, lon_deg);
                let idx = (ty * tile_size + tx) * 4;

                if val <= -9000.0 {
                    rgba[idx] = 0;
                    rgba[idx + 1] = 0;
                    rgba[idx + 2] = 0;
                    rgba[idx + 3] = 0;
                } else {
                    // Encode into R, G, B channels (24-bit physical packing)
                    let normalized = ((val + 50.0) * 100.0).max(0.0).min(16777215.0) as u32;
                    rgba[idx] = (normalized & 0xFF) as u8;
                    rgba[idx + 1] = ((normalized >> 8) & 0xFF) as u8;
                    rgba[idx + 2] = ((normalized >> 16) & 0xFF) as u8;
                    rgba[idx + 3] = 255;
                }
            }
        }
        rgba
    }
}

/// GRIB2 Section 0 & Section 1 Parser
pub fn parse_grib2_header(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() < 16 {
        return Err("File too small to be GRIB2".into());
    }

    if &bytes[0..4] != b"GRIB" {
        return Err("Invalid GRIB magic indicator: expected 'GRIB'".into());
    }

    let mut cursor = Cursor::new(&bytes[4..]);
    let discipline = cursor.read_u8().map_err(|e| e.to_string())?;
    let edition = cursor.read_u8().map_err(|e| e.to_string())?;
    let total_len = cursor.read_u64::<BigEndian>().map_err(|e| e.to_string())?;

    if edition != 2 {
        return Err(format!("Unsupported GRIB edition: {}", edition));
    }

    let discipline_name = match discipline {
        0 => "Meteorological Products",
        1 => "Hydrological Products",
        2 => "Land Surface Products",
        10 => "Oceanographic Products",
        _ => "Other Discipline",
    };

    Ok(format!(
        "GRIB2 Message: Edition {}, Discipline: {} ({}), Total Message Length: {} bytes",
        edition, discipline, discipline_name, total_len
    ))
}

/// Create a synthetic GRIB2/Model grid for benchmarking and local test verification
pub fn generate_synthetic_grid(param: &str, nx: usize, ny: usize) -> ModelGrid {
    let lat_min = 6.0;
    let lat_max = 38.0;
    let lon_min = 68.0;
    let lon_max = 98.0;
    let dx = (lon_max - lon_min) / (nx as f64);
    let dy = (lat_max - lat_min) / (ny as f64);

    let mut values = Vec::with_capacity(nx * ny);
    for iy in 0..ny {
        let lat = lat_max - (iy as f64 * dy);
        for ix in 0..nx {
            let lon = lon_min + (ix as f64 * dx);
            let val = match param {
                "wind" => {
                    let u = ((lat * 0.1).sin() * 15.0) as f32;
                    let v = ((lon * 0.1).cos() * 12.0) as f32;
                    (u * u + v * v).sqrt()
                }
                "pressure" => {
                    (1013.25 + (lat * 0.05).cos() * 18.0) as f32
                }
                _ => {
                    // Temperature: Indian Subcontinent gradient (tropical south to Himalayan north)
                    let temp_c = 34.0 - ((lat - 6.0) * 0.7) + ((lon - 68.0) * 0.05);
                    temp_c as f32
                }
            };
            values.push(val);
        }
    }

    ModelGrid {
        model_name: "ECMWF-IFS-HRES".into(),
        parameter: param.into(),
        unit: if param == "temp" { "°C".into() } else if param == "wind" { "m/s".into() } else { "hPa".into() },
        ref_time: "2026-11-16T12:00:00Z".into(),
        nx,
        ny,
        lat_min,
        lat_max,
        lon_min,
        lon_max,
        dx,
        dy,
        values,
    }
}

fn print_help() {
    println!("Bhartiya Marut Kendra (BMK) - High Performance Weather Model Preprocessor");
    println!("Usage:");
    println!("  bmk_preprocessor inspect <grib2_or_netcdf_file>");
    println!("  bmk_preprocessor synthesize <param: temp|wind|pressure> <nx> <ny> <output.json>");
    println!("  bmk_preprocessor slice <input.json> <lat_min> <lat_max> <lon_min> <lon_max> <output.json>");
    println!("  bmk_preprocessor render-tile <input.json> <z> <x> <y> <output.rgba>");
    println!("  bmk_preprocessor benchmark");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "inspect" => {
            if args.len() < 3 {
                println!("Error: missing file path to inspect");
                return Ok(());
            }
            let path = &args[2];
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            match parse_grib2_header(&buffer) {
                Ok(info) => println!("Header Inspection:\n{}", info),
                Err(err) => println!("Inspection Failed: {}", err),
            }
        }
        "synthesize" => {
            let param = args.get(2).map(|s| s.as_str()).unwrap_or("temp");
            let nx: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(200);
            let ny: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(200);
            let out_path = args.get(5).map(|s| s.as_str()).unwrap_or("synthetic_grid.json");

            println!("Synthesizing high-resolution {} model grid ({}x{} = {} points)...", param, nx, ny, nx * ny);
            let grid = generate_synthetic_grid(param, nx, ny);
            let json = serde_json::to_string_pretty(&grid)?;
            let mut file = File::create(out_path)?;
            file.write_all(json.as_bytes())?;
            println!("Saved synthesized model grid to: {}", out_path);
        }
        "render-tile" => {
            let in_path = args.get(2).expect("Missing input grid JSON");
            let z: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
            let x: u32 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(23);
            let y: u32 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(14);
            let out_path = args.get(6).map(|s| s.as_str()).unwrap_or("tile.rgba");

            let data = std::fs::read_to_string(in_path)?;
            let grid: ModelGrid = serde_json::from_str(&data)?;

            println!("Rendering 256x256 Web Mercator Tile z={}/x={}/y={}...", z, x, y);
            let t0 = Instant::now();
            let rgba = grid.render_mercator_tile(z, x, y);
            let elapsed = t0.elapsed();

            std::fs::write(out_path, &rgba)?;
            println!("Rendered tile ({} bytes) in {:.2?} -> saved to {}", rgba.len(), elapsed, out_path);
        }
        "benchmark" => {
            println!("\n=======================================================");
            println!("BENCHMARKING NATIVE RUST WEATHER PREPROCESSOR");
            println!("=======================================================");

            let nx = 1000;
            let ny = 1000;
            let total_points = nx * ny;
            println!("Generating 1,000,000 point atmospheric grid (0.03° resolution)...");
            let t0 = Instant::now();
            let grid = generate_synthetic_grid("temp", nx, ny);
            println!("Grid generation time: {:.2?}", t0.elapsed());

            println!("Benchmarking 256x256 Web Mercator tile bilinear interpolation (65,536 samples)...");
            let t_tile = Instant::now();
            let rgba = grid.render_mercator_tile(5, 23, 14);
            let tile_elapsed = t_tile.elapsed();

            println!("Tile Rendering time: {:.2?} ({} samples, {} bytes)", tile_elapsed, 65536, rgba.len());
            println!("Throughput: {:.0} tiles/second ({:.2} million sampled points/sec)", 
                1.0 / tile_elapsed.as_secs_f64(),
                (65536.0 / tile_elapsed.as_secs_f64()) / 1_000_000.0
            );
            println!("=======================================================\n");
        }
        _ => {
            print_help();
        }
    }

    Ok(())
}
