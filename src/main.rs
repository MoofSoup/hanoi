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
        unimplemented!()
    }
}

fn ascii_to_image(ascii: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Convert ASCII to image
    // Return an ImageBuffer
    unimplemented!()
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