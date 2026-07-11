use crate::position::Dir;

/// Every world-mutation a blob can request in a tick, resolved via index
/// rather than `BlobId` -- `apply_phase` already has read access to
/// `blobs`/`world`, so target resolution (e.g. "who is in front of me")
/// happens there once, and `resolve_phase` just replays plain indices
/// against mutable state. Indices stay valid for the whole tick because
/// `resolve_phase` defers all removals to a single `cleanup` pass at the
/// end, instead of shrinking `blobs` mid-loop.
///
/// One flat enum on purpose: this is the seam that gets cut when actions
/// become ECS component writes -- each variant here is a candidate for a
/// future `Commands` buffer entry.
#[derive(Clone, Debug)]
pub enum Command {
    Move { actor: usize, dir: Dir },
    Turn { actor: usize },
    Eat { actor: usize },
    Photosynthesize { actor: usize },
    Replicate { actor: usize },
    Attack { actor: usize, target: usize },
    SpendEnergy { actor: usize, amount: i16 },
}
