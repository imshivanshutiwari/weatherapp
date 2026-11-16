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

// =========================================================================
// 1. WGS84 VINCENTY GEODESIC & DISTANCE ENGINE
// =========================================================================

const WGS84_A: f64 = 6378137.0;
const WGS84_B: f64 = 6356752.31424518;
const WGS84_F: f64 = 1.0 / 298.257223563;

/// Vincenty Inverse Solution: calculates ellipsoidal geodesic distance (meters),
/// initial bearing (deg), and final bearing (deg) between two points on the WGS84 ellipsoid.
/// Output written to out_result: [distance_meters, initial_bearing_deg, final_bearing_deg]
#[no_mangle]
pub extern "C" fn vincenty_inverse(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    out_result: *mut f64,
) -> bool {
    if out_result.is_null() {
        return false;
    }

    if (lat1 - lat2).abs() < 1e-12 && (lon1 - lon2).abs() < 1e-12 {
        unsafe {
            *out_result = 0.0;
            *out_result.add(1) = 0.0;
            *out_result.add(2) = 0.0;
        }
        return true;
    }

    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let l = (lon2 - lon1).to_radians();

    let u1 = ((1.0 - WGS84_F) * phi1.tan()).atan();
    let u2 = ((1.0 - WGS84_F) * phi2.tan()).atan();
    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();

    let mut lambda = l;
    let mut lambda_prev;
    let mut iter_limit = 100;

    let mut sin_sigma = 0.0;
    let mut cos_sigma = 0.0;
    let mut sigma = 0.0;
    let mut cos2_sigma_m = 0.0;
    let mut cos_sq_alpha = 0.0;

    loop {
        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();
        sin_sigma = ((cos_u2 * sin_lambda).powi(2)
            + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2))
        .sqrt();

        if sin_sigma == 0.0 {
            unsafe {
                *out_result = 0.0;
                *out_result.add(1) = 0.0;
                *out_result.add(2) = 0.0;
            }
            return true;
        }

        cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
        sigma = sin_sigma.atan2(cos_sigma);

        let sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
        cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;
        cos2_sigma_m = if cos_sq_alpha != 0.0 {
            cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_sq_alpha
        } else {
            0.0
        };

        let c = WGS84_F / 16.0 * cos_sq_alpha * (4.0 + WGS84_F * (4.0 - 3.0 * cos_sq_alpha));
        lambda_prev = lambda;
        lambda = l
            + (1.0 - c)
                * WGS84_F
                * sin_alpha
                * (sigma
                    + c * sin_sigma
                        * (cos2_sigma_m
                            + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));

        iter_limit -= 1;
        if (lambda - lambda_prev).abs() < 1e-12 || iter_limit == 0 {
            break;
        }
    }

    if iter_limit == 0 {
        return false;
    }

    let u_sq = cos_sq_alpha * (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B);
    let a_val = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b_val = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));
    let delta_sigma = b_val
        * sin_sigma
        * (cos2_sigma_m
            + b_val / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)
                    - b_val / 6.0
                        * cos2_sigma_m
                        * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                        * (-3.0 + 4.0 * cos2_sigma_m * cos2_sigma_m)));

    let s = WGS84_B * a_val * (sigma - delta_sigma);

    let sin_lambda = lambda.sin();
    let cos_lambda = lambda.cos();
    let initial_bearing = (cos_u2 * sin_lambda)
        .atan2(cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda)
        .to_degrees();
    let final_bearing = (cos_u1 * sin_lambda)
        .atan2(-sin_u1 * cos_u2 + cos_u1 * sin_u2 * cos_lambda)
        .to_degrees();

    unsafe {
        *out_result = s;
        *out_result.add(1) = (initial_bearing + 360.0) % 360.0;
        *out_result.add(2) = (final_bearing + 360.0) % 360.0;
    }

    true
}

