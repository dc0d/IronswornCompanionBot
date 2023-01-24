pub struct Env {
    pub oracles: oracles::Oracles,
}

impl Env {
    pub fn load() -> Env {
        Env {
            oracles: oracles::Oracles::load(),
        }
    }
}
