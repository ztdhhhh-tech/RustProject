use crate::collectibles::Collectible;
use crate::constants::*;
use crate::obstacles::{Obstacle, choose_obstacle_kind};
use crate::particles::{
    spawn_collect_effect, spawn_death_effect, spawn_shield_burst, update_floating_texts,
    update_particles,
};
use crate::player::update_player;
use crate::types::{CollectibleKind, GameState, PowerUpKind};
use crate::utils::RectExt;
use crate::world::World;
use macroquad::{prelude::*, rand::gen_range};

/// 根据当前 GameState 推进世界状态，是游戏逻辑的核心调度函数。
pub fn update_world(world: &mut World, dt: f32) {
    match world.state {
        GameState::Splash { ref mut timer } => {
            *timer += dt;
            if *timer > 2.6 {
                world.state = GameState::Menu { fade: 0.0 };
            }
        }
        GameState::Menu { ref mut fade } => {
            *fade = (*fade + dt / MENU_FADE_TIME).min(1.0);
            world.day_phase = (world.day_phase + dt / DAY_NIGHT_DURATION) % 1.0;
            update_particles(&mut world.particles, dt, BASE_SCROLL_SPEED * 0.2);
        }
        GameState::Running => {
            // TimeSlow 强化会缩放时间，影响所有更新逻辑。
            let time_scale =
                if world.power_up.active && world.power_up.kind == PowerUpKind::TimeSlow {
                    0.6
                } else {
                    1.0
                };
            let scaled_dt = dt * time_scale;
            world.difficulty.time += scaled_dt;
            world.survival_time += scaled_dt;
            world.day_phase = (world.day_phase + scaled_dt / DAY_NIGHT_DURATION) % 1.0;
            world.power_up.update(scaled_dt);
            world.slow_mo_factor =
                if world.power_up.active && world.power_up.kind == PowerUpKind::TimeSlow {
                    0.6
                } else {
                    1.0
                };

            // 逐个子系统更新：玩家、障碍、收集物、碰撞与粒子。
            let ground = world.difficulty.ground_y();
            update_player(&mut world.player, scaled_dt, ground);
            update_obstacles(world, scaled_dt, ground);
            update_collectibles(world, scaled_dt, ground);
            handle_collisions(world, ground);
            update_particles(
                &mut world.particles,
                scaled_dt,
                world.difficulty.scroll_speed(),
            );
            update_floating_texts(&mut world.floating_texts, scaled_dt);
            world.score += (world.difficulty.scroll_speed() * scaled_dt * world.streak_mult * 0.05)
                + (world.player.vel.x * scaled_dt * 0.07);
            world.best_score = world.best_score.max(world.score);
        }
        GameState::Paused => {
            world.pause_flash = (world.pause_flash - dt).max(0.0);
        }
        GameState::GameOver { ref mut cooldown } => {
            *cooldown = (*cooldown - dt).max(0.0);
            update_particles(&mut world.particles, dt * 0.75, BASE_SCROLL_SPEED * 0.5);
            update_floating_texts(&mut world.floating_texts, dt);
        }
    }
}

/// 控制障碍刷新的随机节奏，并更新已有障碍的位置。
fn update_obstacles(world: &mut World, dt: f32, ground: f32) {
    let speed = world.difficulty.scroll_speed();
    world.difficulty.rng_obstacle_timer -= dt;
    if world.difficulty.rng_obstacle_timer <= 0.0 {
        let kind = choose_obstacle_kind(world.difficulty.time);
        world.obstacles.push(Obstacle::new(kind, ground));
        let interval = world.difficulty.obstacle_interval() + gen_range(-0.22, 0.28);
        world.difficulty.rng_obstacle_timer = interval.max(0.35);
    }
    for obstacle in &mut world.obstacles {
        obstacle.update(dt, speed * world.slow_mo_factor);
    }
    world.obstacles.retain(|o| !o.is_offscreen());
}

