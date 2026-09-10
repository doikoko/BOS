use io::{inb, outb};
use crate::{pause::pause, rdtsc};

pub(crate) struct PIT{
    channel_2: u16,
    mode_command_register: u16,
    channel_2_gate: u16
}

impl PIT{
    pub(crate) fn new() -> Self{
        Self { 
            channel_2: 0x42,
            mode_command_register: 0x43,
            channel_2_gate: 0x61
        }
    }

    pub(crate) fn get_tsc_frequency(&self) -> u64 {
        const PIT_FREQ: u32 = 1_193_182;
        const DURATION_MS: u32 = 50;
        let pit_ticks = (PIT_FREQ * DURATION_MS) / 1000;

        const CHANNEL_2: u8 = 0b10 << 6;
        const LOBYTE_HIBYTE: u8 = 0b11 << 4;

        const CHANNEL_2_GATE_CONTROL: u8 = 1;
        const PC_SPEAKER_DATA_ENABLE: u8 = 1 << 1;

        let orig_2_gate = inb(self.channel_2_gate);
        outb(self.channel_2_gate, orig_2_gate & !(CHANNEL_2_GATE_CONTROL | PC_SPEAKER_DATA_ENABLE));
        
        outb(self.mode_command_register, CHANNEL_2 | LOBYTE_HIBYTE);
        outb(self.channel_2, (pit_ticks & 0xFF) as u8);
        outb(self.channel_2, ((pit_ticks >> 8) & 0xFF) as u8);

        let start = 0u64;
        rdtsc!(start);

        let current_61 = inb(self.channel_2_gate);
        outb(self.channel_2_gate, current_61 & !PC_SPEAKER_DATA_ENABLE | CHANNEL_2_GATE_CONTROL);

        const CHANNEL_2_OUTPUT_PIN_STATUS: u8 = 1 << 5;
        while (inb(self.channel_2_gate) & CHANNEL_2_OUTPUT_PIN_STATUS) == 0 {
            pause();
        }

        let end = 0u64;
        rdtsc!(end);

        outb(self.channel_2_gate, orig_2_gate);
        
        (end - start) * (1000 / DURATION_MS as u64)
    }
}