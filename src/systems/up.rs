use crate::player::Player;

use raylib::prelude::*;

use super::{
    effects::{Effect, Effects},
    units::Unit,
};

#[derive(Debug, Clone, Copy)]
pub enum Up {
    Jump {
        strength: f32,
        jump_count: u8,
        max_jumps: u8,
        can_jump: bool,
    },
    Fly {
        strength: f32,
        duration: f32,
        max_duration: f32,
    },
    Teleport {
        strength: f32,
        teleport_count: u8,
        max_teleport: u8,
        can_teleport: bool,
    },
}

impl Up {
    pub fn new() -> Up {
        Up::Jump {
            strength: 13.0,
            jump_count: 0,
            max_jumps: 2,
            can_jump: true,
        }
        /*Up::Fly {
            strength: 6.0,
            duration: 0.0,
            max_duration: 300.0,
        }*/
        /*Up::Teleport {
            strength: 130.0,
            teleport_count: 0,
            max_teleport: 5,
            can_teleport: true,
        }*/
    }

    pub fn up(mut self, unit: &mut dyn Unit, rl: &RaylibHandle, thread: &RaylibThread) -> Self {
        match &mut self {
            Up::Jump {
                strength,
                jump_count,
                max_jumps,
                can_jump,
            } => {
                if *can_jump && jump_count <= max_jumps && unit.get_effect_stats_mut().can_jump {
                    *jump_count += 1;
                    unit.get_speed_mut().y = -*strength;
                    *can_jump = false;
                }
            }
            Up::Fly {
                strength,
                duration,
                max_duration,
            } => {
                if *duration <= *max_duration && unit.get_effect_stats_mut().can_fly {
                    *duration += 1.0;
                    unit.get_speed_mut().y = -*strength;
                }
            }
            Up::Teleport {
                max_teleport,
                teleport_count,
                strength,
                can_teleport,
            } => {
                if *teleport_count <= *max_teleport
                    && *can_teleport
                    && unit.get_effect_stats_mut().can_teleport
                {
                    unit.get_position_mut().y -= *strength;
                    *teleport_count += 1;
                    unit.get_speed_mut().y = -5.;
                    *can_teleport = false;
                }
            }
        }
        self
    }

    pub fn release(
        mut self,
        unit: &mut dyn Unit,
        rl: &RaylibHandle,
        thread: &RaylibThread,
    ) -> Self {
        match &mut self {
            Up::Jump {
                strength,
                jump_count,
                max_jumps,
                can_jump,
            } => {
                *can_jump = true;
            }
            Up::Fly {
                strength,
                duration,
                max_duration,
            } => {}
            Up::Teleport {
                max_teleport,
                teleport_count,
                strength,
                can_teleport,
            } => {
                *can_teleport = true;
            }
        }
        self
    }

    pub fn down(mut self, unit: &mut dyn Unit, rl: &RaylibHandle, thread: &RaylibThread) -> Self {
        match &mut self {
            Up::Jump {
                strength,
                jump_count,
                max_jumps,
                can_jump: _,
            } => {
                unit.get_speed_mut().y += *strength * 0.2;
            }
            Up::Fly {
                strength,
                duration,
                max_duration,
            } => {}
            Up::Teleport {
                max_teleport,
                teleport_count,
                strength,
                can_teleport: _,
            } => {}
        }
        self
    }

    pub fn land(mut self) -> Self {
        match &mut self {
            Up::Jump {
                strength: _,
                jump_count,
                max_jumps: _,
                can_jump,
            } => {
                *jump_count = 0;
                *can_jump = true;
            }
            Up::Fly {
                strength,
                duration,
                max_duration,
            } => {
                *duration = 0.0;
            }
            Up::Teleport {
                max_teleport,
                teleport_count,
                strength,
                can_teleport,
            } => {
                *teleport_count = 0;
                *can_teleport = true;
            }
        }
        self
    }
}