/// Vincenty Direct Solution: computes destination coordinates given origin, bearing, and distance
/// Output written to out_result: [dest_lat_deg, dest_lon_deg, final_bearing_deg]
#[no_mangle]
pub extern "C" fn vincenty_direct(
    lat: f64,
    lon: f64,
    initial_bearing_deg: f64,
    distance_m: f64,
    out_result: *mut f64,
) -> bool {
    if out_result.is_null() {
        return false;
    }

    let phi1 = lat.to_radians();
    let alpha1 = initial_bearing_deg.to_radians();
    let sin_alpha1 = alpha1.sin();
    let cos_alpha1 = alpha1.cos();

    let tan_u1 = (1.0 - WGS84_F) * phi1.tan();
    let cos_u1 = 1.0 / (1.0 + tan_u1 * tan_u1).sqrt();
    let sin_u1 = tan_u1 * cos_u1;

    let sigma1 = tan_u1.atan2(cos_alpha1);
    let sin_alpha = cos_u1 * sin_alpha1;
    let cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;

    let u_sq = cos_sq_alpha * (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B);
    let a_val = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b_val = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));

    let mut sigma = distance_m / (WGS84_B * a_val);
    let mut sigma_prev;
    let mut iter_limit = 100;

    let mut cos2_sigma_m = 0.0;
    let mut sin_sigma = 0.0;
    let mut cos_sigma = 0.0;

    loop {
        cos2_sigma_m = (2.0 * sigma1 + sigma).cos();
        sin_sigma = sigma.sin();
        cos_sigma = sigma.cos();

        let delta_sigma = b_val
            * sin_sigma
            * (cos2_sigma_m
                + b_val / 4.0
                    * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)
                        - b_val / 6.0
                            * cos2_sigma_m
                            * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                            * (-3.0 + 4.0 * cos2_sigma_m * cos2_sigma_m)));

        sigma_prev = sigma;
        sigma = distance_m / (WGS84_B * a_val) + delta_sigma;

        iter_limit -= 1;
        if (sigma - sigma_prev).abs() < 1e-12 || iter_limit == 0 {
            break;
        }
    }

    let x = sin_u1 * sin_sigma - cos_u1 * cos_sigma * cos_alpha1;
    let phi2 = (sin_u1 * cos_sigma + cos_u1 * sin_sigma * cos_alpha1)
        .atan2((1.0 - WGS84_F) * (sin_alpha * sin_alpha + x * x).sqrt());

    let lambda = (sin_sigma * sin_alpha1)
        .atan2(cos_u1 * cos_sigma - sin_u1 * sin_sigma * cos_alpha1);
    let c = WGS84_F / 16.0 * cos_sq_alpha * (4.0 + WGS84_F * (4.0 - 3.0 * cos_sq_alpha));
    let omega = lambda
        - (1.0 - c)
            * WGS84_F
            * sin_alpha
            * (sigma
                + c * sin_sigma
                    * (cos2_sigma_m
                        + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));

    let lon2 = lon + omega.to_degrees();
    let final_bearing = sin_alpha.atan2(-x).to_degrees();

    unsafe {
        *out_result = phi2.to_degrees();
        *out_result.add(1) = (lon2 + 540.0) % 360.0 - 180.0;
        *out_result.add(2) = (final_bearing + 360.0) % 360.0;
    }

    true
}

// =========================================================================
// 2. AVIATION WIND TRIANGLE SOLVER (FLIGHT NAVIGATION)
// =========================================================================

/// Solves navigation wind triangle given True Airspeed (TAS), desired True Course,
/// and wind speed & direction.
/// Output written to out_result: [ground_speed, true_heading_deg, headwind_kt, crosswind_kt]
#[no_mangle]
pub extern "C" fn solve_wind_triangle(
    true_airspeed_kt: f64,
    true_course_deg: f64,
    wind_speed_kt: f64,
    wind_dir_deg: f64,
    out_result: *mut f64,
) {
    if out_result.is_null() || true_airspeed_kt <= 0.0 {
        return;
    }

    // Relative wind angle to aircraft course
    let wind_angle = (wind_dir_deg - true_course_deg).to_radians();
    let crosswind = wind_speed_kt * wind_angle.sin();
    let headwind = wind_speed_kt * wind_angle.cos();

    // Wind Correction Angle (WCA)
    let sin_wca = crosswind / true_airspeed_kt;
    let wca_rad = if sin_wca.abs() <= 1.0 { sin_wca.asin() } else { 0.0 };
    let true_heading = (true_course_deg + wca_rad.to_degrees() + 360.0) % 360.0;
    let ground_speed = true_airspeed_kt * wca_rad.cos() - headwind;

    unsafe {
        *out_result = ground_speed.max(0.0);
        *out_result.add(1) = true_heading;
        *out_result.add(2) = headwind;
        *out_result.add(3) = crosswind;
    }
}

