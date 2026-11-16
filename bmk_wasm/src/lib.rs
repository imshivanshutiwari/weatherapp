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
