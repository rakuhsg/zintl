pub mod parallel;
pub mod sequenced;

pub enum TaskPriorityHint {
    Lowest,
    BestEffort,
    InputBlocking,
    Immidiately,
}
