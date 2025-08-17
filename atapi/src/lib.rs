#![no_std]
#![allow(dead_code)]
#![allow(private_interfaces)]

#[cfg(target_pointer_width = "64")]
pub mod atapi64{
    // function for write to port 1 byte data
    pub fn outb(port: u16, data: u8){
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") port,
                in("al") data
            )
        }
    }
    // function for write to port 1 byte data
    pub fn outw(port: u16, data: u16){
        unsafe {
            core::arch::asm!(
                "out dx, ax",
                in("dx") port,
                in("ax") data
            )
        }
    }// function to get 8 bit data from port
    pub fn inb(port: u16) -> u8{
        unsafe {
            let value: u8;
            core::arch::asm!(
                "in al, dx",
                in("dx") port,
                out("al") value
            );
            value
        }
    }
    // function to get 16 bit data from port
    pub fn inw(port: u16) -> u16{
        unsafe {
            let value: u16;
            core::arch::asm!(
                "in ax, dx",
                in("dx") port,
                out("ax") value
            );
            value
        }
    }
    
    // R - Read, W - Write, B - Byte, W - Word LBA48
    pub struct ATAPI {
        pub io_registers: IORegisters,
        pub control_registers: ControlRegisters
    }
    pub struct IORegisters{
        pub data_register_rw_w: u16,
        pub error_r_or_features_w_w: u16,
        pub sector_count_rw_w: u16,
        pub lba_low_rw_w: u16,
        pub lba_mid_rw_w: u16,
        pub lba_high_rw_w: u16,
        pub device_or_head_rw_b: u16,
        pub command_w_or_status_r_b: u16
    }
    pub struct ControlRegisters{
        pub alternate_status_register_r_b: u16,
        pub device_control_register_w_b: u16,
        pub drive_address_register_r_b: u16
    }
    #[repr(u8)]
    pub enum DMAOrPIO{
        DMA = 1,
        PIO = 0
    }
    #[repr(u8)]
    pub enum MasterOrSlave{
        Slave = 1 << 4,
        Master = 0
    }
    #[repr(u8)]
    pub enum LBAOrCHS{
        LBA = 1 << 6,
        CHS = 0,
    }
    #[derive(Clone, Copy)]
    #[repr(u16)]
    pub enum PrimaryOrSecondary{
        Primary = 0x1F0,
        Secondary = 0x170
    }
    // construction of ErrorRegister
    // each field contain value matched with name
    // EXAMPLE: AddressMarkNotFound = 1 << 0,
    // you can match value of your register with this enum
    #[repr(u8)]
    pub enum IOErrorRegister{
        AddressMarkNotFound     = 1 << 0,
        TrackZeroNotFound       = 1 << 1,
        AbortedCommand          = 1 << 2,
        MediaChangeRequest      = 1 << 3,
        IDNotFound              = 1 << 4,
        MediaChanged            = 1 << 5,
        UncorrectableDataError  = 1 << 6,
        BadBlockDetected        = 1 << 7
    }
    
    // construction of StatusRegister
    // each field contain value matched with name
    // EXAMPLE: AddressMarkNotFound = 1 << 0,
    // you can match value of your register with this enum
    #[repr(u8)]
    pub enum IOStatusRegister{
        // to reset this send a new command or nuke with software reset
        ErrorIndicator          = 1 << 0,
        IndexAlwaysZero         = 1 << 1,
        CorrectedDataAlwaysZero = 1 << 2,
        // set when the drive has PIO data to transfer, or is ready to accept PIO data
        DRQ                     = 1 << 3,
        // overlapped mode service request
        SRV                     = 1 << 4,
        DriveFaultError         = 1 << 5,
        // Bit is clear when drive is spun down, or after an error. Set otherwise. 
        RDY                     = 1 << 6,
        // Indicates the drive is preparing to send/receive data (wait for it to clear). 
        // In case of 'hang' (it never clears), do a software reset. 
        BSY                     = 1 << 7
    }
    #[repr(u8)]
    pub enum IODeviceOrHeadRegister{
        // In CHS addressing, bits 0 to 3 of the head. In LBA addressing, bits 24 to 27 of the block number
        AddrHigh    = 1 << 0,
        //  Selects the drive number
        DRV         = 1 << 4,
        // Uses CHS addressing if clear or LBA addressing if set
        IsLBA       = 1 << 6,
    }
    #[repr(u8)]
    pub enum ControlDeviceRegister{
        // Set this to stop the current device from sending interrupts. 
        NIEN        = 1 << 1,    
    }
    impl ATAPI {
            pub fn new(base: PrimaryOrSecondary) -> Self {
            Self {
                io_registers: IORegisters {
                    data_register_rw_w: base as u16,
                    error_r_or_features_w_w: base as u16 + 1,
                    sector_count_rw_w: base as u16 + 2,
                    lba_low_rw_w: base as u16 + 3,
                    lba_mid_rw_w: base as u16 + 4,
                    lba_high_rw_w: base as u16 + 5,
                    device_or_head_rw_b: base as u16 + 6,
                    command_w_or_status_r_b: base as u16 + 7 
                },
                control_registers: ControlRegisters { 
                    alternate_status_register_r_b: (base as u16) + 0x206,
                    device_control_register_w_b: (base as u16) + 0x206,
                    drive_address_register_r_b: (base as u16) + 0x206 + 1
                }
            }
        }
        // this function set uo device or head register
            pub fn set_flags(
            &self, 
            master_or_slave: MasterOrSlave,
            lba_or_chs: LBAOrCHS)
        {
            outb(self.io_registers.device_or_head_rw_b, 
                master_or_slave as u8 | lba_or_chs as u8);
        }
        // you need to use this function to set status of dma of PIO transfer
            pub fn set_dma_or_pio(&self, dma_or_pio: DMAOrPIO){
            outb(self.io_registers.error_r_or_features_w_w, dma_or_pio as u8);
        }
        // you need to use this function after each command
            pub fn clear_cache(&self){
            outb(self.io_registers.command_w_or_status_r_b, 0xE7);
        }
        // after this function need to clear_cache
            pub fn is_has_device(&self) -> bool {
            outb(self.io_registers.device_or_head_rw_b, 
                if self.io_registers.data_register_rw_w == 0x1F0 {0xA0} else {0xB0});
            outb(self.io_registers.sector_count_rw_w, 0);
            outb(self.io_registers.lba_low_rw_w, 0);
            outb(self.io_registers.lba_mid_rw_w, 0);
            outb(self.io_registers.lba_high_rw_w, 0);
            outb(self.io_registers.command_w_or_status_r_b, ATAPIOCommands::IdentifyDeviceB as u8);
            
            if inb(self.io_registers.command_w_or_status_r_b) == 0 {
                false
            } else {
                if let None = self.wait_busy(){
                    true
                } else {
                    false
                }
            }
        }
        // this function must to be used before each SCSCI command
        // and than you need to use wait_drq function
            pub fn prepare_scsi(&self) {
            outb(self.io_registers.command_w_or_status_r_b, ATAOtherCommands::PacketB as u8);
        }
        // this function wait while DRQ and BSY register is not ready
        // you need to use this function after send prepare to SCSI command
            pub fn wait_drq_and_busy(&self) -> Option<IOStatusRegister> {
            if let Some(reg) = self.wait_busy(){
                Some(reg)
            } else {
                while inb(self.io_registers.command_w_or_status_r_b) & IOStatusRegister::DRQ as u8 == 0 {}
                None
            }
        }
        // this function wait while command byte is busy
        // if this function return None no one error register is not set
        // otherwise return this register 
            pub fn wait_busy(&self) -> Option<IOStatusRegister>{
            while inb(self.io_registers.command_w_or_status_r_b) & IOStatusRegister::BSY as u8 != 0 {}
            let status = inb(self.io_registers.command_w_or_status_r_b);
            // if DriveFaultError ErrorIndicator not set
            if status & IOStatusRegister::DriveFaultError as u8 != 0 {
                Some(IOStatusRegister::DriveFaultError)
            } else if status & IOStatusRegister::ErrorIndicator as u8 != 0 {
                Some(IOStatusRegister::ErrorIndicator)
            } else {
                None
            }
        }
    }
    // List of all SCSI comamnds with repr(u8)
    // Example: SCSCICommands::TestUnitReady as u8 -> 0x00
    #[repr(u8)]
    pub enum SCSICommands {
        TestUnitReady             	= 0x00,
        RequestSense           		= 0x03,
        FormatUnit	               	= 0x04,
        Inquiry		                = 0x12,
        StartStopUnitOrEjectDevice	= 0x1B,
        PreventAllowMediumRemoval	= 0x1E,
        ReadFormatCapacities	    = 0x23,
        ReadCapacity                = 0x25,
        Read10B		                = 0x28,
        Write10B              		= 0x2A,
        Seek10B		                = 0x2B,
        WriteAndVerify10B   		= 0x2E,
        Verify10B	                = 0x2F,
        SynchonizeCache	        	= 0x35,
        WriteBuffer		            = 0x3B,
        ReadBuffer             		= 0x3C,
        ReadTocORpmaORatip		    = 0x43,
        GetConfiguration	        = 0x46,
        GetEventStatusNotification	= 0x4A,
        ReadDiscInformation		    = 0x51,
        ReadTrackInformation	    = 0x52,
        ReserveTrack		        = 0x53,
        SendOpcInformation	    	= 0x54,
        ModeSelect10B		        = 0x55,
        RepairTrack		            = 0x58,
        ModeSense10B		        = 0x5A,
        CloseTrackSession		    = 0x5B,
        ReadBufferCapacity		    = 0x5C,
        SendCUESheet		        = 0x5D,
        ReportLUNS		            = 0xA0,
        Blank		                = 0xA1,
        SecurityProtocolIn		    = 0xA2,
        SendKey	                	= 0xA3,
        ReportKey		            = 0xA4,
        LoadOrUloadOrMedium		    = 0xA6,
        SetReadAhead         		= 0xA7,
        Read12B		                = 0xA8,
        Write12B		            = 0xAA,
        ReadMediaSerialNumber   	= 0xAB,
        ServiceActionIn12B          = 0x01,
        GetPerformance		        = 0xAC,
        ReadDiscStructure	      	= 0xAD,
        SecurityProtocolOut		    = 0xB5,
        SetStreaming           		= 0xB6,
        ReadCDMSF		            = 0xB9,
        SetCDspeed		            = 0xBB,
        MechanismStatus	        	= 0xBD,
        ReadCD		                = 0xBE,
        SendDiscStructure		    = 0xBF,
    }
    // B - byte, W - word
    #[repr(u8)]
    pub enum ATADMACommands{
        DataSetManagementB          = 0x06,
        DataSetManagementXlB        = 0x07,
        GetPhysicalElementStatusW   = 0x12,
        ReadDMAExtW                 = 0x25,
        ReadDMAQueuedExtW           = 0x26,
        ReadStreamDmaExtW           = 0x2A,
        WriteDMAExtW                = 0x35,
        WriteDMAQueuedExtW          = 0x36,
        WriteStreamDMAExtW          = 0x3A,
        WriteDMAFuaExtW             = 0x3D,
        WWriteDMAQueuedFuaExtW      = 0x3E,
        ReadLogDMAExtW              = 0x47,
        ZacManagementInW            = 0x4A,
        WriteLogDMAExtW             = 0x57,
        TrustedRecieveDMAB          = 0x5D,
        TrustedSendDMAB             = 0x5F,
        ReadFPDMAQueuedW            = 0x60,
        WriteFPDMAQueuedW           = 0x61,
        SendFPDMAQueuedW            = 0x64,
        RecieveFPDMAQueuedW         = 0x65,
        DownloadMicrorodeDMAB       = 0x93,
        XacmanagementOutW           = 0x9F,
        NVCacheW                    = 0xB6,
        ReadDMAQueuedB              = 0xC7,
        ReadDMAB                    = 0xC8,
        ReadDmaWithoutRetryB        = 0xC9,
        WriteDmaB                   = 0xCA,
        WriteDmaWithoutRetryB       = 0xCB,
        WriteDMAQueuedB             = 0xCC,
        ReadBufferDMA               = 0xE9,
        WriteBufferDMA              = 0xEB,
        IdentifyDeviceDMA           = 0xEE
    }
    // B - byte, W - word
    #[repr(u8)]
    pub enum ATAPIOCommands{
        ReadSectorsB                = 0x20,
        ReadSectorsWithoutRetryB    = 0x21,
        ReadLongB                   = 0x22,
        ReadLongWithoutRetryB       = 0x23,
        ReadSectorsExtW             = 0x24,
        ReadMultipleExtW            = 0x29,
        ReadStreamExtW              = 0x2B,
        ReadLogExtW                 = 0x2F,
        WriteSectorsB               = 0x30,
        WriteSectorsWithoutRetryB   = 0x31,
        WriteLongB                  = 0x32,
        WriteLongWithoutRetryB      = 0x33,
        WriteSectorsExtW            = 0x34,
        CfaWriteSectorsWithoutEraseB= 0x38,
        WriteMultipleExtW           = 0x39,
        WriteStreamExtW             = 0x3B,
        WriteVerifyB                = 0x3C,
        WriteLogExt                 = 0x3F,
        FormatTrackB                = 0x50,
        TrustedRecieveB             = 0x5C,
        TrustedSendB                = 0x5E,
        CfaTranslateSectorB         = 0x87,
        DownloadMicrocodeB          = 0x92,
        IdentifyPacketDeivceB       = 0xA1,
        SmartB                      = 0xB0,
        DeviceConfigurationOverlayB = 0xB1,
        ReadMultipleB               = 0xC4,
        WriteMultipleB              = 0xC5,
        CfaWriteMultipleWithoutEraseB= 0xCD,
        WriteMultipleFuaExtW        = 0xCE,
        ReadBufferB                 = 0xE4,
        WriteBufferB                = 0xE8,
        IdentifyDeviceB             = 0xEC,
        SecuritySetPasswordB        = 0xF1,
        SecurityUnlockB             = 0xF2,
        SecurityEraseUnitB          = 0xF4,
        SecurityDisablePasswordB    = 0xF6
    }
    // B - byte, W - word
    #[repr(u8)]
    pub enum ATANoneCommands{
        NOPB                        = 0x00,
        CfaRequestExtendedErrorCodeB= 0x03,
        DeviceResetB                = 0x08,
        RequestSenseDataExtW        = 0x0B,
        ReadNativeMaxAddressExtW    = 0x27,
        SetMaxAddressExtW           = 0x37,
        ReadVerifySectorsB          = 0x40,
        ReadVerifySectorsWithoutRetryB= 0x41,
        ReadVerufySectorsExtW       = 0x42,
        ZeroExtW                    = 0x44,
        WriteUncorrectableExtW      = 0x45,
        ConfigureStreamW            = 0x51,
        TrustedNonDataB             = 0x5B,
        NcqNonDataW                 = 0x63,
        SetDateTimeExtW             = 0x77,
        AccessibleMaxAddressConfigurationW= 0x78,
        RemoveElementAndTruncateW   = 0x7C,
        RestoreElementsAndRebuildW  = 0x7D,
        RemoveElementAndModifyZonesW= 0x7E,
        ExecuteDeviceDiagnosticB    = 0x90,
        InitializeDeviceParametersB = 0x91,
        MutateExtW                  = 0x96,
        CheckPowerModeB             = 0x98,
        SleepW                      = 0x99,
        SetSectorConfigurationExtW  = 0xB2,
        SanitizeDeviceW             = 0xB4,
        CFAEraseSectorsB            = 0xC0,
        SetMultipleModeB            = 0xC6,
        CheckMediaCardTypeB         = 0xD1,
        GetMediaStatusB             = 0xDA,
        AcknowlegdeMediaChangeB     = 0xDB,
        BootPostBootB               = 0xDC,
        BootPrebootB                = 0xDD,
        MediaLockB                  = 0xDE,
        MediaUnlockB                = 0xDF,
        StandbyImmediateB           = 0xE0,
        IDLEImmediateB              = 0xE1,
        StandbyB                    = 0xE2,
        IDLEB                       = 0xE3,
        FlushCacheB                 = 0xE7,
        FlushCacheExtB              = 0xEA,
        MediaEjectB                 = 0xED,
        SetFeaturesB                = 0xEF,
        SecurityErasePrepare        = 0xF3,
        SecuriteFreezeLockB         = 0xF5,
        ReadNativeMaxAddressB       = 0xF8,
        SetMaxAddressB              = 0xF9
    }
    // B - byte, W - word
    #[repr(u8)]
    pub enum ATAOtherCommands {
        PacketB     = 0xA0,
        ServiceB    = 0xA2,
    }
}
