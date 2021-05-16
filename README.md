# Airtag dumper
Simple utitily to glitch and dump the NRF52832 firmware on an airtag using cheap hardware. Requirements:
 - An airtag
 - A bluepill STM32F103 eval board running https://github.com/pd0wm/airtag-glitcher
 - A `probe-rs` compattible debug adapter such as a J-Link
 
## Credits
 - [Colin O'Flynn](https://twitter.com/colinoflynn) for documenting the debug pads: https://github.com/colinoflynn/airtag-re
 - [stacksmashing](https://twitter.com/ghidraninja) for his video explaining the procedure: https://www.youtube.com/watch?v=_E0PWQvW-14
 - [LimitedResults](https://twitter.com/LimitedResults) for their original research into glitching the NRF52: https://limitedresults.com/2020/06/nrf52-debug-resurrection-approtect-bypass/
