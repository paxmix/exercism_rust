#[derive(Debug)]
pub struct HighScores {
    score: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            score: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.score
    }

    pub fn latest(&self) -> Option<u32> {
        self.score.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.score.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut result = self.score.clone();
        let len = result.len();
        result.sort();
        if result.len() < 3 {
            return result.into_iter().rev().collect();
        }
        result
            .into_iter()
            .rev()
            .take(if len < 3 { len } else { 3 })
            .collect()
    }
}
