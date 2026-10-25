#![no_main]
#![no_std]


use cortex_m_rt::entry;          // 程序入口：第 5 章 GDB 里看到的 #[entry] 就是它
use panic_rtt_target as _;       // 程序 panic 时怎么办：把错误信息通过 RTT 打印出来
use rtt_target::rtt_init_print;  // 初始化 RTT，之后才能用 rprintln! 往电脑打印
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display :: blocking ::Display, hal::Timer, pac::usbd::tasks_startepin::TASKS_STARTEPIN_SPEC};


#[entry]
fn main ()->!{
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
      let mut display = Display::new(board.display_pins);
    let mut light_it_all =[
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
    ];
    let steps:[(usize,usize);16] =[(0,0),
    (0,1),
    (0,2),
    (0,3),
    (0,4),
    (1,4),
    (2,4),
    (3,4),
    (4,4),
    (4,3),
    (4,2),
    (4,1),
    (4,0),
    (3,0),
    (2,0),
    (1,0),
    ] ;
    
    loop{
        for (x,y) in steps {

            light_it_all[x][y]=1;
            display.show(&mut timer, light_it_all,100);
            display.clear();

            light_it_all[x][y]=0;  
        }
    }

}