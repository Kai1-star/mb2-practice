// AI4MR demo — Step 4: RAW / ARROW 两模式 + 4 方向 + 死区 + 迟滞（MVP 完整版）
//   上电 = RAW（LED 全灭，打印 ax ay az）
//   按 A = RAW ↔ ARROW；按 B = 回 RAW
//
// 实测轴向（Step 1，屏幕朝你、USB 朝上看）：
//   +x → A 键（左）   +y → 金手指（下）   +z → 从背面穿出
// 和 Discovery MB2 第 13 章的 mb2-axes 图一致。
#![no_main]
#![no_std]
#![deny(unsafe_code)]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// ---------- 校准常量（由 Step 1 实测推出，Step 2 下坡测试验证） ----------
// east  = 下坡方向在屏幕「向右」上的分量 = X_SIGN * ax
// south = 下坡方向在屏幕「向下」上的分量 = Y_SIGN * ay
// 实测：抬高 A 边（下坡朝右）→ ax 变正，所以 X_SIGN = +1
//       抬高 USB 边（下坡朝下）→ ay 变负，所以 Y_SIGN = -1
const X_SIGN: i32 = 1;
const Y_SIGN: i32 = -1;
const SWAP_XY: bool = false;

const DEAD_ZONE_MG: i32 = 150; // ≈ 离水平 8.6°
const HYSTERESIS_FRAMES: u8 = 3; // 新方向连续 3 帧（300 ms）才切换
const LOOP_MS: u32 = 100;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    Flat,
    N,
    E,
    S,
    W,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(&mut timer, AccelMode::Normal, AccelOutputDataRate::Hz100)
        .unwrap();

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let (mut a_prev, mut b_prev) = (false, false);

    rprintln!("demo_ai4mr: RAW  (A = toggle ARROW, B = back to RAW)");

    let mut raw_mode = true; // 上电默认 RAW
    let mut last = (0i32, 0i32, 0i32);

    // 迟滞状态：stable = 正在显示的方向；candidate = 想上位的新方向；count = 它连续出现了几帧
    let mut stable = Dir::Flat;
    let mut candidate = Dir::Flat;
    let mut count = 0u8;

    loop {
        // 1) 读传感器；读失败就沿用上一帧，循环里不 panic
        let (ax, ay, az) = match sensor.acceleration() {
            Ok(data) => {
                last = data.xyz_mg();
                last
            }
            Err(_) => last,
        };

        // 2) 按键：边沿检测（这一帧按下 && 上一帧没按），按住不会连跳
        //    按键接地有效：按下 = low
        let a_now = button_a.is_low().unwrap();
        let b_now = button_b.is_low().unwrap();
        if a_now && !a_prev {
            raw_mode = !raw_mode;
            rprintln!("--- mode: {} ---", if raw_mode { "RAW" } else { "ARROW" });
        }
        if b_now && !b_prev && !raw_mode {
            raw_mode = true;
            rprintln!("--- mode: RAW ---");
        }
        a_prev = a_now;
        b_prev = b_now;

        // 3) 两个模式
        if raw_mode {
            display.clear();
            rprintln!("ax={:>5} ay={:>5} az={:>5}", ax, ay, az);
            timer.delay_ms(LOOP_MS);
        } else {
            let raw_dir = heading(ax, ay);

            // 迟滞：一帧的新方向不算数，要连续 HYSTERESIS_FRAMES 帧才换
            if raw_dir == stable {
                count = 0; // 回到原方向，挑战者作废
            } else if raw_dir == candidate {
                count += 1;
                if count >= HYSTERESIS_FRAMES {
                    stable = raw_dir;
                    count = 0;
                }
            } else {
                candidate = raw_dir; // 换了个新挑战者，从 1 开始数
                count = 1;
            }

            rprintln!(
                "dir={:?} (raw={:?}) ax={:>5} ay={:>5} az={:>5}",
                stable, raw_dir, ax, ay, az
            );
            // show() 本身阻塞 LOOP_MS 毫秒，顺便充当循环节拍
            display.show(&mut timer, pattern(stable), LOOP_MS);
        }
    }
}

/// 传感器读数 → 屏幕上的下坡方向
fn heading(ax: i32, ay: i32) -> Dir {
    // 1) 校准：从传感器系换到「屏幕系」（east 向右，south 向下）
    let (mut east, mut south) = (ax * X_SIGN, ay * Y_SIGN);
    if SWAP_XY {
        core::mem::swap(&mut east, &mut south);
    }

    // 2) 死区：平放时重力几乎全在 z 上，east/south 只剩噪声，方向无定义
    if east.abs() < DEAD_ZONE_MG && south.abs() < DEAD_ZONE_MG {
        return Dir::Flat;
    }

    // 3) 4 方向：哪个分量大就听谁的，符号定朝向（扇区边界在 45°）
    if east.abs() >= south.abs() {
        if east > 0 { Dir::E } else { Dir::W }
    } else if south > 0 {
        Dir::S
    } else {
        Dir::N
    }
}

/// 方向 → 5×5 点阵；leds[row][col]，row 0 = USB 边，col 0 = A 键边
fn pattern(dir: Dir) -> [[u8; 5]; 5] {
    let mut leds = [[0u8; 5]; 5];
    // 箭头 = 中心 (2,2) + 中点 + 头
    let head = match dir {
        Dir::N => Some((0usize, 2usize)),
        Dir::E => Some((2, 4)),
        Dir::S => Some((4, 2)),
        Dir::W => Some((2, 0)),
        Dir::Flat => None,
    };

    match head {
        Some((r, c)) => {
            leds[2][2] = 1;
            leds[(2 + r) / 2][(2 + c) / 2] = 1;
            leds[r][c] = 1;
        }
        None => {
            // 平放：3×3 空心方框
            for i in 1..=3 {
                leds[1][i] = 1;
                leds[3][i] = 1;
                leds[i][1] = 1;
                leds[i][3] = 1;
            }
        }
    }
    leds
}
