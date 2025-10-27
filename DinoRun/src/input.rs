use crate::constants::*;
use crate::particles::spawn_dust;
use crate::types::{GameState, PlayerAction, PowerUpKind};
use crate::world::World;
use macroquad::prelude::*;

/// 统一处理玩家输入，根据当前状态机执行操作或切换状态。
pub fn handle_input(world: &mut World, dt: f32) {
    if is_key_pressed(KeyCode::Escape) {
        match world.state {
            GameState::Running => {
                world.state = GameState::Paused;
                world.pause_flash = 1.0;
            }
            GameState::Paused => {
                world.state = GameState::Running;
            }
            GameState::Menu { .. } | GameState::Splash { .. } => {}
            GameState::GameOver { .. } => {}
        }
    }

    match world.state {
        GameState::Splash { .. } => {}
        GameState::Menu { .. } => {
            if is_key_pressed(KeyCode::Up) {
                world.menu_selected = world.menu_selected.saturating_sub(1);
            }
            if is_key_pressed(KeyCode::Down) {
                world.menu_selected = (world.menu_selected + 1).min(2);
            }
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                if world.menu_selected == 0 {
                    world.reset_run();
                    world.state = GameState::Running;
                } else if world.menu_selected == 1 {
                    world.power_up.activate(PowerUpKind::Shield);
                    world.power_up.timer = 0.1;
                } else {
                    world.reset_run();
                    world.best_score = 0.0;
                    world.currency = 0;
                }
            }
        }
        GameState::Running => {
            let ground = world.difficulty.ground_y();
            let mut dust_events: Vec<(Vec2, usize)> = Vec::new();
            {
                let p = &mut world.player;
                // 支持多键触发跳跃，方便玩家使用偏好键位。
                let jump_pressed = is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Up)
                    || is_key_pressed(KeyCode::W);

                if jump_pressed {
                    if p.on_ground(ground) {
                        p.vel.y = -820.0;
                        p.action = PlayerAction::Jump;
                        p.action_timer = 0.32;
                        p.can_double_jump = true;
                        let origin = p.pos + vec2(PLAYER_SIZE.x * 0.5, PLAYER_SIZE.y);
                        dust_events.push((origin, 8));
                    } else if p.can_double_jump {
                        p.vel.y = -780.0;
                        p.action = PlayerAction::DoubleJump;
                        p.action_timer = 0.28;
                        p.can_double_jump = false;
                        let origin = p.pos + vec2(PLAYER_SIZE.x * 0.5, PLAYER_SIZE.y * 0.4);
                        dust_events.push((origin, 6));
                    }
                }

                if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                    if p.stamina > 0.0 {
                        p.vel.x = DASH_SPEED;
                        p.stamina = (p.stamina - STAMINA_CONSUME_RATE * dt).max(0.0);
                        p.action = PlayerAction::Dash;
                        p.action_timer = 0.2;
                    }
                } else {
                    p.stamina = (p.stamina + STAMINA_RECOVER_RATE * dt).min(MAX_STAMINA);
                    p.vel.x = 0.0;
                }

                if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) && p.on_ground(ground) {
                    p.action = PlayerAction::Slide;
                    p.action_timer = 0.4;
                }
            }

            for (origin, count) in dust_events {
                spawn_dust(&mut world.particles, origin, count);
            }
        }
        GameState::Paused => {
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                world.state = GameState::Running;
            }
        }
        GameState::GameOver { cooldown } => {
            if cooldown <= 0.0 {
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    world.reset_run();
                    world.state = GameState::Running;
                }
                if is_key_pressed(KeyCode::Escape) {
                    world.state = GameState::Menu { fade: 0.0 };
                }
            }
        }
    }
}
