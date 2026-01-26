#![no_main]
#![no_std]

mod life;
use life::*;
use panic_rtt_target as _;
use embedded_hal as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use microbit::board::Board;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut counter = 0u64;
    let mut last_tick = timer.now().ticks() / 16_000;
    let mut timer = Timer::new(board.TIMER0);
    loop {
        let current_tick = timer.now() / 16_000;
        if current_tick.wrapping_sub(last_tick) >= 100 {
            last_tick = current_tick;

            //Update frame counter
            counter += 1;
            rprintln!("Frame: {}", counter);

            //Update game state
            life(&mut fb);
        }
        

    }
}
