use crate::collectibles::draw_collectibles;
use crate::constants::*;
use crate::obstacles::draw_obstacles;
use crate::particles::{draw_floating_texts, draw_particles};
use crate::player::draw_player;
use crate::types::{ColorLerp, GameState, PowerUpKind};
use crate::utils::{draw_text_centered, ease_out_quad};
use crate::world::World;
use macroquad::prelude::*;

// 渲染完整世界：依次绘制背景、实体、特效与 UI。
pub fn draw_world(world: &World) {
    clear_background(day_night_color(world.day_phase));
    draw_parallax(world);
    draw_ground(world);
    draw_player(&world.player, &world.player_texture);

    // Draw shield effect around player if active
    if world.power_up.active && world.power_up.kind == PowerUpKind::Shield {
        let rect = world.player.rect();
        let pad = (get_time() as f32).sin() * 4.0 + 12.0;
        draw_rectangle_lines(
            rect.x - pad * 0.5,
            rect.y - pad * 0.5,
            rect.w + pad,
            rect.h + pad,
            4.0,
            Color::from_rgba(100, 200, 255, 200),
        );
    }

    draw_obstacles(&world.obstacles);
    draw_collectibles(&world.collectibles);
    draw_particles(&world.particles);
    draw_ui(world);
}
// 根据昼夜相位返回背景颜色。
fn day_night_color(t: f32) -> Color {
    let palette = [
        ColorLerp {
            start: Color::from_rgba(32, 39, 73, 255),
            end: Color::from_rgba(70, 63, 112, 255),
        },
        ColorLerp {
            start: Color::from_rgba(70, 63, 112, 255),
            end: Color::from_rgba(158, 121, 101, 255),
        },
        ColorLerp {
            start: Color::from_rgba(158, 121, 101, 255),
            end: Color::from_rgba(44, 59, 101, 255),
        },
        ColorLerp {
            start: Color::from_rgba(44, 59, 101, 255),
            end: Color::from_rgba(32, 39, 73, 255),
        },
    ];
    let idx = (t * palette.len() as f32) as usize % palette.len();
    let next = (idx + 1) % palette.len();
    let segment_t = (t * palette.len() as f32) % 1.0;
    let lerp = ColorLerp {
        start: palette[idx].end,
        end: palette[next].end,
    };
    lerp.sample(segment_t)
}

// 绘制视差背景层，强化速度感。
fn draw_parallax(world: &World) {
    for layer in &world.parallax {
        let speed = layer.speed * world.difficulty.scroll_speed() / BASE_SCROLL_SPEED;
        let offset = (get_time() as f32 * speed) % SCREEN_WIDTH;
        for i in -1..=2 {
            draw_rectangle(
                i as f32 * SCREEN_WIDTH - offset,
                layer.height,
                SCREEN_WIDTH,
                layer.thickness,
                Color::new(layer.color.r, layer.color.g, layer.color.b, 0.65),
            );
        }
    }
}

// 绘制地面与高光。
fn draw_ground(world: &World) {
    let ground_y = world.difficulty.ground_y();
    draw_rectangle(
        0.0,
        ground_y,
        SCREEN_WIDTH,
        SCREEN_HEIGHT - ground_y,
        Color::from_rgba(44, 120, 68, 255),
    );
    draw_line(
        0.0,
        ground_y,
        SCREEN_WIDTH,
        ground_y,
        5.0,
        Color::from_rgba(26, 82, 45, 255),
    );
    let highlight = Rect::new(0.0, ground_y - 8.0, SCREEN_WIDTH, 6.0);
    draw_rectangle(
        highlight.x,
        highlight.y,
        highlight.w,
        highlight.h,
        Color::from_rgba(158, 231, 139, 120),
    );
}

// 根据 GameState 切换不同 UI 场景。
fn draw_ui(world: &World) {
    match world.state {
        GameState::Splash { timer } => draw_splash(timer),
        GameState::Menu { fade } => draw_menu(world, fade),
        GameState::Running => draw_hud(world, 1.0),
        GameState::Paused => {
            draw_hud(world, 0.4);
            draw_pause(world.pause_flash);
        }
        GameState::GameOver { cooldown } => {
            draw_hud(world, 0.4);
            draw_game_over(world, cooldown);
        }
    }
    draw_floating_texts(&world.floating_texts);
}

// 闪屏界面的淡入文案。
fn draw_splash(timer: f32) {
    let t = (timer / 2.0).min(1.0);
    let alpha = (ease_out_quad(t) * 255.0) as u8;
    draw_text_centered(
        "DINO RUN",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.46,
        72.0,
        Color::from_rgba(255, 255, 255, alpha),
    );
    let sub_alpha = (ease_out_quad((timer - 0.8).max(0.0) / 1.2) * 255.0) as u8;
    draw_text_centered(
        "Press Space",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.6,
        36.0,
        Color::from_rgba(180, 220, 255, sub_alpha),
    );
}

