struct HanoiIterator {
    state: Vec<(usize, usize, usize, usize)>,
}

impl HanoiIterator {
    fn new(n: usize) -> Self {
        HanoiIterator {
            state: vec![(n, 0, 1, 2)],
        }
    }
}

impl Iterator for HanoiIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((n, from, aux, to)) = self.state.pop() {
            if n == 1 {
                return Some((from, to));
            } else {
                self.state.push((n - 1, aux, from, to));
                self.state.push((1, from, aux, to));
                self.state.push((n - 1, from, to, aux));
               
            }
        }
        None
    }
}

use image::{ImageBuffer, Rgb};
use gif::{Frame, Encoder, Repeat};
use std::fs::File;
// use std::borrow::Cow;
fn disk_to_ascii(size: usize, max_width: usize) -> String {
    if size == 0 {
        return " ".repeat(max_width);
    }

    let disk_width = size * 2 - 1;
    let padding = (max_width - disk_width) / 2;
    let disk = "-".repeat(disk_width);
    format!("{}{}{}+{}{}", " ".repeat(padding), disk, "-", disk, " ".repeat(padding))
}

struct TowerState {
    left: Vec<usize>,
    mid: Vec<usize>,
    right: Vec<usize>,
}

impl TowerState {
    fn new(n: usize) -> Self {
        TowerState {
            left: (1..=n).rev().collect(),
            mid: Vec::new(),
            right: Vec::new(),
        }
    }

    fn move_disk(&mut self, from: usize, to: usize) {
        let disk = match from {
            0 => self.left.pop(),
            1 => self.mid.pop(),
            2 => self.right.pop(),
            _ => None,
        };

        if let Some(disk) = disk {
            match to {
                0 => self.left.push(disk),
                1 => self.mid.push(disk),
                2 => self.right.push(disk),
                _ => {},
            }
        }
    }

    fn to_ascii(&self) -> String {
        // Implement ASCII rendering here
        // Return the ASCII representation as a String
        let max_disks = self.left.len().max(self.mid.len()).max(self.right.len());
        let max_width = max_disks * 2 + 1;
        
        let mut result = String::new();

        // Draw disks
        for i in (0..max_disks).rev() {
            let left_disk = self.left.get(i).unwrap_or(&0);
            let mid_disk = self.mid.get(i).unwrap_or(&0);
            let right_disk = self.right.get(i).unwrap_or(&0);

            let left_str = disk_to_ascii(*left_disk, max_width);
            let mid_str = disk_to_ascii(*mid_disk, max_width);
            let right_str = disk_to_ascii(*right_disk, max_width);

            result.push_str(&format!("{}   {}   {}\n", left_str, mid_str, right_str));
        }

        // Draw base
        let base = "-".repeat(max_width);
        result.push_str(&format!("{}   {}   {}\n", base, base, base));

        result
    }
}

fn ascii_to_image(ascii: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let lines: Vec<&str> = ascii.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    // Scale factor to make the image larger and more visible
    let scale = 5;
    let img_width = width * scale;
    let img_height = height * scale;

    let mut img = ImageBuffer::new(img_width as u32, img_height as u32);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let color = match ch {
                '-' => Rgb([0, 0, 255]),   // Blue for disks
                '+' => Rgb([255, 0, 0]),   // Red for disk centers
                '|' => Rgb([0, 255, 0]),   // Green for poles
                _ => Rgb([255, 255, 255]), // White for background
            };

            // Fill a scale x scale square for each character
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = (x * scale + dx) as u32;
                    let py = (y * scale + dy) as u32;
                    img.put_pixel(px, py, color);
                }
            }
        }
    }

    img
}

fn main() {
    let n = 3; // Number of disks
    let hanoi = HanoiIterator::new(n);
    let mut state = TowerState::new(n);
    
    let mut frames = Vec::new();

    for (_, (from, to)) in hanoi.enumerate() {
        state.move_disk(from, to);
        let ascii = state.to_ascii();
        let image: ImageBuffer<Rgb<u8>, Vec<u8>> = ascii_to_image(&ascii);
        frames.push(image);
    }

    // Create GIF
    let mut file = File::create("hanoi.gif").unwrap();
    let mut encoder = Encoder::new(&mut file, 100, 100, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    for image in frames {
        // Get raw pixels from ImageBuffer
        let pixels = image.into_raw();
        
        // Create Frame directly from raw pixels
        let frame = Frame::from_rgb_speed(100, 100, &pixels, 10);
        
        // Set delay (in hundredths of a second)
        let mut frame = frame.clone();
        frame.delay = 100; // 1 second delay
        
        encoder.write_frame(&frame).unwrap();
    }
}