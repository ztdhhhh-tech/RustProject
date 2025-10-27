use crate::constants::*;
use crate::types::PlayerAction;
use macroquad::prelude::*;

/// 玩家实体包含物理状态、动作状态机与体力信息。
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub action: PlayerAction,
    pub action_timer: f32,
    pub stamina: f32,
    pub can_double_jump: bool,
    pub combo: u32,
    pub dash_cooldown: f32,
}

impl Player {
    /// 返回一个处于初始位置的玩家，供新开局与重置使用。
    pub fn new() -> Self {
        Self {
            pos: PLAYER_START,
            vel: Vec2::ZERO,
            action: PlayerAction::Running,
            action_timer: 0.0,
            stamina: MAX_STAMINA,
            can_double_jump: true,
            combo: 0,
            dash_cooldown: 0.0,
        }
    }

    /// 计算当前动作下的碰撞盒（滑铲时高度减小）。
    pub fn rect(&self) -> Rect {
        let mut offset = 0.0;
        if self.action == PlayerAction::Slide {
            offset = PLAYER_SIZE.y * 0.35;
        }
        Rect::new(
            self.pos.x,
            self.pos.y + offset,
            PLAYER_SIZE.x,
            PLAYER_SIZE.y - offset,
        )
    }

    /// 判断玩家底部是否接触地面，用于跳跃与滑铲判定。
    pub fn on_ground(&self, ground: f32) -> bool {
        (self.pos.y + PLAYER_SIZE.y - ground).abs() < 0.5
    }

    /// 增加连击计数，封顶以避免 UI 溢出。
    pub fn add_combo(&mut self) {
        self.combo = (self.combo + 1).min(999);
    }

    /// 中断连击链路。
    pub fn reset_combo(&mut self) {
        self.combo = 0;
    }
}

/// 推进玩家物理状态与动作状态机。
pub fn update_player(player: &mut Player, dt: f32, ground: f32) {
    player.action_timer = (player.action_timer - dt).max(0.0);
    player.vel.y += GRAVITY * dt;
    player.vel.y = player.vel.y.min(TERMINAL_VEL);
    player.pos += vec2(player.vel.x * dt, player.vel.y * dt);

    if player.pos.y + PLAYER_SIZE.y >= ground {
        player.pos.y = ground - PLAYER_SIZE.y;
        player.vel.y = 0.0;
        player.can_double_jump = true;
        if player.action != PlayerAction::Dash {
            player.action = PlayerAction::Running;
        }
    }

    if player.pos.y + PLAYER_SIZE.y < ground - 4.0 && player.action == PlayerAction::Running {
        player.action = PlayerAction::Jump;
    }

    if player.action_timer == 0.0
        && player.action != PlayerAction::Running
        && player.on_ground(ground)
    {
        player.action = PlayerAction::Running;
    }

    if player.dash_cooldown > 0.0 {
        player.dash_cooldown -= dt;
    }
}

/// 根据玩家当前状态绘制贴图。
pub fn draw_player(player: &Player, texture: &Texture2D) {
    draw_texture_ex(
        texture,
        player.pos.x,
        player.pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(PLAYER_SIZE),
            ..Default::default()
        },
    );
}
