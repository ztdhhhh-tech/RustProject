use crate::collectibles::Collectible;
use crate::constants::*;
use crate::obstacles::Obstacle;
use crate::particles::Particle;
use crate::player::Player;
use crate::types::*;
use macroquad::{prelude::*, rand::gen_range};

/// 表示一次强化效果的运行时状态。
pub struct PowerUp {
    pub active: bool,
    pub kind: PowerUpKind,
    pub timer: f32,
}

impl Default for PowerUp {
    fn default() -> Self {
        Self {
            active: false,
            kind: PowerUpKind::Shield,
            timer: 0.0,
        }
    }
}

impl PowerUp {
    /// 激活指定类型的强化效果并重置倒计时。
    pub fn activate(&mut self, kind: PowerUpKind) {
        self.active = true;
        self.kind = kind;
        self.timer = POWERUP_DURATION;
    }

    /// 关闭强化效果并清零计时。
    pub fn deactivate(&mut self) {
        self.active = false;
        self.timer = 0.0;
    }

    /// 根据帧间隔推进计时器，自动在归零时失效。
    pub fn update(&mut self, dt: f32) {
        if self.active {
            self.timer -= dt;
            if self.timer <= 0.0 {
                self.deactivate();
            }
        }
    }
}

/// 统一管理玩家实体、场景元素以及游戏状态机的根容器。
pub struct World {
    pub player: Player,
    pub player_texture: Texture2D,
    pub particles: Vec<Particle>,
    pub parallax: Vec<ParallaxLayer>,
    pub obstacles: Vec<Obstacle>,
    pub collectibles: Vec<Collectible>,
    pub floating_texts: Vec<FloatingText>,
    pub power_up: PowerUp,
    pub state: GameState,
    pub difficulty: DifficultyTrack,
    pub menu_selected: usize,
    pub score: f32,
    pub best_score: f32,
    pub currency: u32,
    pub survival_time: f32,
    pub slow_mo_factor: f32,
    pub streak_mult: f32,
    pub day_phase: f32,
    pub pause_flash: f32,
}

impl World {
    /// 游戏全局状态的聚合体，贯穿输入、更新与渲染。
    pub fn new(player_texture: Texture2D) -> Self {
        Self {
            player: Player::new(),
            player_texture,
            particles: vec![Particle::default(); MAX_PARTICLES],
            parallax: vec![
                ParallaxLayer {
                    height: BASE_GROUND_Y + 90.0,
                    speed: 32.0,
                    color: Color::from_rgba(44, 62, 105, 255),
                    thickness: 120.0,
                },
                ParallaxLayer {
                    height: BASE_GROUND_Y + 60.0,
                    speed: 48.0,
                    color: Color::from_rgba(63, 83, 141, 255),
                    thickness: 76.0,
                },
                ParallaxLayer {
                    height: BASE_GROUND_Y + 32.0,
                    speed: 68.0,
                    color: Color::from_rgba(84, 112, 174, 255),
                    thickness: 48.0,
                },
            ],
            obstacles: Vec::new(),
            collectibles: Vec::new(),
            floating_texts: Vec::new(),
            power_up: PowerUp::default(),
            state: GameState::Splash { timer: 0.0 },
            difficulty: DifficultyTrack::new(),
            menu_selected: 0,
            score: 0.0,
            best_score: 0.0,
            currency: 0,
            survival_time: 0.0,
            slow_mo_factor: 1.0,
            streak_mult: 1.0,
            day_phase: 0.0,
            pause_flash: 0.0,
        }
    }

    /// 恢复到初始状态，用于开始新一轮奔跑。
    pub fn reset_run(&mut self) {
        self.player = Player::new();
        self.obstacles.clear();
        self.collectibles.clear();
        self.floating_texts.clear();
        self.power_up.deactivate();
        self.difficulty = DifficultyTrack::new();
        self.score = 0.0;
        self.survival_time = 0.0;
        self.slow_mo_factor = 1.0;
        self.streak_mult = 1.0;
        self.day_phase = gen_range(0.0, 1.0);
    }
}