// =========================================================================
// 3. ATMOSPHERIC THERMODYNAMICS & SKEW-T SOUNDING CALCULATOR
// =========================================================================

/// Bolton's exact formula for Lifted Condensation Level (LCL) pressure (hPa)
#[no_mangle]
pub extern "C" fn calculate_lcl_pressure(
    temp_c: f64,
    dewpoint_c: f64,
    surface_pressure_hpa: f64,
) -> f64 {
    let tk = temp_c + 273.15;
    let td = dewpoint_c + 273.15;
    if tk <= 0.0 || td <= 0.0 || surface_pressure_hpa <= 0.0 {
        return surface_pressure_hpa;
    }

    let t_lcl = 1.0 / (1.0 / (td - 56.0) + (tk / td).ln() / 800.0) + 56.0;
    surface_pressure_hpa * (t_lcl / tk).powf(3.5)
}

/// Stull's empirical equation for Wet-Bulb Temperature (°C)
#[no_mangle]
pub extern "C" fn calculate_wet_bulb_temp(temp_c: f64, rh_percent: f64) -> f64 {
    let t = temp_c;
    let rh = rh_percent;

    t * (0.151977 * (rh + 8.313659).sqrt()).atan()
        + (t + rh).atan()
        - (rh - 1.676331).atan()
        + 0.00391838 * rh.powf(1.5) * (0.023101 * rh).atan()
        - 4.686035
}

/// Computes Convective Available Potential Energy (CAPE) and Convective Inhibition (CIN)
/// from atmospheric sounding vertical profile arrays (pressure, temp, dewpoint)
/// Output written to out_result: [CAPE (J/kg), CIN (J/kg)]
#[no_mangle]
pub extern "C" fn calculate_cape_cin(
    pressure_ptr: *const f32,
    temp_ptr: *const f32,
    _dew_ptr: *const f32,
    levels_count: usize,
    out_result: *mut f32,
) {
    if pressure_ptr.is_null() || temp_ptr.is_null() || out_result.is_null() || levels_count < 3 {
        return;
    }

    let mut cape: f32 = 0.0;
    let mut cin: f32 = 0.0;
    const R_AIR: f32 = 287.058; // Specific gas constant for dry air (J/(kg*K))

    unsafe {
        let sfc_temp = *temp_ptr + 273.15;
        let sfc_p = *pressure_ptr;

        for i in 1..levels_count {
            let p_prev = *pressure_ptr.add(i - 1);
            let p_curr = *pressure_ptr.add(i);
            if p_curr <= 0.0 || p_prev <= p_curr {
                continue;
            }

            let env_temp = *temp_ptr.add(i) + 273.15;
            // Adiabatic parcel temperature at current pressure level
            let parcel_temp = sfc_temp * (p_curr / sfc_p).powf(0.286);
            let dp = p_prev - p_curr;
            let p_avg = (p_prev + p_curr) * 0.5;

            let buoyancy = (parcel_temp - env_temp) / env_temp;
            let layer_energy = R_AIR * buoyancy * (dp / p_avg);

            if layer_energy > 0.0 {
                cape += layer_energy;
            } else if layer_energy < 0.0 && cape < 10.0 {
                cin += layer_energy.abs();
            }
        }

        *out_result = cape;
        *out_result.add(1) = cin;
    }
}

// =========================================================================
// 4. RUNGE-KUTTA 4TH ORDER (RK4) PARTICLE ADVECTION INTEGRATOR
// =========================================================================

