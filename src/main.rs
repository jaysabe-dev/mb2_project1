#![no_main]
#![no_std]

use panic_rtt_target as _;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use embedded_hal::{digital::InputPin, delay::DelayNs};
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer, Rng as HwRng}
};
use nanorand::{Rng, SeedableRng};

// imported game logic 
mod life;
use life::*;

//remember this is a single core operation

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    
    // Starts with a random board layout + random starting point for the life game (in life.rs)
    let mut hw_rng = HwRng::new(board.RNG);
    let mut seed_buffer = [0u8; 8];
    hw_rng.read(&mut seed_buffer);
    let seeed = u64::from_le_bytes(seed_buffer);
    let mut rng = nanorand::Pcg64::new_seed(seed);

    //init 5x5 2d array with microbit::hal::Rng as frame_buffer 
    let mut fb = [[0u8; 5]; 5];
    
    randomize_board(&mut fb, &mut rng);
   
    // State counters for B button states
    let mut b_ignore_timer = 0;
    let mut end_timer = 0;

    // While "A" button is held down, 
    // a) the board is re-randomized every frame
    // b) otherwise, when the B button is not ignored and is pressed, the board is
    // "complemented": every "on" cell is turned "off" and every "off" cell is turned
    // "on".

    // Then: The B button is then ignored for 5 frames (0.5s) 

    //ELSE : Otherwise, if the program reaches a state where all cells on the board are off,
    // the program waits 5 frames (0.5s). 
    
    // If it has not received a button press, it then starts with a new random board
    
    loop {
        // -------------  Input
        let is_a_pressed = board.button_a.is_low().unwrap();
        let is_b_pressed = board.button_b.is_low().unwrap();
        let mut button_action_taken = false;

        // Rule: While A is held, re-randomize every frame
        if is_a_pressed {
            randomize_baord(&mut fb, &mut rng);
            button_action_taken = true;
            end_timer = 0; // reset if user interacts
        }
        // Rule: If B is pressed (and not ignored), complement the board
        else if is_b_pressed && b_ignore_timer == 0 {
            complement_board(&mut fb);
            b_ignore_timer = 5;
            button_action_taken = true;
            end_timer = 0;
        }
        

        // GAME MAIN LOGIC
        

        timer.delay_ms(100u32);         // Program at 100 ms intervals (10 frames per second)
    }
}

// helper to seed board with a random starting state
fn randomize_board(fb: &mut [[u8; 5]; 5], rng: &mut nanorand::Pcg64) {
    for r in 0..5 {
        for c in 0..5 {
            //rng.generate() returns a bool, convert to u8 type (1 or 0) for game logic
            fb[r][c] = if rng.generate::<bool>() { 1 } else { 0 };
        }
    }
}

// helper to complement baord states (invert all cells)
fn complement_board(fb: &mut [[u8; 5]; 5]){
    for r in 0..5 {
        for c in 0..5 {
            fb[r][c] = if fb[r][c] == 1 { 0 } else { 1 };
        }
    }
}

/* Scrap code
state = match state {
            State::LedOff => {
                board.display_pins.row1.set_high().unwrap();
                rprintln!("high");
                State::LedOff
            }
            State::LedOn => {
                board.display_pins.row1.set_low().unwrap();
                rprintln!("low");
                State::LedOff
            }

 let mut btn_a = board.buttons.button_a;
    let mut btn_b = board.buttons.button_b;

    //config LED grid (top-left at row1, col1)
    let mut row1 = board
        .display_pins
        .row1
        .into_push_pull_output(gpio::Level::Low);
    let _col1 = board
        .display_pins
        .col1
        .into_push_pull_output(gpio::Level::Low);

*/