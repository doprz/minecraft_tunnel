use crate::world::World;
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<u32>, // RGB framebuffer (0xRRGGBB format)
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            frame_buffer: vec![0; width * height],
        }
    }

    /// Main render function that generates a frame for the current time
    pub fn render(&mut self, world: &World, elapsed_ms: f32) -> &[u32] {
        // Clear the frame buffer
        self.frame_buffer.fill(0x000000);

        // Camera rotation
        let time_factor = (elapsed_ms % 10000.0) / 10000.0 * PI * 2.0;
        let x_rot = time_factor.sin() * 0.4 + PI / 2.0;
        let y_rot = time_factor.cos() * 0.4;

        let y_cos = y_rot.cos();
        let y_sin = y_rot.sin();
        let x_cos = x_rot.cos();
        let x_sin = x_rot.sin();

        // Camera position (moving forward through the tunnel)
        let ox = 32.5 + (elapsed_ms % 10000.0) / 10000.0 * 64.0;
        let oy = 32.5;
        let oz = 32.5;

        // Render each pixel
        for x in 0..self.width {
            let xd_screen = (x as f32 - self.width as f32 / 2.0) / self.height as f32;

            for y in 0..self.height {
                let yd_screen = (y as f32 - self.height as f32 / 2.0) / self.height as f32;
                let zd_screen = 1.0;

                // Apply rotation transformations
                let zd_rot = zd_screen * y_cos + yd_screen * y_sin;
                let yd_rot = yd_screen * y_cos - zd_screen * y_sin;

                let xd_final = xd_screen * x_cos + zd_rot * x_sin;
                let zd_final = zd_rot * x_cos - xd_screen * x_sin;

                // Cast ray using DDA algorithm
                let color = self.cast_ray_dda(
                    world,
                    Vec3::new(ox, oy, oz),
                    Vec3::new(xd_final, yd_rot, zd_final),
                );

                // Store in frame buffer
                let pixel_idx = y * self.width + x;
                self.frame_buffer[pixel_idx] = color;
            }
        }

        &self.frame_buffer
    }

    /// DDA ray casting algorithm
    /// Tests intersection with voxel boundaries along each dimension (X, Y, Z)
    /// and steps to the closest intersection point.
    fn cast_ray_dda(&self, world: &World, origin: Vec3, direction: Vec3) -> u32 {
        let mut col = 0u32;
        let mut br = 255.0;
        let mut ddist = 0.0;
        let mut closest = 32.0;

        // Test each dimension (X=0, Y=1, Z=2)
        for d in 0..3 {
            let dim_length = match d {
                0 => direction.x,
                1 => direction.y,
                _ => direction.z,
            };

            // Skip if ray is parallel to this set of planes
            if dim_length.abs() < 1e-10 {
                continue;
            }

            let ll = 1.0 / dim_length.abs();
            let xd = direction.x * ll;
            let yd = direction.y * ll;
            let zd = direction.z * ll;

            // Calculate initial offset to next grid boundary
            let mut initial = match d {
                0 => origin.x - origin.x.floor(),
                1 => origin.y - origin.y.floor(),
                _ => origin.z - origin.z.floor(),
            };

            if dim_length > 0.0 {
                initial = 1.0 - initial;
            }

            let mut dist = ll * initial;

            // Starting position for ray marching
            let mut xp = origin.x + xd * initial;
            let mut yp = origin.y + yd * initial;
            let mut zp = origin.z + zd * initial;

            // Adjust position for negative direction
            if dim_length < 0.0 {
                match d {
                    0 => xp -= 1.0,
                    1 => yp -= 1.0,
                    _ => zp -= 1.0,
                }
            }

            // March along the ray
            while dist < closest {
                // Get voxel at current position
                let vx = (xp as i32) & 63;
                let vy = (yp as i32) & 63;
                let vz = (zp as i32) & 63;

                let tex = world.get_voxel(vx, vy, vz);

                if tex > 0 {
                    // Calculate texture coordinates based on hit face
                    let (u, v) = if d == 1 {
                        // Hit Y face (top/bottom)
                        let u = ((xp * 16.0) as i32) & 15;
                        let mut v = ((zp * 16.0) as i32) & 15;
                        if yd < 0.0 {
                            v += 32; // Bottom face
                        }
                        (u as usize, v as usize)
                    } else {
                        // Hit X or Z face (sides)
                        let u = (((xp + zp) * 16.0) as i32) & 15;
                        let v = (((yp * 16.0) as i32) & 15) + 16;
                        (u as usize, v as usize)
                    };

                    let cc = world.get_texture(tex, u, v);
                    if cc > 0 {
                        col = cc;
                        ddist = 255.0 - ((dist / 32.0 * 255.0) as u32).min(255) as f32;
                        // Face-based lighting (different brightness for different faces)
                        br = 255.0 * (255.0 - ((d + 2) % 3) as f32 * 50.0) / 255.0;
                        closest = dist;
                        break;
                    }
                }

                // Step to next voxel boundary
                xp += xd;
                yp += yd;
                zp += zd;
                dist += ll;
            }
        }

        // Apply lighting if we hit something
        if col > 0 {
            let r = (((col >> 16) & 0xFF) as f32 * br * ddist / (255.0 * 255.0)) as u32;
            let g = (((col >> 8) & 0xFF) as f32 * br * ddist / (255.0 * 255.0)) as u32;
            let b = ((col & 0xFF) as f32 * br * ddist / (255.0 * 255.0)) as u32;

            (r << 16) | (g << 8) | b
        } else {
            // Sky color (black)
            0x000000
        }
    }
}
