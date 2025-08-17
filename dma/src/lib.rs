#![no_std]
#![allow(dead_code)]
#![allow(private_interfaces)]
#![cfg(target_pointer_width = "64")]

// local function to write byte to register
fn outb(port: u16, data: u8){
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") data
        )
    }
}

pub enum Result{
    Ok,
    Err
}
impl Result{
    pub fn unwrap(&self){
        match self{
            Result::Ok => {},
            Result::Err => panic!("")
        }
    }
    pub fn expect(&self, msg: &str){
        match self{
            Result::Ok => {},
            Result::Err => panic!("{}", msg)
        }
    }
}
// struct for containing all possible registers
// and build register set for each channel separately
#[derive(Copy, Clone)]
struct Registers{
    start_address_ww: u16,
    count_register_ww: u16,
    status_register_br: u16,
    command_register_bw: u16,
    request_register_bw: u16,
    single_channel_mask_register_bw: u16,
    mode_register_bw: u16,
    flip_flop_reset_register_bw: u16,
    intermediate_register_br: u16,
    master_reset_register_bw: u16,
    mask_reset_register_bw: u16,
    multichannel_mask_register_bw: u16,
    channel_page_register_brw: u16
}

// use this structure for comfortable choose correct channel
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum PossibleMasterChannels{
    FloppyDisk      = 2,
    ParallelPort    = 3,
}
// use this structure for comfortable choose correct channel
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum PossibleSlaveChannels{
    Cascading       = 4,
    SoundBlaster    = 5,
    SCSIController  = 6,
    Enternet        = 7
}

// enum for mode register, DMA need to know Periphal behavior
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum TransferType{
    DMASelfTest                 = 0b00,
    PeriphalIsWrtiteToMemory    = 0b01,
    PeriphalIsReadFromMemory    = 0b10
}

// enum for mode register, if TRUE address will be automate changed
// (incriment or decrement by IncOrDec enum)
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum AutoChangeAddr{
    False   = 0,
    True    = 1
}

// if inc each iteration address will be incremented, transfering from
// low addr to high addr
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum IncOrDec{
    Inc     = 0,
    Dec     = 1
}

// enum for set up DMA behavior
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum DMAMode{
    // transfering while DRQ demands
    TransferOnDemand    = 0b00,
    // transfering 1 iteration (byte/word - master/slave)
    // 1 time and masked
    SingleDMATransfer   = 0b01,
    // transfering all block of data
    BlockDMATransfer    = 0b10,
    // using when cascading
    CascadeMode         = 0b11,
}
// structure for work with master channels
// contain information about channel and registers for every channel
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct DmaMasterChannel<'a> {
    // this field contains info about current channel
    // you can transfer it to u8 if you want to match it with documentation
    pub channel: &'a PossibleMasterChannels,
    // this field contains all possible for use registers and info about them
    // READ THE SUFFIX! b - byte w - word, w - write r - read rw - read write
    // EXAMPLE: start_address_ww it accepts 16 bit data and register only for write
    pub registers: Registers
}

// structure for work with slave channels
// contain information about channel and registers for every channel
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct DmaSlaveChannel<'a> {
    // this field contains info about current channel
    // you can transfer it to u8 if you want to match it with documentation
    pub channel: &'a PossibleSlaveChannels,
    // this field contains all possible for use registers and info about them
    // READ THE SUFFIX! b - byte w - word, w - write r - read rw - read write
    // EXAMPLE: start_address_ww it accepts 16 bit data and register only for write
    pub registers: Registers
}

pub trait Dma {
    // if register accepts 16 bit value it firstly accepts low bytes, then high
    // to prevent sendings firstly high bytes, this function set register for accepting low bytes data first
    fn reset_flip_flop(&self);
    // to prevent unplanned channel work it need to masked first
    fn mask_channel(&self);
    // unmask channel after mask
    fn unmask_channel(&self);
    // set 24 bit addr it RAM for transfering
    fn set_start_addr(&self, high_bytes_addr: u8, low_bytes_addr: u16);
    // set repeats in bytes/words (master/slave channel) of incrementing address
    // WARNING: function automatically decrementing value
    fn set_repeats(&self, repeats: u16);
    // set mode for dma work using enum "modes"
    fn set_mod(&self, 
        transfer_type: TransferType,
        auto_change_addr: AutoChangeAddr,
        inc_or_dec: IncOrDec,
        dma_mode: DMAMode
    );
}

