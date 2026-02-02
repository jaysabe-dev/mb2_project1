# Game of Life :: MicroBit v2

**Author:** Jay Abegglem

## What I Did

For this assignment, I implemented **Conway’s Game of Life** in **Rust** and ran it on a **BBC micro:bit v2** embedded device. The simulation was flashed directly to the board and renders the automaton on the micro:bit’s **5×5 LED matrix**. I used the microbit hardware abstraction layer (HAL) to initialize the board, control the LEDs, and manage timing between simulation steps.

## How It Went

Most of the project effort went into properly initializing and managing the correct variable types. Early on, I experimented with more low-level crates and direct peripheral access, which made even basic tasks like LED control and timing difficult to reason about. While this provided insight into how the hardware works, it also introduced a large amount of complexity and slowed overall progress.

Switching to the microbit hardware abstraction layer (HAL) significantly improved development. The HAL provided safer, higher-level interfaces for interacting with the board, but still required careful handling of ownership, lifetimes, and scope. I often encountered situations where the program logic was correct, but variable types were slightly mismatched or methods were out of scope due to Rust’s strict borrowing rules when working with HAL-managed peripherals.

Explicitly managing timers, delays, and peripheral ownership—rather than relying on standard Rust library features—also posed challenges. This became especially apparent when I attempted to implement a fully random seed_buffer for initializing the board state. Integrating randomness in a no-std embedded environment proved more complex than expected and consumed a significant amount of time, ultimately leading me to simplify the initialization logic.

Once the data type mismatches and scoping issues were resolved, the core Game of Life logic translated cleanly to the LED grid and behaved as expected. Debugging was more challenging than in a desktop environment, relying primarily on RTT logging and direct observation of the LED output to verify correctness.

## Observations

Working on this project highlighted the difference between algorithmic logic and embedded systems development. The limited 5×5 display significantly constrained how interesting patterns behaved: many classic Game of Life patterns stabilize or die out quickly on such a small grid. Despite the constraints, seeing the simulation run on physical hardware made the project more engaging and helped solidify my understanding of how low-level hardware setup interacts with higher-level Rust code. 



Copied as a template from : [Github template repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template):