// 主菜单选项与提示文字。
fn draw_menu(world: &World, fade: f32) {
    let alpha = (fade * 255.0) as u8;
    draw_text_centered(
        "DINO RUN",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.3,
        72.0,
        Color::from_rgba(255, 255, 255, alpha),
    );
    let options = ["Start Run", "Instant Shield", "Reset Progress"];
    for (idx, option) in options.iter().enumerate() {
        let y = SCREEN_HEIGHT * 0.45 + idx as f32 * 46.0;
        let selected = idx == world.menu_selected;
        let color = if selected {
            Color::from_rgba(255, 200, 90, alpha)
        } else {
            Color::from_rgba(200, 200, 210, alpha)
        };
        draw_text_centered(option, SCREEN_WIDTH * 0.5, y, 36.0, color);
    }
    draw_text_centered(
        &format!("Best Distance: {:0.0}m", world.best_score),
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.7,
        28.0,
        Color::from_rgba(200, 220, 255, alpha),
    );
}

// 游戏 HUD，显示分数、连击与体力条。
fn draw_hud(world: &World, alpha: f32) {
    let panel_color = Color::from_rgba(20, 36, 58, (alpha * 170.0) as u8);
    draw_rectangle(24.0, 24.0, 320.0, 148.0, panel_color);
    draw_text(
        &format!("Distance: {:0.0}m", world.score),
        36.0,
        66.0,
        32.0,
        WHITE,
    );
    draw_text(
        &format!("Best: {:0.0}m", world.best_score),
        36.0,
        102.0,
        28.0,
        LIGHTGRAY,
    );
    draw_text(
        &format!("Combo x{:0.2}", world.streak_mult),
        36.0,
        138.0,
        24.0,
        Color::from_rgba(255, 210, 110, 255),
    );
    draw_text(
        &format!("Currency: {}", world.currency),
        36.0,
        174.0,
        24.0,
        Color::from_rgba(140, 210, 255, 255),
    );

    let stamina_ratio = world.player.stamina / MAX_STAMINA;
    draw_bar(
        Vec2::new(28.0, 196.0),
        Vec2::new(312.0, 12.0),
        stamina_ratio,
        Color::from_rgba(80, 200, 255, 255),
        alpha,
    );

    if world.power_up.active {
        let text = match world.power_up.kind {
            PowerUpKind::Shield => "Shield",
            PowerUpKind::ScoreBoost => "Score Boost",
            PowerUpKind::TimeSlow => "Time Slow",
        };
        let ratio = (world.power_up.timer / POWERUP_DURATION).clamp(0.0, 1.0);
        draw_bar(
            Vec2::new(SCREEN_WIDTH - 360.0, 36.0),
            Vec2::new(320.0, 16.0),
            ratio,
            Color::from_rgba(255, 200, 120, 255),
            alpha,
        );
        draw_text(text, SCREEN_WIDTH - 348.0, 66.0, 28.0, WHITE);
    }
}

// 通用进度条绘制函数。
fn draw_bar(pos: Vec2, size: Vec2, ratio: f32, fill: Color, alpha: f32) {
    draw_rectangle(
        pos.x,
        pos.y,
        size.x,
        size.y,
        Color::from_rgba(0, 0, 0, (alpha * 90.0) as u8),
    );
    draw_rectangle(
        pos.x,
        pos.y,
        size.x * ratio,
        size.y,
        Color::new(fill.r, fill.g, fill.b, alpha),
    );
    draw_rectangle_lines(
        pos.x,
        pos.y,
        size.x,
        size.y,
        2.0,
        Color::from_rgba(0, 0, 0, (alpha * 255.0) as u8),
    );
}

// 暂停提示文本。
fn draw_pause(flash: f32) {
    let alpha = (flash * 220.0) as u8;
    draw_text_centered(
        "PAUSED",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.45,
        64.0,
        Color::from_rgba(255, 255, 255, alpha),
    );
    draw_text_centered(
        "Press Space to resume",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.55,
        32.0,
        Color::from_rgba(220, 220, 220, alpha),
    );
}

// 游戏结束界面展示成绩与提示。
fn draw_game_over(world: &World, cooldown: f32) {
    let alpha = ((1.0 - cooldown) * 255.0) as u8;
    draw_text_centered(
        "RUN OVER",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.4,
        64.0,
        Color::from_rgba(255, 160, 160, alpha),
    );
    draw_text_centered(
        &format!("Distance: {:0.0}m", world.score),
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.5,
        32.0,
        Color::from_rgba(255, 255, 255, alpha),
    );
    draw_text_centered(
        &format!("Currency gained: {}", world.currency),
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.56,
        28.0,
        Color::from_rgba(200, 220, 255, alpha),
    );
    draw_text_centered(
        "Press Space to retry",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.64,
        32.0,
        Color::from_rgba(255, 255, 255, alpha),
    );
    draw_text_centered(
        "Press Esc for menu",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.7,
        24.0,
        Color::from_rgba(200, 200, 200, alpha),
    );
}
