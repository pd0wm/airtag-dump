# Airtag dumper
Simple utility to glitch and dump the NRF52832 firmware on an airtag using cheap hardware. Requirements:
 - An airtag
 - A bluepill STM32F103 eval board running https://github.com/pd0wm/airtag-glitcher
 - A `probe-rs` compatible debug adapter such as a J-Link

Connect the following pins from the STM to the airtag (`[test point numbering](https://github.com/colinoflynn/airtag-re#test-points)):
| Function | STM | Airtag |
|----------|-----|--------|
| Glitch output | PB7 | 28 (using an NFET) |
| Trigger | PB8 | 34 (1.8V) |
| Power | PB9 | VCC1 + VCC2 |

Just run `cargo run` to start the process.
 
## Credits
 - [Colin O'Flynn](https://twitter.com/colinoflynn) for documenting the test points: https://github.com/colinoflynn/airtag-re
 - [stacksmashing](https://twitter.com/ghidraninja) for his video explaining the procedure: https://www.youtube.com/watch?v=_E0PWQvW-14
 - [LimitedResults](https://twitter.com/LimitedResults) for their original research into glitching the NRF52: https://limitedresults.com/2020/06/nrf52-debug-resurrection-approtect-bypass/
