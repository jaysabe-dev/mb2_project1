# Copilot Instructions for mb2_project1

## Project Overview
CS-471/571 Winter 2026 assignment: Interactive Conway's Game of Life on BBC micro:bit v2 (nrf52833). The program runs at 10 fps (100ms updates) with button-driven interactivity: button A re-randomizes, button B complements the board, and the program auto-resets when the board stabilizes. Implemented in bare-metal Rust using `cortex-m-rt` and microbit HAL. No_std, no_main, custom entry point with RTT logging.

## Architecture

### Key Components
- **[../src/main.rs](../src/main.rs)**: Entry point with board initialization and main loop. Must implement:
  - Timer-based 10 fps update loop (100ms intervals)
  - Button state polling (A and B inputs are active-low)
  - Random board generation and display updates
  - State machine for game logic (normal play, A-button override, B-button complement, board-empty wait)
- **[../src/life.rs](../src/life.rs)**: Conway's Game of Life engine (provided). Core functions:
  - `done(fb)`: Checks if frame buffer is empty (all zeros)
  - `life(fb)`: Applies one generation step with 5×5 toroidal wraparound logic

### Critical Implementation Details
- Frame buffer is `[[u8; 5]; 5]` where 0 = dead cell, 1 = alive cell
- Toroidal topology: edges wrap around (see modulo arithmetic in neighbor calculation)
- No dynamic allocation (heapless, embedded constraint)
- Logging via RTT (Real Time Transfer), not stdout
- **Button inputs are active-low** (logic 0 when pressed, 1 when released) due to hardware design
- **Button debouncing**: With 100ms poll interval, mechanical bounce is naturally handled
- **Timer**: Use `microbit::hal::Timer` for frame rate control
- **Display**: Use `Board.display` (Display struct with `show()` method) to drive 5×5 LED matrix
- **Buttons**: Use `Board.buttons` (Buttons struct with `button_a` and `button_b` members)

## Build & Development Workflow

### Setup (one-time)
```bash
rustup target add thumbv7em-none-eabihf  # ARM Cortex-M4 with FPU
rustup component add llvm-tools
cargo install cargo-binutils              # For binary tools
cargo install --locked probe-rs-tools     # For device flashing
```

### Build
```bash
cargo build --release  # Optimized embedded build
```

### Debug & Flash
- **[Embed.toml](../Embed.toml)** configures probe-rs with RTT enabled
- Flash and debug: `cargo embed --release`
- RTT output appears in console during execution

### Key Build Settings
- **Edition**: 2024 (check Cargo.toml - unusual, may need validation)
- **Target**: `thumbv7em-none-eabihf` (ARMv7 Cortex-M4)
- **No heap/allocator**: Implies all code uses stack or static allocation

## Project Conventions

### Code Style
- No clippy warnings (see `#[allow(clippy::manual_range_contains)]` in life.rs - intentional exception)
- Use `rprintln!()` for logging (not `println!`)
- Always unwrap board initialization (panics indicate hardware errors - expected in bare-metal)

### State Machine Logic (Main Loop Hierarchy)
The main loop must implement this priority order each 100ms frame:
1. **A-button held**: Re-randomize board every frame, skip all other logic
2. **B-button pressed** (not in ignore window): Complement board (`fb[r][c] = 1 - fb[r][c]`), set 5-frame ignore timer, proceed to step 4
3. **Board empty check**: If `done(fb)` is true:
   - Wait 5 frames (0.5s)
   - If no button pressed during wait, generate new random board
   - Otherwise, reset wait counter and continue
4. **Normal step**: Apply `life(fb)` once per frame

### Module Organization
- Keep device-specific code in main.rs
- Algorithm/business logic in separate modules (life.rs pattern)
- No std library - use core/cortex-m crate equivalents

### Critical Constraints
- **Stack-only allocation**: No Box, Vec, or heap allocation. Use arrays or fixed-capacity types from `heapless`
- **Synchronous execution**: No async/await without proper Cortex-M support
- **Panic handling**: Custom panic handler (`panic_rtt_target`) routes panics to RTT

## Dependencies & Integration

### Core Dependencies
- `cortex-m-rt`: Provides `#[entry]` macro and ARM runtime
- `microbit-v2` / `nrf52833-hal`: Board and chip HAL abstractions
  - `Board::take().unwrap()`: Get exclusive board access
  - `Board.display`: LED matrix control (call `show(leds)` with `[[u8; 5]; 5]`)
  - `Board.buttons.button_a` / `button_b`: InputPin trait (read with `.is_low()` for active-low logic)
  - `Board.timer0`: Timer for frame timing via `wait(Duration)` or busy-wait
- `rtt-target`: Real Time Transfer logging (bidirectional serial over JTAG)
- `panic-rtt-target`: Panic messages go to RTT, not lost on embedded hardware
- `nanorand` (optional): Hardware-seeded RNG via `Pcg64` for random board generation
  - Initialize: `let mut rng = Pcg64::new_seed(1);`
  - Generate bool: `let cell: bool = rng.generate();`

### Typical Integration Patterns
- Board initialization: `Board::take().unwrap()` (panics if called twice)
- HAL access: Destructure `Board` for peripherals (GPIO, timers, I2C, etc.)
- Infinite loop: `#[entry]` function must return `!` (never type)
- Button read (active-low): `if button_a.is_low().ok() { /* pressed */ }`
- Display update: `display.show(&fb);`

## Testing & Validation

### No Traditional Unit Tests
- Embedded constraints prevent standard `cargo test`
- Logic modules (life.rs) can use property-based tests if needed
- Device behavior validated on-hardware via RTT logging

### Recommended Validation
- Log frame buffer state changes via RTT
- Verify Game of Life rules with known patterns (blinkers, blocks, gliders)
- Use `done()` to detect simulation termination
- Test button state transitions and timing with manual press/release

## Common Tasks

### Implement Main Loop Frame Timing
Use a Timer to enforce 100ms frame intervals. Example pattern:
```rust
let mut timer = board.timer;
loop {
    // Poll buttons, update game state, display...
    timer.wait(Duration::from_millis(100));
}
```

### Generate Random Board
```rust
use nanorand::{Pcg64, Rng, SeedableRng};
let mut rng = Pcg64::new_seed(hw_rng_seed);
for r in 0..5 {
    for c in 0..5 {
        fb[r][c] = if rng.generate::<bool>() { 1 } else { 0 };
    }
}
```

### Read Buttons (Active-Low Logic)
```rust
let a_pressed = board.buttons.button_a.is_low().ok() == Some(true);
let b_pressed = board.buttons.button_b.is_low().ok() == Some(true);
```

### Display Frame Buffer
```rust
board.display.show(&mut fb);
```

### Modify Game of Life Rules
- Edit `life()` function neighbor matching logic
- Ensure toroidal wraparound is preserved (modulo arithmetic)

### Add Debug Output
- Use `rprintln!("value: {}", var)` instead of println
- Connect debugger to see RTT output in real-time

## References
- [Cortex-M docs](https://docs.rs/cortex-m/)
- [microbit-v2 crate](https://docs.rs/microbit/)
- [embedded-rust book](https://docs.rust-embedded.org/)
- [nanorand](https://docs.rs/nanorand/) for RNG
- Probe-rs documentation in `Embed.toml` for debugging workflows