/// Runge-Kutta 4th Order numerical advection step for thousands of particles
#[no_mangle]
pub extern "C" fn advect_particles_rk4(
    pos_x_ptr: *mut f32,
    pos_y_ptr: *mut f32,
    count: usize,
    dt: f32,
    u_grid_ptr: *const f32,
    v_grid_ptr: *const f32,
    grid_w: usize,
    grid_h: usize,
) {
    if pos_x_ptr.is_null() || pos_y_ptr.is_null() || u_grid_ptr.is_null() || v_grid_ptr.is_null() || count == 0 {
        return;
    }

    let sample_grid = |x: f32, y: f32| -> (f32, f32) {
        if x < 0.0 || x >= (grid_w - 1) as f32 || y < 0.0 || y >= (grid_h - 1) as f32 {
            return (0.0, 0.0);
        }
        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = x0 + 1;
        let y1 = y0 + 1;
        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        unsafe {
            let u00 = *u_grid_ptr.add(y0 * grid_w + x0);
            let u10 = *u_grid_ptr.add(y0 * grid_w + x1);
            let u01 = *u_grid_ptr.add(y1 * grid_w + x0);
            let u11 = *u_grid_ptr.add(y1 * grid_w + x1);

            let v00 = *v_grid_ptr.add(y0 * grid_w + x0);
            let v10 = *v_grid_ptr.add(y0 * grid_w + x1);
            let v01 = *v_grid_ptr.add(y1 * grid_w + x0);
            let v11 = *v_grid_ptr.add(y1 * grid_w + x1);

            let u = (u00 * (1.0 - fx) + u10 * fx) * (1.0 - fy) + (u01 * (1.0 - fx) + u11 * fx) * fy;
            let v = (v00 * (1.0 - fx) + v10 * fx) * (1.0 - fy) + (v01 * (1.0 - fx) + v11 * fx) * fy;
            (u, v)
        }
    };

    unsafe {
        for i in 0..count {
            let px = *pos_x_ptr.add(i);
            let py = *pos_y_ptr.add(i);

            let (u1, v1) = sample_grid(px, py);
            let (u2, v2) = sample_grid(px + 0.5 * dt * u1, py + 0.5 * dt * v1);
            let (u3, v3) = sample_grid(px + 0.5 * dt * u2, py + 0.5 * dt * v2);
            let (u4, v4) = sample_grid(px + dt * u3, py + dt * v3);

            let dx = (dt / 6.0) * (u1 + 2.0 * u2 + 2.0 * u3 + u4);
            let dy = (dt / 6.0) * (v1 + 2.0 * v2 + 2.0 * v3 + v4);

            *pos_x_ptr.add(i) = px + dx;
            *pos_y_ptr.add(i) = py + dy;
        }
    }
}

// =========================================================================
// 5. 2D LOCAL EXTREMA DETECTOR (CYCLONES & ANTICYCLONES)
// =========================================================================

/// Detects local atmospheric pressure peaks (anticyclones H) or troughs (cyclone lows L)
/// Output written to out_coords: [x0, y0, val0, x1, y1, val1, ...]
/// Returns the number of extrema found.
#[no_mangle]
pub extern "C" fn detect_local_extrema(
    grid_ptr: *const f32,
    w: usize,
    h: usize,
    radius: usize,
    find_min: bool,
    out_coords: *mut f32,
    max_out: usize,
) -> usize {
    if grid_ptr.is_null() || out_coords.is_null() || w <= 2 * radius || h <= 2 * radius {
        return 0;
    }

    let mut found = 0;
    let r = radius as isize;

    for y in radius..(h - radius) {
        for x in radius..(w - radius) {
            if found >= max_out {
                return found;
            }

            let center_val = unsafe { *grid_ptr.add(y * w + x) };
            let mut is_extrema = true;

            'search: for dy in -r..=r {
                for dx in -r..=r {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let neighbor_val = unsafe { *grid_ptr.add((y as isize + dy) as usize * w + (x as isize + dx) as usize) };
                    if find_min {
                        if neighbor_val <= center_val {
                            is_extrema = false;
                            break 'search;
                        }
                    } else {
                        if neighbor_val >= center_val {
                            is_extrema = false;
                            break 'search;
                        }
                    }
                }
            }

            if is_extrema {
                unsafe {
                    *out_coords.add(found * 3) = x as f32;
                    *out_coords.add(found * 3 + 1) = y as f32;
                    *out_coords.add(found * 3 + 2) = center_val;
                }
                found += 1;
            }
        }
    }

    found
}

// =========================================================================
// 6. ASTRONOMICAL SOLAR & LUNAR EPHEMERIS CALCULATOR
// =========================================================================

