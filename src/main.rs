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

fn main() {
    let n = 3; // Number of disks
    let hanoi = HanoiIterator::new(n);
    
    println!("Solution for Tower of Hanoi with {} disks:", n);
    
    // Using a for loop
    for (step, (from, to)) in hanoi.enumerate() {
        println!("Step {}: Move disk from peg {} to peg {}", step + 1, from, to);
    }

    // Alternatively, you can use other iterator methods:
    
    // Collect all moves into a vector
    // let moves: Vec<_> = HanoiIterator::new(n).collect();
    
    // Count the number of moves
    // let move_count = HanoiIterator::new(n).count();
    
    // Find the first move that transfers a disk to peg 2
    // let first_to_peg_2 = HanoiIterator::new(n).find(|&(_, to)| to == 2);
}