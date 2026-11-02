#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use core::fmt::Write;
use heapless::Vec;
use panic_rtt_target as _;
use serial_setup::UartePort;
use microbit::{
    hal::{Timer, twim, uarte}, pac::{comp::events_ready, twim0::frequency::FREQUENCY_A},
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

//把字节变成字符串 
use core::str::from_utf8;

#[entry]
fn main -> !{
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
//初始化串口
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

//初始化i2c
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer0 = Timer::new(board.TIMER0);

    // Code from documentation 初始化加速度计
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();



    loop {
        buffer.clear();

        loop {
            // We assume that the receiving cannot fail
            let byte = serial.read().unwrap();

            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
                break;
            }

            if byte == b'\r' {
                let line =&buffer[..buffer.len()-1];
                let text = core::str::from_utf8(line);
                match text {
                    "acc" => {
                        if sensor.accel_status().unwrap().xyz_new_data() {
                        let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
            // RTT instead of normal print
                        rprintln!("Acceleration: x {} y {} z {}", x, y, z);
                        serial.write((x,y,z)).unwrap();
                        }
                    }
                    "_" =>{ }
                }
            }
        }
        serial.flush().unwrap()
    }




}