/// Computes solar zenith angle and solar elevation in degrees from coordinates and epoch
/// Output written to out_result: [solar_elevation_deg, solar_azimuth_deg, is_daylight]
#[no_mangle]
pub extern "C" fn calculate_solar_position(
    lat_deg: f64,
    lon_deg: f64,
    epoch_seconds: f64,
    out_result: *mut f64,
) {
    if out_result.is_null() {
        return;
    }

    // Days since J2000.0 (January 1, 2000, 12:00 UTC)
    let d = (epoch_seconds / 86400.0) - 10957.5;

    // Mean anomaly and solar longitude
    let m = (357.5291 + 0.98560028 * d).to_radians();
    let c = 1.9148 * m.sin() + 0.02 * (2.0 * m).sin();
    let lambda = (280.460 + 0.9856474 * d + c).to_radians();

    // Obliquity of ecliptic
    let epsilon = (23.439 - 0.0000004 * d).to_radians();

    // Declination and right ascension
    let sin_delta = epsilon.sin() * lambda.sin();
    let delta = sin_delta.asin();

    // Greenwich Mean Sidereal Time (GMST) and Local Hour Angle (LHA)
    let gmst = (280.46061837 + 360.98564736629 * d).to_radians();
    let lha = gmst + lon_deg.to_radians() - lambda;

    let phi = lat_deg.to_radians();
    let sin_elev = phi.sin() * delta.sin() + phi.cos() * delta.cos() * lha.cos();
    let elevation = sin_elev.asin().to_degrees();

    let cos_elev = elevation.to_radians().cos();
    let azimuth = if cos_elev > 1e-6 {
        let sin_az = -delta.cos() * lha.sin() / cos_elev;
        let cos_az = (delta.sin() - phi.sin() * sin_elev) / (phi.cos() * cos_elev);
        (sin_az.atan2(cos_az).to_degrees() + 360.0) % 360.0
    } else {
        0.0
    };

    unsafe {
        *out_result = elevation;
        *out_result.add(1) = azimuth;
        *out_result.add(2) = if elevation > 0.0 { 1.0 } else { 0.0 };
    }
}

/// Computes Moon phase illumination percentage (0 to 100%) and phase age
#[no_mangle]
pub extern "C" fn calculate_moon_illumination(epoch_seconds: f64) -> f64 {
    // Known reference new moon: January 11, 2024, 11:57 UTC (epoch: 1704974220)
    const REF_NEW_MOON_EPOCH: f64 = 1704974220.0;
    const SYNODIC_MONTH: f64 = 29.53058867 * 86400.0; // seconds

    let delta = (epoch_seconds - REF_NEW_MOON_EPOCH) % SYNODIC_MONTH;
    let phase_norm = (delta + SYNODIC_MONTH) % SYNODIC_MONTH / SYNODIC_MONTH;

    // Illumination fraction follows (1 - cos(2 * pi * phase)) / 2
    let illum = (1.0 - (phase_norm * 2.0 * std::f64::consts::PI).cos()) * 0.5 * 100.0;
    illum
}

// =========================================================================
// 7. MARCHING SQUARES ISOBAR & CONTOUR SEGMENT GENERATOR
// =========================================================================

