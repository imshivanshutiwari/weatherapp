use std::f64::consts::PI;

/// Allocate a block of memory for JS to pass data into WASM linear memory
#[no_mangle]
pub extern "C" fn alloc_buffer(size: usize) -> *mut f32 {
    let mut vec = Vec::<f32>::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec);
    ptr
}

/// Free memory previously allocated by alloc_buffer
#[no_mangle]
pub extern "C" fn dealloc_buffer(ptr: *mut f32, size: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, size, size);
        }
    }
}

/// High-speed vectorized Alpha LUT generator for particle shaders
/// Directly mirrors Windy's prepareAlphaLUT logic with compiled SIMD speed
#[no_mangle]
pub extern "C" fn calculate_alpha_lut(
    total_frames: u32,
    fade_in_ratio: f32,
    in_pow: f32,
    fade_out_ratio: f32,
    out_pow: f32,
    out_ptr: *mut f32,
) {
    if out_ptr.is_null() || total_frames == 0 {
        return;
    }

    let in_frames = (fade_in_ratio * total_frames as f32).round() as u32;
    let out_frames = (fade_out_ratio * total_frames as f32).round() as u32;
    let out_start = if total_frames > out_frames {
        total_frames - out_frames
    } else {
        0
    };

    unsafe {
        for frame in 0..total_frames {
            let mut alpha: f32 = 1.0;

            if frame < in_frames && in_frames > 0 {
                let norm = (frame as f32) / (in_frames as f32);
                alpha = norm.powf(in_pow);
            } else if frame >= out_start && out_frames > 0 {
                let norm = ((total_frames - frame) as f32) / (out_frames as f32);
                alpha = norm.powf(out_pow);
            }

            *out_ptr.add(frame as usize) = alpha;
        }
    }
}

/// Compute wind speed magnitude sqrt(u^2 + v^2) for thousands of grid points
#[no_mangle]
pub extern "C" fn batch_wind_magnitude(
    u_ptr: *const f32,
    v_ptr: *const f32,
    out_ptr: *mut f32,
    count: usize,
    multiplier: f32, // e.g. 3.6 for km/h, 1.94384 for knots
) {
    if u_ptr.is_null() || v_ptr.is_null() || out_ptr.is_null() || count == 0 {
        return;
    }

    unsafe {
        for i in 0..count {
            let u = *u_ptr.add(i);
            let v = *v_ptr.add(i);
            let speed = (u * u + v * v).sqrt() * multiplier;
            *out_ptr.add(i) = speed;
        }
    }
}

/// Spherical Earth Haversine formula (km) for distance and route planning
#[no_mangle]
pub extern "C" fn haversine_distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0088;

    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS_KM * c
}

/// Initial bearing (forward azimuth in degrees [0, 360)) between two coordinates
#[no_mangle]
pub extern "C" fn initial_bearing_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    let y = d_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * d_lon.cos();

    let brng = y.atan2(x).to_degrees();
    (brng + 360.0) % 360.0
}

/// Allocate a byte buffer in WASM linear memory
#[no_mangle]
pub extern "C" fn alloc_u8_buffer(size: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec);
    ptr
}

/// Deallocate a byte buffer in WASM linear memory
#[no_mangle]
pub extern "C" fn dealloc_u8_buffer(ptr: *mut u8, size: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, size, size);
        }
    }
}

/// Vectorized unpack RGBA raw weather tile pixels into scalar physical values
/// Formula: physical = (R + G * 256 + B * 65536) * scale + offset
#[no_mangle]
pub extern "C" fn unpack_rgba_to_values(
    rgba_ptr: *const u8,
    out_ptr: *mut f32,
    pixel_count: usize,
    scale: f32,
    offset: f32,
) {
    if rgba_ptr.is_null() || out_ptr.is_null() || pixel_count == 0 {
        return;
    }

    unsafe {
        for i in 0..pixel_count {
            let base = i * 4;
            let r = *rgba_ptr.add(base) as f32;
            let g = *rgba_ptr.add(base + 1) as f32;
            let b = *rgba_ptr.add(base + 2) as f32;
            let a = *rgba_ptr.add(base + 3);

            if a == 0 {
                *out_ptr.add(i) = -9999.0;
            } else {
                let raw_val = (r as u32 | ((g as u32) << 8) | ((b as u32) << 16)) as f64;
                *out_ptr.add(i) = (raw_val * (scale as f64) + (offset as f64)) as f32;
            }
        }
    }
}

/// Vectorized unpack RGBA wind tiles into separate U and V orthogonal velocity arrays
#[no_mangle]
pub extern "C" fn unpack_wind_uv(
    rgba_ptr: *const u8,
    u_out: *mut f32,
    v_out: *mut f32,
    pixel_count: usize,
    scale: f32,
    offset: f32,
) {
    if rgba_ptr.is_null() || u_out.is_null() || v_out.is_null() || pixel_count == 0 {
        return;
    }

    unsafe {
        for i in 0..pixel_count {
            let base = i * 4;
            let r = *rgba_ptr.add(base) as f32;
            let g = *rgba_ptr.add(base + 1) as f32;
            *u_out.add(i) = r * scale + offset;
            *v_out.add(i) = g * scale + offset;
        }
    }
}

/// Unpack Doppler radar reflectivity (dBZ) with clutter noise filtering
#[no_mangle]
pub extern "C" fn filter_radar_dbz(
    rgba_ptr: *const u8,
    out_dbz: *mut f32,
    pixel_count: usize,
    min_dbz: f32,
) {
    if rgba_ptr.is_null() || out_dbz.is_null() || pixel_count == 0 {
        return;
    }

    unsafe {
        for i in 0..pixel_count {
            let base = i * 4;
            let r = *rgba_ptr.add(base) as f32;
            let a = *rgba_ptr.add(base + 3);
            if a == 0 {
                *out_dbz.add(i) = 0.0;
            } else {
                let dbz = r * 0.5 - 32.0;
                *out_dbz.add(i) = if dbz >= min_dbz { dbz } else { 0.0 };
            }
        }
    }
}