impl<'a> DmaMasterChannel<'a>{
    // generates new DMA master channel
        pub fn new(channel_number: &'a PossibleMasterChannels) -> Self{
        Self { 
            channel: channel_number, 
            registers: Registers { 
                start_address_ww: 0x00 + (*channel_number as u16 * 2),
                count_register_ww: 0x01 + (*channel_number as u16 * 2),
                status_register_br: 0x08,
                command_register_bw: 0x08,
                request_register_bw: 0x09,
                single_channel_mask_register_bw: 0x0A,
                mode_register_bw: 0x0B,
                flip_flop_reset_register_bw: 0x0C,
                intermediate_register_br: 0x0D,
                master_reset_register_bw: 0x0D,
                mask_reset_register_bw: 0x0E,
                multichannel_mask_register_bw: 0x0F,
                channel_page_register_brw: 0x80 + (*channel_number as u16) - 1
            }
        }
    }    
}
impl<'a> Dma for DmaMasterChannel<'a> {
        fn mask_channel(&self) {
        outb(self.registers.single_channel_mask_register_bw, 0b100 | *self.channel as u8);
    }
        fn unmask_channel(&self) {
        outb(self.registers.single_channel_mask_register_bw, 0b000 | *self.channel as u8);
    }
        fn reset_flip_flop(&self) {
        outb(self.registers.flip_flop_reset_register_bw, 0xFF);
    }
        fn set_start_addr(&self, high_bytes_addr: u8, low_bytes_addr: u16){
        outb(self.registers.start_address_ww, (low_bytes_addr & 0x00FF) as u8);
        outb(self.registers.start_address_ww, ((low_bytes_addr & 0xFF00) >> 8) as u8);
        outb(self.registers.channel_page_register_brw, high_bytes_addr);
    }
        fn set_repeats(&self, mut repeats: u16){
        repeats -= 1;
        outb(self.registers.count_register_ww, (repeats & 0x00FF) as u8);
        outb(self.registers.count_register_ww, ((repeats & 0xFF00) >> 8) as u8);        
    }
        fn set_mod(&self, 
            transfer_type: TransferType,
            auto_change_addr: AutoChangeAddr,
            inc_or_dec: IncOrDec,
            dma_mode: DMAMode
        ) {
        outb(self.registers.mode_register_bw, 
            0 | (*self.channel as u8) | ((transfer_type as u8) << 2) |
            ((auto_change_addr as u8) << 4) | ((inc_or_dec as u8) << 5) |
            ((dma_mode as u8) << 6)
        );
    }
}

impl<'a> DmaSlaveChannel<'a>{
    // generates new DMA slave channel
        pub fn new(channel_number: &'a PossibleSlaveChannels) -> Self{
        Self { 
            channel: channel_number, 
            registers: Registers { 
                start_address_ww: 0xC0 + ((*channel_number as u16 - 4) * 4),
                count_register_ww: 0xC2 + ((*channel_number as u16 - 4) * 4),
                status_register_br: 0xD0,
                command_register_bw: 0xD0,
                request_register_bw: 0xD2,
                single_channel_mask_register_bw: 0xD4,
                mode_register_bw: 0xD6,
                flip_flop_reset_register_bw: 0xD8,
                intermediate_register_br: 0xDA,
                master_reset_register_bw: 0xDA,
                mask_reset_register_bw: 0xDC,
                multichannel_mask_register_bw: 0xDE,
                channel_page_register_brw: match channel_number{
                    &PossibleSlaveChannels::Cascading => 0x8F,
                    &PossibleSlaveChannels::SoundBlaster => 0x8B,
                    &PossibleSlaveChannels::SCSIController => 0x89,
                    &PossibleSlaveChannels::Enternet => 0x8A
                }
            }
        }
    }
}
impl<'a> Dma for DmaSlaveChannel<'a> {
        fn mask_channel(&self) {
        outb(self.registers.single_channel_mask_register_bw, 0b100 | *self.channel as u8);
    }
        fn unmask_channel(&self) {
        outb(self.registers.single_channel_mask_register_bw, 0b000 | *self.channel as u8);
    }
        fn reset_flip_flop(&self) {
        outb(self.registers.flip_flop_reset_register_bw, 0xFF);
    }
        fn set_start_addr(&self, high_bytes_addr: u8, low_bytes_addr: u16){
        outb(self.registers.start_address_ww, (low_bytes_addr & 0x00FF) as u8);
        outb(self.registers.start_address_ww, ((low_bytes_addr & 0xFF00) >> 8) as u8);
        outb(self.registers.channel_page_register_brw, high_bytes_addr);
    }
        fn set_repeats(&self, mut repeats: u16){
        repeats -= 1;
        outb(self.registers.count_register_ww, (repeats & 0x00FF) as u8);
        outb(self.registers.count_register_ww, ((repeats & 0xFF00) >> 8) as u8);        
    }
        fn set_mod(&self, 
            transfer_type: TransferType,
            auto_change_addr: AutoChangeAddr,
            inc_or_dec: IncOrDec,
            dma_mode: DMAMode
        ) {
        outb(self.registers.mode_register_bw, 
            0 | ((*self.channel as u8) - 4) | ((transfer_type as u8) << 2) |
            ((auto_change_addr as u8) << 4) | ((inc_or_dec as u8) << 5) |
            ((dma_mode as u8) << 6)
        );
    }
}