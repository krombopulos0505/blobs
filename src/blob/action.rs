pub const ACTION_COUNT: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    WalkForward,
    WalkRandom,
    Turn,
    Eat,
    Photosynthesize,
    Replicate,
    Attack,
    SetSignal,
}

impl Action {
    pub const ALL: [Self; ACTION_COUNT] = [
        Action::WalkForward, Action::WalkRandom, Action::Turn, Action::Eat,
        Action::Photosynthesize, Action::Replicate, Action::Attack, Action::SetSignal,
    ];
}