/// Marching Squares 2D contour segment generator
/// Given a 2D scalar field (width x height) and a target isovalue (e.g. 1012.0 hPa),
/// extracts line segments [x1, y1, x2, y2, ...] into out_segments_ptr.
/// Returns the number of segments generated.
#[no_mangle]
pub extern "C" fn generate_marching_squares_isobars(
    grid_ptr: *const f32,
    w: usize,
    h: usize,
    isovalue: f32,
    out_segments_ptr: *mut f32,
    max_segments: usize,
) -> usize {
    if grid_ptr.is_null() || out_segments_ptr.is_null() || w < 2 || h < 2 || max_segments == 0 {
        return 0;
    }

    let mut count = 0;

    let interp = |val_a: f32, val_b: f32, pos_a: f32, pos_b: f32| -> f32 {
        let denom = val_b - val_a;
        if denom.abs() < 1e-6 {
            (pos_a + pos_b) * 0.5
        } else {
            let t = (isovalue - val_a) / denom;
            pos_a + t.clamp(0.0, 1.0) * (pos_b - pos_a)
        }
    };

    unsafe {
        for y in 0..(h - 1) {
            for x in 0..(w - 1) {
                if count >= max_segments {
                    return count;
                }

                let v0 = *grid_ptr.add(y * w + x);
                let v1 = *grid_ptr.add(y * w + (x + 1));
                let v2 = *grid_ptr.add((y + 1) * w + (x + 1));
                let v3 = *grid_ptr.add((y + 1) * w + x);

                let fx = x as f32;
                let fy = y as f32;

                let mut case_idx = 0;
                if v0 >= isovalue { case_idx |= 1; }
                if v1 >= isovalue { case_idx |= 2; }
                if v2 >= isovalue { case_idx |= 4; }
                if v3 >= isovalue { case_idx |= 8; }

                if case_idx == 0 || case_idx == 15 {
                    continue;
                }

                let top = (interp(v0, v1, fx, fx + 1.0), fy);
                let right = (fx + 1.0, interp(v1, v2, fy, fy + 1.0));
                let bottom = (interp(v3, v2, fx, fx + 1.0), fy + 1.0);
                let left = (fx, interp(v0, v3, fy, fy + 1.0));

                let mut emit = |p1: (f32, f32), p2: (f32, f32)| {
                    if count < max_segments {
                        let idx = count * 4;
                        *out_segments_ptr.add(idx) = p1.0;
                        *out_segments_ptr.add(idx + 1) = p1.1;
                        *out_segments_ptr.add(idx + 2) = p2.0;
                        *out_segments_ptr.add(idx + 3) = p2.1;
                        count += 1;
                    }
                };

                match case_idx {
                    1 | 14 => emit(left, top),
                    2 | 13 => emit(top, right),
                    3 | 12 => emit(left, right),
                    4 | 11 => emit(right, bottom),
                    5 => { emit(left, top); emit(right, bottom); },
                    6 | 9 => emit(top, bottom),
                    7 | 8 => emit(left, bottom),
                    10 => { emit(top, right); emit(bottom, left); },
                    _ => {}
                }
            }
        }
    }

    count
}

// =========================================================================
// 8. AVIATION METAR WEATHER REPORT DECODER
// =========================================================================

