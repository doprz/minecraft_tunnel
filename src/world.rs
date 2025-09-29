pub struct World {
    voxels: Vec<u8>,
    pub textures: Vec<u32>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            voxels: vec![0; 64 * 64 * 64],
            textures: vec![0; 16 * 16 * 3 * 16],
        };
        world.init();
        world
    }

    pub fn init(&mut self) {
        for i in 1..16 {
            let mut brightness = 255 - (fastrand::u32(0..96));
            for y in 0..16 * 3 {
                for x in 0..16 {
                    let mut color = 0x966C4A;

                    // Stone
                    if i == 4 {
                        color = 0x7F7F7F;
                    }

                    // Brick
                    if i == 5 {
                        color = 0xB53A15;
                        if (x + (y >> 2) * 4) % 8 == 0 || y % 4 == 0 {
                            color = 0xBCAFA5;
                        }
                    }

                    // Add some noise to all blocks except stone
                    if i != 4 || fastrand::u8(0..3) != 0 {
                        brightness = 255 - (fastrand::u32(0..96));
                    }

                    // Grass
                    if i == 1 && y < (((x * x * 3 + x * 81) >> 2) & 3) + 18 {
                        color = 0x6AAA40;
                    } else if i == 1 && y < (((x * x * 3 + x * 81) >> 2) & 3) + 19 {
                        brightness = brightness * 2 / 3;
                    }

                    // Log
                    if i == 7 {
                        color = 0x675231;
                        if x > 0 && x < 15 && ((y > 0 && y < 15) || (y > 32 && y < 47)) {
                            color = 0xBC9862;
                            let mut xd = (x as i32) - 7;
                            let mut yd = ((y & 15) as i32) - 7;
                            if xd < 0 {
                                xd = 1 - xd;
                            }
                            if yd < 0 {
                                yd = 1 - yd;
                            }
                            if yd > xd {
                                xd = yd;
                            }
                            brightness = (196 - fastrand::u32(0..32) + (xd % 3) as u32 * 32) as u32;
                        } else if fastrand::bool() {
                            brightness = (brightness * (150 - (x & 1) * 100) / 100) as u32;
                        }
                    }

                    // Water
                    if i == 9 {
                        color = 0x4040FF;
                    }

                    let mut brightness_new = brightness;

                    if y >= 32 {
                        brightness_new /= 2;
                    }

                    if i == 8 {
                        color = 0x50D937;
                        if fastrand::bool() {
                            color = 0x000000;
                            brightness_new = 255;
                        }
                    }

                    let idx = x + y * 16 + i * 256 * 3;

                    let r = ((color >> 16) & 0xFF) * brightness_new / 255;
                    let g = ((color >> 8) & 0xFF) * brightness_new / 255;
                    let b = (color & 0xFF) * brightness_new / 255;
                    let color = (r << 16) | (g << 8) | b;

                    self.textures[idx as usize] = color;
                }
            }
        }

        for x in 0..64 {
            for y in 0..64 {
                for z in 0..64 {
                    let idx = z << 12 | y << 6 | x;
                    let yd = (y as f32 - 32.5) * 0.4;
                    let zd = (z as f32 - 32.5) * 0.4;

                    let block_type = fastrand::u8(0..16);
                    let dist = (yd * yd + zd * zd).sqrt().sqrt();

                    self.voxels[idx] = if fastrand::f32() < dist - 0.8 {
                        block_type
                    } else {
                        0
                    };
                }
            }
        }
    }

    pub fn get_voxel(&self, x: i32, y: i32, z: i32) -> u8 {
        if x < 0 || y < 0 || z < 0 || x >= 64 || y >= 64 || z >= 64 {
            return 0;
        }
        let idx = (z as usize) << 12 | (y as usize) << 6 | (x as usize);
        self.voxels[idx]
    }

    pub fn get_texture(&self, block_type: u8, u: usize, v: usize) -> u32 {
        if block_type == 0 || block_type >= 16 {
            return 0;
        }
        let idx = u + v * 16 + (block_type as usize) * 256 * 3;
        self.textures[idx]
    }
}