/// 刷新收集物并推进其动画。
fn update_collectibles(world: &mut World, dt: f32, ground: f32) {
    let speed = world.difficulty.scroll_speed();
    world.difficulty.rng_collectible_timer -= dt;
    if world.difficulty.rng_collectible_timer <= 0.0 {
        let kind = if gen_range(0.0, 1.0) > 0.8 {
            CollectibleKind::Gem
        } else {
            CollectibleKind::Coin
        };
        world.collectibles.push(Collectible::new(kind, ground));
        let interval = world.difficulty.collectible_interval() + gen_range(-0.3, 0.5);
        world.difficulty.rng_collectible_timer = interval.max(0.24);
    }
    for item in &mut world.collectibles {
        item.update(dt, speed * world.slow_mo_factor);
    }
    world.collectibles.retain(|c| !c.is_offscreen());
}

/// 统一处理玩家与障碍、收集物和强化之间的交互。
fn handle_collisions(world: &mut World, ground: f32) {
    let player_rect = world.player.rect();
    let shielded = world.power_up.active && world.power_up.kind == PowerUpKind::Shield;
    let mut dead = false;
    let mut shield_hit_info: Option<(usize, Rect)> = None;

    for (i, obstacle) in world.obstacles.iter().enumerate() {
        if obstacle.kind == crate::types::ObstacleKind::Pit {
            if player_rect.x + player_rect.w > obstacle.rect.x
                && player_rect.x < obstacle.rect.x + obstacle.rect.w
                && world.player.pos.y + PLAYER_SIZE.y >= ground - 4.0
            {
                dead = true;
                break;
            }
        } else if <Rect as RectExt>::overlaps(&obstacle.hurt_box, &player_rect) {
            if shielded {
                shield_hit_info = Some((i, obstacle.rect));
                break;
            } else {
                dead = true;
                break;
            }
        }
    }

    if let Some((idx, rect)) = shield_hit_info {
        world.obstacles.remove(idx);
        spawn_shield_burst(&mut world.particles, rect);
        world.power_up.deactivate();
        world.player.reset_combo();
    }

    if dead {
        world.state = GameState::GameOver { cooldown: 0.8 };
        world.currency += (world.score as u32 / 10) + world.player.combo;
        spawn_death_effect(
            &mut world.particles,
            &mut world.floating_texts,
            world.player.pos + PLAYER_SIZE * 0.5,
        );
        world.player.reset_combo();
        return;
    }

    let mut collected = Vec::new();
    for (idx, item) in world.collectibles.iter().enumerate() {
        if <Rect as RectExt>::overlaps(&item.rect, &player_rect) {
            collected.push(idx);
        }
    }
    collected.sort_by(|a, b| b.cmp(a));
    for idx in collected.iter().copied() {
        let item = world.collectibles.remove(idx);
        world.score += item.value as f32 * world.streak_mult;
        world.currency += item.value / 4;
        world.player.add_combo();
        world.streak_mult = (world.streak_mult + 0.08).min(3.0);
        spawn_collect_effect(&mut world.particles, &mut world.floating_texts, item);
    }

    if collected.is_empty() {
        world.streak_mult = (world.streak_mult - 0.012).max(1.0);
    }

    if gen_range(0.0, 1.0) > 0.996 && !world.power_up.active {
        let kind = match gen_range(0.0, 1.0) {
            v if v < 0.4 => PowerUpKind::Shield,
            v if v < 0.75 => PowerUpKind::ScoreBoost,
            _ => PowerUpKind::TimeSlow,
        };
        world.power_up.activate(kind);
        let label = match kind {
            PowerUpKind::Shield => "Shield!",
            PowerUpKind::ScoreBoost => "Score Boost!",
            PowerUpKind::TimeSlow => "Slow Time!",
        };
        world.floating_texts.push(crate::types::FloatingText {
            text: label.to_string(),
            pos: world.player.pos + vec2(PLAYER_SIZE.x * 0.5, -24.0),
            vel: vec2(0.0, -42.0),
            timer: 1.2,
            max_timer: 1.2,
            color: GOLD,
        });
    }
}
