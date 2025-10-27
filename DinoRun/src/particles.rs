use crate::collectibles::Collectible;
use crate::constants::*;
use crate::types::{CollectibleKind, FloatingText};
use macroquad::{prelude::*, rand::gen_range};

/// 粒子对象用于表现尘土、爆裂等瞬时特效。
#[derive(Clone, Default)]
pub struct Particle {
    pub active: bool,
    pub pos: Vec2,
    pub vel: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color: Color,
}

/// 在玩家踩地时生成尘土特效。
pub fn spawn_dust(particles: &mut [Particle], origin: Vec2, count: usize) {
    for particle in particles.iter_mut().filter(|p| !p.active).take(count) {
        particle.active = true;
        particle.pos = origin;
        particle.vel = vec2(gen_range(-90.0, 90.0), gen_range(-240.0, -120.0));
        particle.color = Color::from_rgba(222, 205, 162, 255);
        // 使用随机寿命让尘土在短时间内自然散去。
        particle.max_lifetime = gen_range(0.35, 0.6);
        particle.lifetime = particle.max_lifetime;
    }
}

/// 护盾抵挡伤害时触发的爆裂特效。
pub fn spawn_shield_burst(particles: &mut [Particle], rect: Rect) {
    for particle in particles.iter_mut().filter(|p| !p.active).take(32) {
        particle.active = true;
        particle.pos = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        particle.vel = vec2(gen_range(-260.0, 260.0), gen_range(-260.0, 260.0));
        particle.color = Color::from_rgba(120, 200, 255, 255);
        // 护盾爆裂粒子寿命更长，强调反馈。
        particle.max_lifetime = gen_range(0.4, 0.8);
        particle.lifetime = particle.max_lifetime;
    }
}

/// 玩家失败时的爆散特效与提示文字。
pub fn spawn_death_effect(
    particles: &mut [Particle],
    floating_texts: &mut Vec<FloatingText>,
    pos: Vec2,
) {
    for particle in particles.iter_mut() {
        if !particle.active {
            particle.active = true;
            particle.pos = pos;
            particle.vel = vec2(gen_range(-380.0, 380.0), gen_range(-380.0, 120.0));
            // 死亡特效采用红色并延长寿命，使其更显著。
            particle.color = RED;
            particle.max_lifetime = gen_range(0.7, 1.1);
            particle.lifetime = particle.max_lifetime;
        }
    }
    floating_texts.push(FloatingText {
        text: "Run Lost".to_string(),
        pos,
        vel: vec2(0.0, -28.0),
        timer: 1.6,
        max_timer: 1.6,
        color: WHITE,
    });
}

/// 收集物被拾取时的闪光与提示文字。
pub fn spawn_collect_effect(
    particles: &mut [Particle],
    floating_texts: &mut Vec<FloatingText>,
    item: Collectible,
) {
    for particle in particles.iter_mut().filter(|p| !p.active).take(14) {
        particle.active = true;
        particle.pos = vec2(item.rect.x + item.rect.w * 0.5, item.rect.y);
        particle.vel = vec2(gen_range(-110.0, 110.0), gen_range(-220.0, -60.0));
        particle.color = match item.kind {
            CollectibleKind::Coin => Color::from_rgba(255, 215, 0, 255),
            CollectibleKind::Gem => Color::from_rgba(80, 200, 255, 255),
        };
        // 不同寿命营造拾取闪光的层次感。
        particle.max_lifetime = gen_range(0.4, 0.8);
        particle.lifetime = particle.max_lifetime;
    }
    let label = match item.kind {
        CollectibleKind::Coin => format!("+{}", item.value),
        CollectibleKind::Gem => format!("+{} Combo!", item.value),
    };
    floating_texts.push(FloatingText {
        text: label,
        pos: vec2(item.rect.x, item.rect.y),
        vel: vec2(0.0, -40.0),
        timer: 1.0,
        max_timer: 1.0,
        color: Color::from_rgba(255, 255, 255, 255),
    });
}

/// 推进所有粒子的生命周期与速度，并考虑场景卷轴影响。
pub fn update_particles(particles: &mut [Particle], dt: f32, scroll_speed: f32) {
    for particle in particles {
        if !particle.active {
            continue;
        }
        // 衰减寿命并受水平卷轴与重力影响。
        particle.lifetime -= dt;
        if particle.lifetime <= 0.0 {
            particle.active = false;
            continue;
        }
        particle.pos += particle.vel * dt;
        particle.vel.x -= scroll_speed * 0.32 * dt;
        particle.vel.y += GRAVITY * 0.26 * dt;
    }
}

/// 按透明度绘制粒子圆形。
pub fn draw_particles(particles: &[Particle]) {
    for particle in particles {
        if !particle.active {
            continue;
        }
        // 使用剩余寿命作为透明度，增强消散感。
        let alpha = (particle.lifetime / particle.max_lifetime).clamp(0.0, 1.0);
        draw_circle(
            particle.pos.x,
            particle.pos.y,
            6.0,
            Color::new(particle.color.r, particle.color.g, particle.color.b, alpha),
        );
    }
}

/// 更新浮动文本的位置与寿命。
pub fn update_floating_texts(floating_texts: &mut Vec<FloatingText>, dt: f32) {
    for text in floating_texts.iter_mut() {
        text.pos += text.vel * dt;
        text.timer -= dt;
    }
    floating_texts.retain(|text| text.timer > 0.0);
}

/// 渲染浮动文本，随时间淡出。
pub fn draw_floating_texts(floating_texts: &[FloatingText]) {
    for text in floating_texts {
        let ratio = (text.timer / text.max_timer).clamp(0.0, 1.0);
        draw_text(
            &text.text,
            text.pos.x,
            text.pos.y,
            28.0,
            Color::new(text.color.r, text.color.g, text.color.b, ratio),
        );
    }
}
