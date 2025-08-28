use rayon::prelude::*;
use chrono::prelude::*;

// Takes camera height(meters), camera angle(degrees), and camera view width and height(px), and a vector of 2d coordinates on the camera's view in pixel coords
// and returns a vector of 2d coordinates projected onto the ground in world coordinates
pub fn flatten(
    cam_height: f64,
    cam_angle: f64,
    view_width: f64,
    view_height: f64,
    fov: f64,
    coords: &Vec<(f64, f64, Option<DateTime<Utc>>)>,
) -> Vec<(f64, f64, Option<DateTime<Utc>>)> {
    let angle_rad = cam_angle.to_radians();

    // horizontal field of view
    let fov_rad = fov.to_radians();
    let half_fov = fov_rad / 2.0;

    // Calculate vertical FOV based on aspect ratio
    let aspect_ratio = view_width / view_height;
    let vfov_rad = 2.0 * (half_fov.tan() / aspect_ratio).atan();
    let half_vfov = vfov_rad / 2.0;

    return coords
        .clone()
        .par_iter()
        .filter_map(|&(x, y, time)| {
            // Convert pixel coordinates to normalized device coordinates (-1 to 1)
            let ndc_x = (2.0 * x as f64 / view_width) - 1.0;
            let ndc_y = 1.0 - (2.0 * y as f64 / view_height);

            // Calculate ray angles from camera center
            let ray_angle_x = ndc_x * half_fov;
            let ray_angle_y = ndc_y * half_vfov;

            // Calculate ray direction - camera angle is downward from horizontal
            let ray_pitch = angle_rad - ray_angle_y;

            // Skip if ray doesn't hit ground (looking upward)
            if ray_pitch <= 0.0 {
                return None;
            }

            // Calculate ground intersection using trigonometry
            // Distance forward from camera base (y-axis)
            let forward_distance = cam_height / ray_pitch.tan();
            // Lateral distance (x-axis)
            let lateral_distance = forward_distance * ray_angle_x.tan();

            Some((lateral_distance, forward_distance, time))
        })
        .collect::<Vec<(f64, f64, Option<DateTime<Utc>>)>>();
}

pub fn normalize(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let x_max = 640.0;
    let y_max = 480.0;

    points
        .iter()
        .map(|&(x, y)| (x / x_max, y / y_max))
        .collect()
}