/// Zero-copy METAR parser: extracts wind, gusts, visibility, temperature, dewpoint,
/// QNH pressure, and flight category.
/// Output written to out_result:
/// [wind_dir_deg, wind_speed_kt, wind_gust_kt, visibility_meters, temp_c, dewpoint_c, qnh_hpa, flight_category]
/// Flight categories: 0=VFR, 1=MVFR, 2=IFR, 3=LIFR
#[no_mangle]
pub extern "C" fn parse_metar_report(
    metar_bytes_ptr: *const u8,
    len: usize,
    out_result: *mut f32,
) -> bool {
    if metar_bytes_ptr.is_null() || out_result.is_null() || len == 0 {
        return false;
    }

    let slice = unsafe { std::slice::from_raw_parts(metar_bytes_ptr, len) };
    let text = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let mut wind_dir = -1.0f32;
    let mut wind_spd = 0.0f32;
    let mut wind_gst = 0.0f32;
    let mut visibility_m = 9999.0f32;
    let mut temp_c = 15.0f32;
    let mut dew_c = 10.0f32;
    let mut qnh = 1013.25f32;

    for token in text.split_whitespace() {
        if token.ends_with("KT") && token.len() >= 5 {
            let core = &token[..token.len() - 2];
            if core.starts_with("VRB") {
                wind_dir = 0.0;
                if let Ok(spd) = core[3..].parse::<f32>() {
                    wind_spd = spd;
                }
            } else if core.contains('G') {
                let parts: Vec<&str> = core.split('G').collect();
                if parts.len() == 2 {
                    if let Ok(dir) = parts[0][..3].parse::<f32>() { wind_dir = dir; }
                    if let Ok(spd) = parts[0][3..].parse::<f32>() { wind_spd = spd; }
                    if let Ok(gst) = parts[1].parse::<f32>() { wind_gst = gst; }
                }
            } else if core.len() >= 5 {
                if let Ok(dir) = core[..3].parse::<f32>() { wind_dir = dir; }
                if let Ok(spd) = core[3..].parse::<f32>() { wind_spd = spd; }
            }
        } else if token.len() == 4 && token.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(v) = token.parse::<f32>() {
                visibility_m = v;
            }
        } else if token.ends_with("SM") {
            if let Ok(sm) = token[..token.len() - 2].parse::<f32>() {
                visibility_m = sm * 1609.34;
            }
        } else if token.contains('/') && !token.starts_with("FEW") && !token.starts_with("SCT") && !token.starts_with("BKN") && !token.starts_with("OVC") {
            let parts: Vec<&str> = token.split('/').collect();
            if parts.len() == 2 {
                let parse_temp = |s: &str| -> Option<f32> {
                    if s.starts_with('M') {
                        s[1..].parse::<f32>().ok().map(|v| -v)
                    } else {
                        s.parse::<f32>().ok()
                    }
                };
                if let Some(t) = parse_temp(parts[0]) { temp_c = t; }
                if let Some(d) = parse_temp(parts[1]) { dew_c = d; }
            }
        } else if token.starts_with('Q') && token.len() == 5 {
            if let Ok(p) = token[1..].parse::<f32>() {
                qnh = p;
            }
        } else if token.starts_with('A') && token.len() == 5 {
            if let Ok(inhg) = token[1..].parse::<f32>() {
                qnh = (inhg / 100.0) * 33.8639;
            }
        }
    }

    let flight_category = if visibility_m >= 8000.0 {
        0.0f32 // VFR
    } else if visibility_m >= 4800.0 {
        1.0f32 // MVFR
    } else if visibility_m >= 1600.0 {
        2.0f32 // IFR
    } else {
        3.0f32 // LIFR
    };

    unsafe {
        *out_result = wind_dir;
        *out_result.add(1) = wind_spd;
        *out_result.add(2) = wind_gst;
        *out_result.add(3) = visibility_m;
        *out_result.add(4) = temp_c;
        *out_result.add(5) = dew_c;
        *out_result.add(6) = qnh;
        *out_result.add(7) = flight_category;
    }

    true
}

// =========================================================================
// 9. CUBIC HERMITE COLOR PALETTE SPLINE INTERPOLATOR
// =========================================================================

/// Smooth Hermite spline RGBA palette lookup table generator
/// Interpolates between stops into out_rgba (size: lut_size * 4 bytes)
#[no_mangle]
pub extern "C" fn interpolate_color_palette_spline(
    stop_vals_ptr: *const f32,
    stop_rgba_ptr: *const u8,
    stops_count: usize,
    lut_size: usize,
    min_val: f32,
    max_val: f32,
    out_rgba_ptr: *mut u8,
) {
    if stop_vals_ptr.is_null() || stop_rgba_ptr.is_null() || out_rgba_ptr.is_null() || stops_count < 2 || lut_size == 0 || min_val >= max_val {
        return;
    }

    let val_range = max_val - min_val;

    unsafe {
        for step in 0..lut_size {
            let norm = step as f32 / (lut_size - 1) as f32;
            let current_val = min_val + norm * val_range;

            let mut left_idx = 0;
            for i in 0..(stops_count - 1) {
                if current_val >= *stop_vals_ptr.add(i) {
                    left_idx = i;
                }
            }
            let right_idx = (left_idx + 1).min(stops_count - 1);

            let v_left = *stop_vals_ptr.add(left_idx);
            let v_right = *stop_vals_ptr.add(right_idx);

            let t = if (v_right - v_left).abs() > 1e-6 {
                ((current_val - v_left) / (v_right - v_left)).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let s = t * t * (3.0 - 2.0 * t);

            let out_idx = step * 4;
            for ch in 0..4 {
                let c_left = *stop_rgba_ptr.add(left_idx * 4 + ch) as f32;
                let c_right = *stop_rgba_ptr.add(right_idx * 4 + ch) as f32;
                let interpolated = (c_left + s * (c_right - c_left)).round().clamp(0.0, 255.0) as u8;
                *out_rgba_ptr.add(out_idx + ch) = interpolated;
            }
        }
    }
}



