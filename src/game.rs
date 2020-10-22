use crate::engine::*;

pub struct Game {
    m_engine: Box<dyn Engine>,
}

impl Game {
    pub fn new(engine: Box<dyn Engine>) -> Game {
        Game {
            m_engine: engine,
        }
    }

    pub fn play(&self) -> Result<(), String> {
        self.m_engine.play()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_return_pass() {
        let mut engine = Box::new(MockEngine::new());
        engine.expect_play()
            .times(1)
            .returning(|| Ok(()));
        let game = Game::new(engine);
        assert_eq!(true, game.play().is_ok());
    }
    #[test]
    fn engine_return_error() {
        let mut engine = Box::new(MockEngine::new());
        engine.expect_play()
            .times(1)
            .returning(|| Err("not working".to_string()));
        let game = Game::new(engine);
        assert_eq!(true, game.play().is_err());
    }
}