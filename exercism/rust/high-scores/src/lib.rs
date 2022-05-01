#[derive(Debug)]
pub struct HighScores<'scores> {
    scores: &'scores [u32],
    sorted_scores: Vec<u32>,
}

impl<'scores> HighScores<'scores> {
    pub fn new(scores: &'scores [u32]) -> Self {
        let mut sorted_scores = Vec::new();

        sorted_scores.extend_from_slice(scores);

        sorted_scores.sort_unstable();

        sorted_scores.reverse();

        Self {
            scores,
            sorted_scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.sorted_scores.first().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.sorted_scores[..3.min(self.scores.len())].into()
    }
}
