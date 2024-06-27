#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrunState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
}
