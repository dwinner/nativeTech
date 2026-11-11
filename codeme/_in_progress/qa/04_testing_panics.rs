// 04_testing_panics.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file, compile and run the test harness:
// 1. Compile: rustc --test 04_testing_panics.rs -o test_runner
// 2. Run:     ./test_runner (or test_runner.exe on Windows)
// ---------------------------------------------------------

pub struct ScoreKeeper {
    scores: Vec<u32>,
    max_scores: usize,
}

impl ScoreKeeper {
    pub fn new(max_scores: usize) -> Self {
        // Validation: Panic if the user tries to create a useless keeper
        if max_scores == 0 {
            panic!("Cannot create a ScoreKeeper with zero capacity!");
        }
        ScoreKeeper { 
            scores: Vec::with_capacity(max_scores), 
            max_scores 
        }
    }

    pub fn add_score(&mut self, score: u32) {
        // Validation: Panic if the user tries to overflow the keeper
        if self.scores.len() >= self.max_scores {
            panic!("Cannot add score: ScoreKeeper is full!");
        }
        self.scores.push(score);
    }

    pub fn get_scores(&self) -> &[u32] {
        &self.scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_scores_within_limit() {
        let mut keeper = ScoreKeeper::new(2);
        keeper.add_score(100);
        keeper.add_score(95);
        assert_eq!(keeper.get_scores(), &[100, 95]);
    }

    // This test passes ONLY if the code inside it panics.
    // If the code runs successfully without crashing, the test fails.
    #[test]
    #[should_panic] 
    fn adding_score_to_full_keeper_panics() {
        let mut keeper = ScoreKeeper::new(1);
        keeper.add_score(80);
        
        // This line should cause a panic because capacity is 1
        keeper.add_score(70); 
    }

    // This test checks for a panic AND verifies the panic message contains specific text.
    // This ensures we are panicking for the *right* reason, not a random bug.
    #[test]
    #[should_panic(expected = "ScoreKeeper is full")] 
    fn adding_to_full_keeper_panics_with_specific_message() {
        let mut keeper = ScoreKeeper::new(1);
        keeper.add_score(88);
        keeper.add_score(99); // Panics here
    }

    #[test]
    #[should_panic(expected = "zero capacity")] 
    fn new_score_keeper_with_zero_capacity_panics() {
        ScoreKeeper::new(0); // This should panic immediately
    }
}

fn main() {
    println!("This file contains unit tests.");
    println!("Run with `rustc --test 04_testing_panics.rs`");
}