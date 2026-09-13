use atapi::{ATAPI, DMAOrPIO, LBAOrCHS, MasterOrSlave};
use io::{outb, outw, inw};
use crate::SECTOR_SIZE;

pub trait ReadLba{
    // maximum sectors - 256 (sectors = 0), 1 sectors to CD-rom is 2048 bytes
    fn read_pio_lba_28(&self, sectors: u8, lba: usize, buffer: *mut u16) -> Result<(), &'static str>;
}
impl ReadLba for ATAPI{
    fn read_pio_lba_28(&self, sectors: u8, lba: usize, mut buffer: *mut u16) -> Result<(), &'static str> {
        let count = if sectors == 0 { 256 } else { sectors as usize };

        for s in 0..count {
            let sector_lba = lba + s;

            self.set_flags(MasterOrSlave::Master, LBAOrCHS::LBA);
            self.wait_busy();
            self.set_dma_or_pio(DMAOrPIO::PIO);

            // set byte transfer limit in LBA mid and LBA high registers: 2048 bytes (0x0800)
            outb(self.io_registers.lba_mid_rw_w, 0x00);
            outb(self.io_registers.lba_high_rw_w, 0x08);

            self.prepare_scsi();
            if let Some(_) = self.wait_drq_and_busy(){
                return Err("timeout waiting for DRQ \\ \0");
            }

            // send 12-byte SCSI READ (12) packet (6 words to data port)
            let packet: [u8; 12] = [
                0xA8, // SCSI READ (12) opcode
                0x00, // flags
                ((sector_lba >> 24) & 0xFF) as u8,
                ((sector_lba >> 16) & 0xFF) as u8,
                ((sector_lba >> 8) & 0xFF) as u8,
                (sector_lba & 0xFF) as u8,
                0x00,
                0x00,
                0x00,
                1,    // 1 sector (2048 bytes)
                0x00, // reserved
                0x00, // control
            ];

            for chunk in 0..6 {
                let low = packet[chunk * 2] as u16;
                let high = (packet[chunk * 2 + 1] as u16) << 8;
                outw(self.io_registers.data_register_rw_w, low | high);
            }

            self.prepare_scsi();
            if let Some(_) = self.wait_drq_and_busy(){
                return Err("timeout waiting for DRQ \\ \0");
            }

            for _ in 0..(SECTOR_SIZE / 2) {
                unsafe {
                    *buffer = inw(self.io_registers.data_register_rw_w);
                    buffer = buffer.add(1);
                }
            }
        }
        Ok(())
    }
}
