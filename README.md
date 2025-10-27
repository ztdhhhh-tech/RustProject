# Dino Run

一款使用 [macroquad](https://github.com/not-fl3/macroquad) 构建的快节奏横版无尽奔跑游戏。玩家可以跳跃、二段跳与冲刺，在动态难度、昼夜循环与粒子特效中争取更长期的存活距离。

## 运行环境
- Rust 1.78 或更新版本（支持 2024 Edition）
- 已安装 `cargo` 构建工具
- Windows、macOS 或 Linux 均可运行（本项目主要在 Windows 下开发）

## 快速开始
```bash
# 克隆仓库后进入目录
cargo run --release
```
> `--release` 可以开启优化，获得更流畅的帧率。开发调试时也可以直接执行 `cargo run`。

## 游戏操作
| 动作 | 键位 |
| --- | --- |
| 跳跃 | `Space` / `Up` / `W` |
| 二段跳 | 空中再次按下跳跃键 |
| 冲刺 | 按住 `Left Shift` / `Right Shift` |
| 暂停 / 恢复 | `Escape` / `Space` |
| 菜单导航 | `Up` / `Down` 选择，`Enter` / `Space` 确认 |

## 核心特性
- **动态难度曲线**：随生存时间逐渐提升滚屏速度与障碍密度。
- **多样障碍**：木箱、圆锯、深坑与空中无人机，需要灵活运用技能应对。
- **收集与连击**：金币与宝石可提升分数并累积连击倍率，获取游戏内货币。
- **能量与体力管理**：冲刺会消耗体力，需要在进攻与恢复之间平衡。
- **随机能力加成**：护盾、防御、时间减速等强化在关键时刻帮助延续奔跑。
- **视觉表现**：昼夜循环、视差背景与粒子特效营造霓虹赛博氛围。

## 项目结构
```
src/
  main.rs          # 程序入口，负责游戏主循环
  constants.rs     # 全局常量、屏幕与物理参数
  world.rs         # 世界状态、难度进程、复位逻辑
  input.rs         # 键盘输入与状态机切换
  update.rs        # 游戏状态更新、碰撞判定与得分
  render.rs        # 场景渲染、UI 与特效
  player.rs        # 玩家角色数据与物理行为
  obstacles.rs     # 障碍生成与渲染
  collectibles.rs  # 可收集物与浮动动画
  particles.rs     # 粒子系统、提示文字
  utils.rs         # 通用工具、插值与绘制辅助
assets/
  player.png       # 玩家贴图（像素风格）
Cargo.toml         # Rust 包配置，依赖 macroquad 0.4
```
