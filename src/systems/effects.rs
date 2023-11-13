use crate::player::Player;

#[derive(Debug, Clone, Copy)]
pub struct Effect {
    pub eff: Effects,
    pub duration: f32,
    pub max_duration: f32,
    pub is_buff: bool,
}

impl Effect {
    pub fn update(mut self, player: &mut Player) -> Self {
        self.eff = self.eff.update(player);
        self.duration -= 1.0;
        self
    }

    pub fn disposable(&mut self) -> bool {
        if self.duration <= 0.0 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Effects {
    Speed { speed: f32, linger: f32 },
    SlowFall { speed: f32 },
    Stun,
}

impl Effects {
    pub fn update(mut self, player: &mut Player) -> Self {
        match &mut self {
            Effects::Speed { speed, linger } => {
                if *speed < -player.stats.max_speed {
                    *speed = -player.stats.max_speed;
                    *linger = 0.0;
                }
                *speed -= *linger;
                player.stats.max_speed += *speed;
            }
            Effects::SlowFall { speed } => {
                if player.speed.y > 0. {
                    player.speed.y *= *speed;
                }
            }
            Effects::Stun => {
                player.effect_stats.can_jump = false;
                player.effect_stats.can_move = false;
                player.effect_stats.can_teleport = false;
                player.effect_stats.can_fly = false;
                player.effect_stats.can_croutch = false;
            }
        }
        self
    }

    pub fn is_buff(&self) -> bool {
        match self {
            Effects::Speed { speed, .. } => {
                if *speed > 0.0 {
                    true
                } else {
                    false
                }
            }
            Effects::SlowFall { .. } => false,
            Effects::Stun => false,
        }
    }
}
