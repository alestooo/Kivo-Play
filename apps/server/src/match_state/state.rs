#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchPhase {
    Intro,
    Playing,
    RoundResult,
    MatchResult,
}
