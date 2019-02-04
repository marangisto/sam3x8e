#![no_std]
use volatile_register::{RO, WO, RW};

#[repr(C)]
/// High Speed MultiMedia Card Interface
pub struct HSMCI {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Data Timeout Register
  pub dtor: RW<u32>,
  /// [12]: SD/SDIO Card Register
  pub sdcr: RW<u32>,
  /// [16]: Argument Register
  pub argr: RW<u32>,
  /// [20]: Command Register
  pub cmdr: WO<u32>,
  /// [24]: Block Register
  pub blkr: RW<u32>,
  /// [28]: Completion Signal Timeout Register
  pub cstor: RW<u32>,
  /// [32]: Response Register
  pub rsprs: RO<u32>,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  /// [48]: Receive Data Register
  pub rdr: RO<u32>,
  /// [52]: Transmit Data Register
  pub tdr: WO<u32>,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: Status Register
  pub sr: RO<u32>,
  /// [68]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [72]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [76]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [80]: DMA Configuration Register
  pub dma: RW<u32>,
  /// [84]: Configuration Register
  pub cfg: RW<u32>,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protection Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protection Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  reserved0x100: u32,
  reserved0x104: u32,
  reserved0x108: u32,
  reserved0x10c: u32,
  reserved0x110: u32,
  reserved0x114: u32,
  reserved0x118: u32,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  reserved0x180: u32,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  reserved0x190: u32,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  reserved0x1e4: u32,
  reserved0x1e8: u32,
  reserved0x1ec: u32,
  reserved0x1f0: u32,
  reserved0x1f4: u32,
  reserved0x1f8: u32,
  reserved0x1fc: u32,
  /// [512]: FIFO Memory Aperture0
  pub fifos: RW<u32>,
}

pub fn hsmci() -> *mut HSMCI {
  1073741824 as *mut HSMCI
}

#[repr(C)]
/// Synchronous Serial Controller
pub struct SSC {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Clock Mode Register
  pub cmr: RW<u32>,
  reserved0x8: u32,
  reserved0xc: u32,
  /// [16]: Receive Clock Mode Register
  pub rcmr: RW<u32>,
  /// [20]: Receive Frame Mode Register
  pub rfmr: RW<u32>,
  /// [24]: Transmit Clock Mode Register
  pub tcmr: RW<u32>,
  /// [28]: Transmit Frame Mode Register
  pub tfmr: RW<u32>,
  /// [32]: Receive Holding Register
  pub rhr: RO<u32>,
  /// [36]: Transmit Holding Register
  pub thr: WO<u32>,
  reserved0x28: u32,
  reserved0x2c: u32,
  /// [48]: Receive Sync. Holding Register
  pub rshr: RO<u32>,
  /// [52]: Transmit Sync. Holding Register
  pub tshr: RW<u32>,
  /// [56]: Receive Compare 0 Register
  pub rc0r: RW<u32>,
  /// [60]: Receive Compare 1 Register
  pub rc1r: RW<u32>,
  /// [64]: Status Register
  pub sr: RO<u32>,
  /// [68]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [72]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [76]: Interrupt Mask Register
  pub imr: RO<u32>,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
}

pub fn ssc() -> *mut SSC {
  1073758208 as *mut SSC
}

#[repr(C)]
/// Serial Peripheral Interface 0
pub struct SPI0 {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Receive Data Register
  pub rdr: RO<u32>,
  /// [12]: Transmit Data Register
  pub tdr: WO<u32>,
  /// [16]: Status Register
  pub sr: RO<u32>,
  /// [20]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [24]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [28]: Interrupt Mask Register
  pub imr: RO<u32>,
  reserved0x20: u32,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  /// [48]: Chip Select Register
  pub csrs: RW<u32>,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protection Control Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protection Status Register
  pub wpsr: RO<u32>,
}

pub fn spi0() -> *mut SPI0 {
  1073774592 as *mut SPI0
}

#[repr(C)]
/// Timer Counter 0
pub struct TC0 {
  /// [0]: Channel Control Register (channel = 0)
  pub ccr0: WO<u32>,
  /// [4]: Channel Mode Register (channel = 0)
  pub cmr0_wave_eq_1: RW<u32>,
  /// [8]: Stepper Motor Mode Register (channel = 0)
  pub smmr0: RW<u32>,
  reserved0xc: u32,
  /// [16]: Counter Value (channel = 0)
  pub cv0: RO<u32>,
  /// [20]: Register A (channel = 0)
  pub ra0: RW<u32>,
  /// [24]: Register B (channel = 0)
  pub rb0: RW<u32>,
  /// [28]: Register C (channel = 0)
  pub rc0: RW<u32>,
  /// [32]: Status Register (channel = 0)
  pub sr0: RO<u32>,
  /// [36]: Interrupt Enable Register (channel = 0)
  pub ier0: WO<u32>,
  /// [40]: Interrupt Disable Register (channel = 0)
  pub idr0: WO<u32>,
  /// [44]: Interrupt Mask Register (channel = 0)
  pub imr0: RO<u32>,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: Channel Control Register (channel = 1)
  pub ccr1: WO<u32>,
  /// [68]: Channel Mode Register (channel = 1)
  pub cmr1_wave_eq_1: RW<u32>,
  /// [72]: Stepper Motor Mode Register (channel = 1)
  pub smmr1: RW<u32>,
  reserved0x4c: u32,
  /// [80]: Counter Value (channel = 1)
  pub cv1: RO<u32>,
  /// [84]: Register A (channel = 1)
  pub ra1: RW<u32>,
  /// [88]: Register B (channel = 1)
  pub rb1: RW<u32>,
  /// [92]: Register C (channel = 1)
  pub rc1: RW<u32>,
  /// [96]: Status Register (channel = 1)
  pub sr1: RO<u32>,
  /// [100]: Interrupt Enable Register (channel = 1)
  pub ier1: WO<u32>,
  /// [104]: Interrupt Disable Register (channel = 1)
  pub idr1: WO<u32>,
  /// [108]: Interrupt Mask Register (channel = 1)
  pub imr1: RO<u32>,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: Channel Control Register (channel = 2)
  pub ccr2: WO<u32>,
  /// [132]: Channel Mode Register (channel = 2)
  pub cmr2_wave_eq_1: RW<u32>,
  /// [136]: Stepper Motor Mode Register (channel = 2)
  pub smmr2: RW<u32>,
  reserved0x8c: u32,
  /// [144]: Counter Value (channel = 2)
  pub cv2: RO<u32>,
  /// [148]: Register A (channel = 2)
  pub ra2: RW<u32>,
  /// [152]: Register B (channel = 2)
  pub rb2: RW<u32>,
  /// [156]: Register C (channel = 2)
  pub rc2: RW<u32>,
  /// [160]: Status Register (channel = 2)
  pub sr2: RO<u32>,
  /// [164]: Interrupt Enable Register (channel = 2)
  pub ier2: WO<u32>,
  /// [168]: Interrupt Disable Register (channel = 2)
  pub idr2: WO<u32>,
  /// [172]: Interrupt Mask Register (channel = 2)
  pub imr2: RO<u32>,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  /// [192]: Block Control Register
  pub bcr: WO<u32>,
  /// [196]: Block Mode Register
  pub bmr: RW<u32>,
  /// [200]: QDEC Interrupt Enable Register
  pub qier: WO<u32>,
  /// [204]: QDEC Interrupt Disable Register
  pub qidr: WO<u32>,
  /// [208]: QDEC Interrupt Mask Register
  pub qimr: RO<u32>,
  /// [212]: QDEC Interrupt Status Register
  pub qisr: RO<u32>,
  /// [216]: Fault Mode Register
  pub fmr: RW<u32>,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
}

pub fn tc0() -> *mut TC0 {
  1074266112 as *mut TC0
}

#[repr(C)]
/// Timer Counter 1
pub struct TC1 {
  /// [0]: Channel Control Register (channel = 0)
  pub ccr0: WO<u32>,
  /// [4]: Channel Mode Register (channel = 0)
  pub cmr0_wave_eq_1: RW<u32>,
  /// [8]: Stepper Motor Mode Register (channel = 0)
  pub smmr0: RW<u32>,
  reserved0xc: u32,
  /// [16]: Counter Value (channel = 0)
  pub cv0: RO<u32>,
  /// [20]: Register A (channel = 0)
  pub ra0: RW<u32>,
  /// [24]: Register B (channel = 0)
  pub rb0: RW<u32>,
  /// [28]: Register C (channel = 0)
  pub rc0: RW<u32>,
  /// [32]: Status Register (channel = 0)
  pub sr0: RO<u32>,
  /// [36]: Interrupt Enable Register (channel = 0)
  pub ier0: WO<u32>,
  /// [40]: Interrupt Disable Register (channel = 0)
  pub idr0: WO<u32>,
  /// [44]: Interrupt Mask Register (channel = 0)
  pub imr0: RO<u32>,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: Channel Control Register (channel = 1)
  pub ccr1: WO<u32>,
  /// [68]: Channel Mode Register (channel = 1)
  pub cmr1_wave_eq_1: RW<u32>,
  /// [72]: Stepper Motor Mode Register (channel = 1)
  pub smmr1: RW<u32>,
  reserved0x4c: u32,
  /// [80]: Counter Value (channel = 1)
  pub cv1: RO<u32>,
  /// [84]: Register A (channel = 1)
  pub ra1: RW<u32>,
  /// [88]: Register B (channel = 1)
  pub rb1: RW<u32>,
  /// [92]: Register C (channel = 1)
  pub rc1: RW<u32>,
  /// [96]: Status Register (channel = 1)
  pub sr1: RO<u32>,
  /// [100]: Interrupt Enable Register (channel = 1)
  pub ier1: WO<u32>,
  /// [104]: Interrupt Disable Register (channel = 1)
  pub idr1: WO<u32>,
  /// [108]: Interrupt Mask Register (channel = 1)
  pub imr1: RO<u32>,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: Channel Control Register (channel = 2)
  pub ccr2: WO<u32>,
  /// [132]: Channel Mode Register (channel = 2)
  pub cmr2_wave_eq_1: RW<u32>,
  /// [136]: Stepper Motor Mode Register (channel = 2)
  pub smmr2: RW<u32>,
  reserved0x8c: u32,
  /// [144]: Counter Value (channel = 2)
  pub cv2: RO<u32>,
  /// [148]: Register A (channel = 2)
  pub ra2: RW<u32>,
  /// [152]: Register B (channel = 2)
  pub rb2: RW<u32>,
  /// [156]: Register C (channel = 2)
  pub rc2: RW<u32>,
  /// [160]: Status Register (channel = 2)
  pub sr2: RO<u32>,
  /// [164]: Interrupt Enable Register (channel = 2)
  pub ier2: WO<u32>,
  /// [168]: Interrupt Disable Register (channel = 2)
  pub idr2: WO<u32>,
  /// [172]: Interrupt Mask Register (channel = 2)
  pub imr2: RO<u32>,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  /// [192]: Block Control Register
  pub bcr: WO<u32>,
  /// [196]: Block Mode Register
  pub bmr: RW<u32>,
  /// [200]: QDEC Interrupt Enable Register
  pub qier: WO<u32>,
  /// [204]: QDEC Interrupt Disable Register
  pub qidr: WO<u32>,
  /// [208]: QDEC Interrupt Mask Register
  pub qimr: RO<u32>,
  /// [212]: QDEC Interrupt Status Register
  pub qisr: RO<u32>,
  /// [216]: Fault Mode Register
  pub fmr: RW<u32>,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
}

pub fn tc1() -> *mut TC1 {
  1074282496 as *mut TC1
}

#[repr(C)]
/// Timer Counter 2
pub struct TC2 {
  /// [0]: Channel Control Register (channel = 0)
  pub ccr0: WO<u32>,
  /// [4]: Channel Mode Register (channel = 0)
  pub cmr0_wave_eq_1: RW<u32>,
  /// [8]: Stepper Motor Mode Register (channel = 0)
  pub smmr0: RW<u32>,
  reserved0xc: u32,
  /// [16]: Counter Value (channel = 0)
  pub cv0: RO<u32>,
  /// [20]: Register A (channel = 0)
  pub ra0: RW<u32>,
  /// [24]: Register B (channel = 0)
  pub rb0: RW<u32>,
  /// [28]: Register C (channel = 0)
  pub rc0: RW<u32>,
  /// [32]: Status Register (channel = 0)
  pub sr0: RO<u32>,
  /// [36]: Interrupt Enable Register (channel = 0)
  pub ier0: WO<u32>,
  /// [40]: Interrupt Disable Register (channel = 0)
  pub idr0: WO<u32>,
  /// [44]: Interrupt Mask Register (channel = 0)
  pub imr0: RO<u32>,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: Channel Control Register (channel = 1)
  pub ccr1: WO<u32>,
  /// [68]: Channel Mode Register (channel = 1)
  pub cmr1_wave_eq_1: RW<u32>,
  /// [72]: Stepper Motor Mode Register (channel = 1)
  pub smmr1: RW<u32>,
  reserved0x4c: u32,
  /// [80]: Counter Value (channel = 1)
  pub cv1: RO<u32>,
  /// [84]: Register A (channel = 1)
  pub ra1: RW<u32>,
  /// [88]: Register B (channel = 1)
  pub rb1: RW<u32>,
  /// [92]: Register C (channel = 1)
  pub rc1: RW<u32>,
  /// [96]: Status Register (channel = 1)
  pub sr1: RO<u32>,
  /// [100]: Interrupt Enable Register (channel = 1)
  pub ier1: WO<u32>,
  /// [104]: Interrupt Disable Register (channel = 1)
  pub idr1: WO<u32>,
  /// [108]: Interrupt Mask Register (channel = 1)
  pub imr1: RO<u32>,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: Channel Control Register (channel = 2)
  pub ccr2: WO<u32>,
  /// [132]: Channel Mode Register (channel = 2)
  pub cmr2_wave_eq_1: RW<u32>,
  /// [136]: Stepper Motor Mode Register (channel = 2)
  pub smmr2: RW<u32>,
  reserved0x8c: u32,
  /// [144]: Counter Value (channel = 2)
  pub cv2: RO<u32>,
  /// [148]: Register A (channel = 2)
  pub ra2: RW<u32>,
  /// [152]: Register B (channel = 2)
  pub rb2: RW<u32>,
  /// [156]: Register C (channel = 2)
  pub rc2: RW<u32>,
  /// [160]: Status Register (channel = 2)
  pub sr2: RO<u32>,
  /// [164]: Interrupt Enable Register (channel = 2)
  pub ier2: WO<u32>,
  /// [168]: Interrupt Disable Register (channel = 2)
  pub idr2: WO<u32>,
  /// [172]: Interrupt Mask Register (channel = 2)
  pub imr2: RO<u32>,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  /// [192]: Block Control Register
  pub bcr: WO<u32>,
  /// [196]: Block Mode Register
  pub bmr: RW<u32>,
  /// [200]: QDEC Interrupt Enable Register
  pub qier: WO<u32>,
  /// [204]: QDEC Interrupt Disable Register
  pub qidr: WO<u32>,
  /// [208]: QDEC Interrupt Mask Register
  pub qimr: RO<u32>,
  /// [212]: QDEC Interrupt Status Register
  pub qisr: RO<u32>,
  /// [216]: Fault Mode Register
  pub fmr: RW<u32>,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
}

pub fn tc2() -> *mut TC2 {
  1074298880 as *mut TC2
}

#[repr(C)]
/// Two-wire Interface 0
pub struct TWI0 {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Master Mode Register
  pub mmr: RW<u32>,
  /// [8]: Slave Mode Register
  pub smr: RW<u32>,
  /// [12]: Internal Address Register
  pub iadr: RW<u32>,
  /// [16]: Clock Waveform Generator Register
  pub cwgr: RW<u32>,
  reserved0x14: u32,
  reserved0x18: u32,
  reserved0x1c: u32,
  /// [32]: Status Register
  pub sr: RO<u32>,
  /// [36]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [40]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [44]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [48]: Receive Holding Register
  pub rhr: RO<u32>,
  /// [52]: Transmit Holding Register
  pub thr: WO<u32>,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  reserved0xe4: u32,
  reserved0xe8: u32,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Receive Pointer Register
  pub rpr: RW<u32>,
  /// [260]: Receive Counter Register
  pub rcr: RW<u32>,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  /// [272]: Receive Next Pointer Register
  pub rnpr: RW<u32>,
  /// [276]: Receive Next Counter Register
  pub rncr: RW<u32>,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn twi0() -> *mut TWI0 {
  1074315264 as *mut TWI0
}

#[repr(C)]
/// Two-wire Interface 1
pub struct TWI1 {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Master Mode Register
  pub mmr: RW<u32>,
  /// [8]: Slave Mode Register
  pub smr: RW<u32>,
  /// [12]: Internal Address Register
  pub iadr: RW<u32>,
  /// [16]: Clock Waveform Generator Register
  pub cwgr: RW<u32>,
  reserved0x14: u32,
  reserved0x18: u32,
  reserved0x1c: u32,
  /// [32]: Status Register
  pub sr: RO<u32>,
  /// [36]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [40]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [44]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [48]: Receive Holding Register
  pub rhr: RO<u32>,
  /// [52]: Transmit Holding Register
  pub thr: WO<u32>,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  reserved0xe4: u32,
  reserved0xe8: u32,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Receive Pointer Register
  pub rpr: RW<u32>,
  /// [260]: Receive Counter Register
  pub rcr: RW<u32>,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  /// [272]: Receive Next Pointer Register
  pub rnpr: RW<u32>,
  /// [276]: Receive Next Counter Register
  pub rncr: RW<u32>,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn twi1() -> *mut TWI1 {
  1074331648 as *mut TWI1
}

#[repr(C)]
/// Pulse Width Modulation Controller
pub struct PWM {
  /// [0]: PWM Clock Register
  pub clk: RW<u32>,
  /// [4]: PWM Enable Register
  pub ena: WO<u32>,
  /// [8]: PWM Disable Register
  pub dis: WO<u32>,
  /// [12]: PWM Status Register
  pub sr: RO<u32>,
  /// [16]: PWM Interrupt Enable Register 1
  pub ier1: WO<u32>,
  /// [20]: PWM Interrupt Disable Register 1
  pub idr1: WO<u32>,
  /// [24]: PWM Interrupt Mask Register 1
  pub imr1: RO<u32>,
  /// [28]: PWM Interrupt Status Register 1
  pub isr1: RO<u32>,
  /// [32]: PWM Sync Channels Mode Register
  pub scm: RW<u32>,
  reserved0x24: u32,
  /// [40]: PWM Sync Channels Update Control Register
  pub scuc: RW<u32>,
  /// [44]: PWM Sync Channels Update Period Register
  pub scup: RW<u32>,
  /// [48]: PWM Sync Channels Update Period Update Register
  pub scupupd: WO<u32>,
  /// [52]: PWM Interrupt Enable Register 2
  pub ier2: WO<u32>,
  /// [56]: PWM Interrupt Disable Register 2
  pub idr2: WO<u32>,
  /// [60]: PWM Interrupt Mask Register 2
  pub imr2: RO<u32>,
  /// [64]: PWM Interrupt Status Register 2
  pub isr2: RO<u32>,
  /// [68]: PWM Output Override Value Register
  pub oov: RW<u32>,
  /// [72]: PWM Output Selection Register
  pub os: RW<u32>,
  /// [76]: PWM Output Selection Set Register
  pub oss: WO<u32>,
  /// [80]: PWM Output Selection Clear Register
  pub osc: WO<u32>,
  /// [84]: PWM Output Selection Set Update Register
  pub ossupd: WO<u32>,
  /// [88]: PWM Output Selection Clear Update Register
  pub oscupd: WO<u32>,
  /// [92]: PWM Fault Mode Register
  pub fmr: RW<u32>,
  /// [96]: PWM Fault Status Register
  pub fsr: RO<u32>,
  /// [100]: PWM Fault Clear Register
  pub fcr: WO<u32>,
  /// [104]: PWM Fault Protection Value Register
  pub fpv: RW<u32>,
  /// [108]: PWM Fault Protection Enable Register 1
  pub fpe1: RW<u32>,
  /// [112]: PWM Fault Protection Enable Register 2
  pub fpe2: RW<u32>,
  reserved0x74: u32,
  reserved0x78: u32,
  /// [124]: PWM Event Line 0 Mode Register
  pub elmrs: RW<u32>,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  /// [176]: PWM Stepper Motor Mode Register
  pub smmr: RW<u32>,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: PWM Write Protect Control Register
  pub wpcr: WO<u32>,
  /// [232]: PWM Write Protect Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  reserved0x100: u32,
  reserved0x104: u32,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  reserved0x110: u32,
  reserved0x114: u32,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
  reserved0x128: u32,
  reserved0x12c: u32,
  /// [304]: PWM Comparison 0 Value Register
  pub cmpv0: RW<u32>,
  /// [308]: PWM Comparison 0 Value Update Register
  pub cmpvupd0: WO<u32>,
  /// [312]: PWM Comparison 0 Mode Register
  pub cmpm0: RW<u32>,
  /// [316]: PWM Comparison 0 Mode Update Register
  pub cmpmupd0: WO<u32>,
  /// [320]: PWM Comparison 1 Value Register
  pub cmpv1: RW<u32>,
  /// [324]: PWM Comparison 1 Value Update Register
  pub cmpvupd1: WO<u32>,
  /// [328]: PWM Comparison 1 Mode Register
  pub cmpm1: RW<u32>,
  /// [332]: PWM Comparison 1 Mode Update Register
  pub cmpmupd1: WO<u32>,
  /// [336]: PWM Comparison 2 Value Register
  pub cmpv2: RW<u32>,
  /// [340]: PWM Comparison 2 Value Update Register
  pub cmpvupd2: WO<u32>,
  /// [344]: PWM Comparison 2 Mode Register
  pub cmpm2: RW<u32>,
  /// [348]: PWM Comparison 2 Mode Update Register
  pub cmpmupd2: WO<u32>,
  /// [352]: PWM Comparison 3 Value Register
  pub cmpv3: RW<u32>,
  /// [356]: PWM Comparison 3 Value Update Register
  pub cmpvupd3: WO<u32>,
  /// [360]: PWM Comparison 3 Mode Register
  pub cmpm3: RW<u32>,
  /// [364]: PWM Comparison 3 Mode Update Register
  pub cmpmupd3: WO<u32>,
  /// [368]: PWM Comparison 4 Value Register
  pub cmpv4: RW<u32>,
  /// [372]: PWM Comparison 4 Value Update Register
  pub cmpvupd4: WO<u32>,
  /// [376]: PWM Comparison 4 Mode Register
  pub cmpm4: RW<u32>,
  /// [380]: PWM Comparison 4 Mode Update Register
  pub cmpmupd4: WO<u32>,
  /// [384]: PWM Comparison 5 Value Register
  pub cmpv5: RW<u32>,
  /// [388]: PWM Comparison 5 Value Update Register
  pub cmpvupd5: WO<u32>,
  /// [392]: PWM Comparison 5 Mode Register
  pub cmpm5: RW<u32>,
  /// [396]: PWM Comparison 5 Mode Update Register
  pub cmpmupd5: WO<u32>,
  /// [400]: PWM Comparison 6 Value Register
  pub cmpv6: RW<u32>,
  /// [404]: PWM Comparison 6 Value Update Register
  pub cmpvupd6: WO<u32>,
  /// [408]: PWM Comparison 6 Mode Register
  pub cmpm6: RW<u32>,
  /// [412]: PWM Comparison 6 Mode Update Register
  pub cmpmupd6: WO<u32>,
  /// [416]: PWM Comparison 7 Value Register
  pub cmpv7: RW<u32>,
  /// [420]: PWM Comparison 7 Value Update Register
  pub cmpvupd7: WO<u32>,
  /// [424]: PWM Comparison 7 Mode Register
  pub cmpm7: RW<u32>,
  /// [428]: PWM Comparison 7 Mode Update Register
  pub cmpmupd7: WO<u32>,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  reserved0x1e4: u32,
  reserved0x1e8: u32,
  reserved0x1ec: u32,
  reserved0x1f0: u32,
  reserved0x1f4: u32,
  reserved0x1f8: u32,
  reserved0x1fc: u32,
  /// [512]: PWM Channel Mode Register (ch_num = 0)
  pub cmr0: RW<u32>,
  /// [516]: PWM Channel Duty Cycle Register (ch_num = 0)
  pub cdty0: RW<u32>,
  /// [520]: PWM Channel Duty Cycle Update Register (ch_num = 0)
  pub cdtyupd0: WO<u32>,
  /// [524]: PWM Channel Period Register (ch_num = 0)
  pub cprd0: RW<u32>,
  /// [528]: PWM Channel Period Update Register (ch_num = 0)
  pub cprdupd0: WO<u32>,
  /// [532]: PWM Channel Counter Register (ch_num = 0)
  pub ccnt0: RO<u32>,
  /// [536]: PWM Channel Dead Time Register (ch_num = 0)
  pub dt0: RW<u32>,
  /// [540]: PWM Channel Dead Time Update Register (ch_num = 0)
  pub dtupd0: WO<u32>,
  /// [544]: PWM Channel Mode Register (ch_num = 1)
  pub cmr1: RW<u32>,
  /// [548]: PWM Channel Duty Cycle Register (ch_num = 1)
  pub cdty1: RW<u32>,
  /// [552]: PWM Channel Duty Cycle Update Register (ch_num = 1)
  pub cdtyupd1: WO<u32>,
  /// [556]: PWM Channel Period Register (ch_num = 1)
  pub cprd1: RW<u32>,
  /// [560]: PWM Channel Period Update Register (ch_num = 1)
  pub cprdupd1: WO<u32>,
  /// [564]: PWM Channel Counter Register (ch_num = 1)
  pub ccnt1: RO<u32>,
  /// [568]: PWM Channel Dead Time Register (ch_num = 1)
  pub dt1: RW<u32>,
  /// [572]: PWM Channel Dead Time Update Register (ch_num = 1)
  pub dtupd1: WO<u32>,
  /// [576]: PWM Channel Mode Register (ch_num = 2)
  pub cmr2: RW<u32>,
  /// [580]: PWM Channel Duty Cycle Register (ch_num = 2)
  pub cdty2: RW<u32>,
  /// [584]: PWM Channel Duty Cycle Update Register (ch_num = 2)
  pub cdtyupd2: WO<u32>,
  /// [588]: PWM Channel Period Register (ch_num = 2)
  pub cprd2: RW<u32>,
  /// [592]: PWM Channel Period Update Register (ch_num = 2)
  pub cprdupd2: WO<u32>,
  /// [596]: PWM Channel Counter Register (ch_num = 2)
  pub ccnt2: RO<u32>,
  /// [600]: PWM Channel Dead Time Register (ch_num = 2)
  pub dt2: RW<u32>,
  /// [604]: PWM Channel Dead Time Update Register (ch_num = 2)
  pub dtupd2: WO<u32>,
  /// [608]: PWM Channel Mode Register (ch_num = 3)
  pub cmr3: RW<u32>,
  /// [612]: PWM Channel Duty Cycle Register (ch_num = 3)
  pub cdty3: RW<u32>,
  /// [616]: PWM Channel Duty Cycle Update Register (ch_num = 3)
  pub cdtyupd3: WO<u32>,
  /// [620]: PWM Channel Period Register (ch_num = 3)
  pub cprd3: RW<u32>,
  /// [624]: PWM Channel Period Update Register (ch_num = 3)
  pub cprdupd3: WO<u32>,
  /// [628]: PWM Channel Counter Register (ch_num = 3)
  pub ccnt3: RO<u32>,
  /// [632]: PWM Channel Dead Time Register (ch_num = 3)
  pub dt3: RW<u32>,
  /// [636]: PWM Channel Dead Time Update Register (ch_num = 3)
  pub dtupd3: WO<u32>,
  /// [640]: PWM Channel Mode Register (ch_num = 4)
  pub cmr4: RW<u32>,
  /// [644]: PWM Channel Duty Cycle Register (ch_num = 4)
  pub cdty4: RW<u32>,
  /// [648]: PWM Channel Duty Cycle Update Register (ch_num = 4)
  pub cdtyupd4: WO<u32>,
  /// [652]: PWM Channel Period Register (ch_num = 4)
  pub cprd4: RW<u32>,
  /// [656]: PWM Channel Period Update Register (ch_num = 4)
  pub cprdupd4: WO<u32>,
  /// [660]: PWM Channel Counter Register (ch_num = 4)
  pub ccnt4: RO<u32>,
  /// [664]: PWM Channel Dead Time Register (ch_num = 4)
  pub dt4: RW<u32>,
  /// [668]: PWM Channel Dead Time Update Register (ch_num = 4)
  pub dtupd4: WO<u32>,
  /// [672]: PWM Channel Mode Register (ch_num = 5)
  pub cmr5: RW<u32>,
  /// [676]: PWM Channel Duty Cycle Register (ch_num = 5)
  pub cdty5: RW<u32>,
  /// [680]: PWM Channel Duty Cycle Update Register (ch_num = 5)
  pub cdtyupd5: WO<u32>,
  /// [684]: PWM Channel Period Register (ch_num = 5)
  pub cprd5: RW<u32>,
  /// [688]: PWM Channel Period Update Register (ch_num = 5)
  pub cprdupd5: WO<u32>,
  /// [692]: PWM Channel Counter Register (ch_num = 5)
  pub ccnt5: RO<u32>,
  /// [696]: PWM Channel Dead Time Register (ch_num = 5)
  pub dt5: RW<u32>,
  /// [700]: PWM Channel Dead Time Update Register (ch_num = 5)
  pub dtupd5: WO<u32>,
  /// [704]: PWM Channel Mode Register (ch_num = 6)
  pub cmr6: RW<u32>,
  /// [708]: PWM Channel Duty Cycle Register (ch_num = 6)
  pub cdty6: RW<u32>,
  /// [712]: PWM Channel Duty Cycle Update Register (ch_num = 6)
  pub cdtyupd6: WO<u32>,
  /// [716]: PWM Channel Period Register (ch_num = 6)
  pub cprd6: RW<u32>,
  /// [720]: PWM Channel Period Update Register (ch_num = 6)
  pub cprdupd6: WO<u32>,
  /// [724]: PWM Channel Counter Register (ch_num = 6)
  pub ccnt6: RO<u32>,
  /// [728]: PWM Channel Dead Time Register (ch_num = 6)
  pub dt6: RW<u32>,
  /// [732]: PWM Channel Dead Time Update Register (ch_num = 6)
  pub dtupd6: WO<u32>,
  /// [736]: PWM Channel Mode Register (ch_num = 7)
  pub cmr7: RW<u32>,
  /// [740]: PWM Channel Duty Cycle Register (ch_num = 7)
  pub cdty7: RW<u32>,
  /// [744]: PWM Channel Duty Cycle Update Register (ch_num = 7)
  pub cdtyupd7: WO<u32>,
  /// [748]: PWM Channel Period Register (ch_num = 7)
  pub cprd7: RW<u32>,
  /// [752]: PWM Channel Period Update Register (ch_num = 7)
  pub cprdupd7: WO<u32>,
  /// [756]: PWM Channel Counter Register (ch_num = 7)
  pub ccnt7: RO<u32>,
  /// [760]: PWM Channel Dead Time Register (ch_num = 7)
  pub dt7: RW<u32>,
  /// [764]: PWM Channel Dead Time Update Register (ch_num = 7)
  pub dtupd7: WO<u32>,
}

pub fn pwm() -> *mut PWM {
  1074348032 as *mut PWM
}

#[repr(C)]
/// Universal Synchronous Asynchronous Receiver Transmitter 0
pub struct USART0 {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [12]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [16]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [20]: Channel Status Register
  pub csr: RO<u32>,
  /// [24]: Receiver Holding Register
  pub rhr: RO<u32>,
  /// [28]: Transmitter Holding Register
  pub thr: WO<u32>,
  /// [32]: Baud Rate Generator Register
  pub brgr: RW<u32>,
  /// [36]: Receiver Time-out Register
  pub rtor: RW<u32>,
  /// [40]: Transmitter Timeguard Register
  pub ttgr: RW<u32>,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: FI DI Ratio Register
  pub fidi: RW<u32>,
  /// [68]: Number of Errors Register
  pub ner: RO<u32>,
  reserved0x48: u32,
  /// [76]: IrDA Filter Register
  pub r#if: RW<u32>,
  /// [80]: Manchester Encoder Decoder Register
  pub man: RW<u32>,
  /// [84]: LIN Mode Register
  pub linmr: RW<u32>,
  /// [88]: LIN Identifier Register
  pub linir: RW<u32>,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Receive Pointer Register
  pub rpr: RW<u32>,
  /// [260]: Receive Counter Register
  pub rcr: RW<u32>,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  /// [272]: Receive Next Pointer Register
  pub rnpr: RW<u32>,
  /// [276]: Receive Next Counter Register
  pub rncr: RW<u32>,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn usart0() -> *mut USART0 {
  1074364416 as *mut USART0
}

#[repr(C)]
/// Universal Synchronous Asynchronous Receiver Transmitter 1
pub struct USART1 {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [12]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [16]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [20]: Channel Status Register
  pub csr: RO<u32>,
  /// [24]: Receiver Holding Register
  pub rhr: RO<u32>,
  /// [28]: Transmitter Holding Register
  pub thr: WO<u32>,
  /// [32]: Baud Rate Generator Register
  pub brgr: RW<u32>,
  /// [36]: Receiver Time-out Register
  pub rtor: RW<u32>,
  /// [40]: Transmitter Timeguard Register
  pub ttgr: RW<u32>,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: FI DI Ratio Register
  pub fidi: RW<u32>,
  /// [68]: Number of Errors Register
  pub ner: RO<u32>,
  reserved0x48: u32,
  /// [76]: IrDA Filter Register
  pub r#if: RW<u32>,
  /// [80]: Manchester Encoder Decoder Register
  pub man: RW<u32>,
  /// [84]: LIN Mode Register
  pub linmr: RW<u32>,
  /// [88]: LIN Identifier Register
  pub linir: RW<u32>,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Receive Pointer Register
  pub rpr: RW<u32>,
  /// [260]: Receive Counter Register
  pub rcr: RW<u32>,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  /// [272]: Receive Next Pointer Register
  pub rnpr: RW<u32>,
  /// [276]: Receive Next Counter Register
  pub rncr: RW<u32>,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn usart1() -> *mut USART1 {
  1074380800 as *mut USART1
}

#[repr(C)]
/// Universal Synchronous Asynchronous Receiver Transmitter 2
pub struct USART2 {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [12]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [16]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [20]: Channel Status Register
  pub csr: RO<u32>,
  /// [24]: Receiver Holding Register
  pub rhr: RO<u32>,
  /// [28]: Transmitter Holding Register
  pub thr: WO<u32>,
  /// [32]: Baud Rate Generator Register
  pub brgr: RW<u32>,
  /// [36]: Receiver Time-out Register
  pub rtor: RW<u32>,
  /// [40]: Transmitter Timeguard Register
  pub ttgr: RW<u32>,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: FI DI Ratio Register
  pub fidi: RW<u32>,
  /// [68]: Number of Errors Register
  pub ner: RO<u32>,
  reserved0x48: u32,
  /// [76]: IrDA Filter Register
  pub r#if: RW<u32>,
  /// [80]: Manchester Encoder Decoder Register
  pub man: RW<u32>,
  /// [84]: LIN Mode Register
  pub linmr: RW<u32>,
  /// [88]: LIN Identifier Register
  pub linir: RW<u32>,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Receive Pointer Register
  pub rpr: RW<u32>,
  /// [260]: Receive Counter Register
  pub rcr: RW<u32>,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  /// [272]: Receive Next Pointer Register
  pub rnpr: RW<u32>,
  /// [276]: Receive Next Counter Register
  pub rncr: RW<u32>,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn usart2() -> *mut USART2 {
  1074397184 as *mut USART2
}

#[repr(C)]
/// Universal Synchronous Asynchronous Receiver Transmitter 3
pub struct USART3 {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [12]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [16]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [20]: Channel Status Register
  pub csr: RO<u32>,
  /// [24]: Receiver Holding Register
  pub rhr: RO<u32>,
  /// [28]: Transmitter Holding Register
  pub thr: WO<u32>,
  /// [32]: Baud Rate Generator Register
  pub brgr: RW<u32>,
  /// [36]: Receiver Time-out Register
  pub rtor: RW<u32>,
  /// [40]: Transmitter Timeguard Register
  pub ttgr: RW<u32>,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: FI DI Ratio Register
  pub fidi: RW<u32>,
  /// [68]: Number of Errors Register
  pub ner: RO<u32>,
  reserved0x48: u32,
  /// [76]: IrDA Filter Register
  pub r#if: RW<u32>,
  /// [80]: Manchester Encoder Decoder Register
  pub man: RW<u32>,
  /// [84]: LIN Mode Register
  pub linmr: RW<u32>,
  /// [88]: LIN Identifier Register
  pub linir: RW<u32>,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Receive Pointer Register
  pub rpr: RW<u32>,
  /// [260]: Receive Counter Register
  pub rcr: RW<u32>,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  /// [272]: Receive Next Pointer Register
  pub rnpr: RW<u32>,
  /// [276]: Receive Next Counter Register
  pub rncr: RW<u32>,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn usart3() -> *mut USART3 {
  1074413568 as *mut USART3
}

#[repr(C)]
/// USB On-The-Go Interface
pub struct UOTGHS {
  /// [0]: Device General Control Register
  pub devctrl: RW<u32>,
  /// [4]: Device Global Interrupt Status Register
  pub devisr: RO<u32>,
  /// [8]: Device Global Interrupt Clear Register
  pub devicr: WO<u32>,
  /// [12]: Device Global Interrupt Set Register
  pub devifr: WO<u32>,
  /// [16]: Device Global Interrupt Mask Register
  pub devimr: RO<u32>,
  /// [20]: Device Global Interrupt Disable Register
  pub devidr: WO<u32>,
  /// [24]: Device Global Interrupt Enable Register
  pub devier: WO<u32>,
  /// [28]: Device Endpoint Register
  pub devept: RW<u32>,
  /// [32]: Device Frame Number Register
  pub devfnum: RO<u32>,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  reserved0xe4: u32,
  reserved0xe8: u32,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Device Endpoint Configuration Register (n = 0)
  pub deveptcfgs: RW<u32>,
  reserved0x104: u32,
  reserved0x108: u32,
  reserved0x10c: u32,
  reserved0x110: u32,
  reserved0x114: u32,
  reserved0x118: u32,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  /// [304]: Device Endpoint Status Register (n = 0)
  pub deveptisrs: RO<u32>,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  /// [352]: Device Endpoint Clear Register (n = 0)
  pub devepticrs: WO<u32>,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  reserved0x180: u32,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  /// [400]: Device Endpoint Set Register (n = 0)
  pub deveptifrs: WO<u32>,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  /// [448]: Device Endpoint Mask Register (n = 0)
  pub deveptimrs: RO<u32>,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  reserved0x1e4: u32,
  reserved0x1e8: u32,
  reserved0x1ec: u32,
  /// [496]: Device Endpoint Enable Register (n = 0)
  pub deveptiers: WO<u32>,
  reserved0x1f4: u32,
  reserved0x1f8: u32,
  reserved0x1fc: u32,
  reserved0x200: u32,
  reserved0x204: u32,
  reserved0x208: u32,
  reserved0x20c: u32,
  reserved0x210: u32,
  reserved0x214: u32,
  reserved0x218: u32,
  reserved0x21c: u32,
  /// [544]: Device Endpoint Disable Register (n = 0)
  pub deveptidrs: WO<u32>,
  reserved0x224: u32,
  reserved0x228: u32,
  reserved0x22c: u32,
  reserved0x230: u32,
  reserved0x234: u32,
  reserved0x238: u32,
  reserved0x23c: u32,
  reserved0x240: u32,
  reserved0x244: u32,
  reserved0x248: u32,
  reserved0x24c: u32,
  reserved0x250: u32,
  reserved0x254: u32,
  reserved0x258: u32,
  reserved0x25c: u32,
  reserved0x260: u32,
  reserved0x264: u32,
  reserved0x268: u32,
  reserved0x26c: u32,
  reserved0x270: u32,
  reserved0x274: u32,
  reserved0x278: u32,
  reserved0x27c: u32,
  reserved0x280: u32,
  reserved0x284: u32,
  reserved0x288: u32,
  reserved0x28c: u32,
  reserved0x290: u32,
  reserved0x294: u32,
  reserved0x298: u32,
  reserved0x29c: u32,
  reserved0x2a0: u32,
  reserved0x2a4: u32,
  reserved0x2a8: u32,
  reserved0x2ac: u32,
  reserved0x2b0: u32,
  reserved0x2b4: u32,
  reserved0x2b8: u32,
  reserved0x2bc: u32,
  reserved0x2c0: u32,
  reserved0x2c4: u32,
  reserved0x2c8: u32,
  reserved0x2cc: u32,
  reserved0x2d0: u32,
  reserved0x2d4: u32,
  reserved0x2d8: u32,
  reserved0x2dc: u32,
  reserved0x2e0: u32,
  reserved0x2e4: u32,
  reserved0x2e8: u32,
  reserved0x2ec: u32,
  reserved0x2f0: u32,
  reserved0x2f4: u32,
  reserved0x2f8: u32,
  reserved0x2fc: u32,
  reserved0x300: u32,
  reserved0x304: u32,
  reserved0x308: u32,
  reserved0x30c: u32,
  /// [784]: Device DMA Channel Next Descriptor Address Register (n = 1)
  pub devdmanxtdsc1: RW<u32>,
  /// [788]: Device DMA Channel Address Register (n = 1)
  pub devdmaaddress1: RW<u32>,
  /// [792]: Device DMA Channel Control Register (n = 1)
  pub devdmacontrol1: RW<u32>,
  /// [796]: Device DMA Channel Status Register (n = 1)
  pub devdmastatus1: RW<u32>,
  /// [800]: Device DMA Channel Next Descriptor Address Register (n = 2)
  pub devdmanxtdsc2: RW<u32>,
  /// [804]: Device DMA Channel Address Register (n = 2)
  pub devdmaaddress2: RW<u32>,
  /// [808]: Device DMA Channel Control Register (n = 2)
  pub devdmacontrol2: RW<u32>,
  /// [812]: Device DMA Channel Status Register (n = 2)
  pub devdmastatus2: RW<u32>,
  /// [816]: Device DMA Channel Next Descriptor Address Register (n = 3)
  pub devdmanxtdsc3: RW<u32>,
  /// [820]: Device DMA Channel Address Register (n = 3)
  pub devdmaaddress3: RW<u32>,
  /// [824]: Device DMA Channel Control Register (n = 3)
  pub devdmacontrol3: RW<u32>,
  /// [828]: Device DMA Channel Status Register (n = 3)
  pub devdmastatus3: RW<u32>,
  /// [832]: Device DMA Channel Next Descriptor Address Register (n = 4)
  pub devdmanxtdsc4: RW<u32>,
  /// [836]: Device DMA Channel Address Register (n = 4)
  pub devdmaaddress4: RW<u32>,
  /// [840]: Device DMA Channel Control Register (n = 4)
  pub devdmacontrol4: RW<u32>,
  /// [844]: Device DMA Channel Status Register (n = 4)
  pub devdmastatus4: RW<u32>,
  /// [848]: Device DMA Channel Next Descriptor Address Register (n = 5)
  pub devdmanxtdsc5: RW<u32>,
  /// [852]: Device DMA Channel Address Register (n = 5)
  pub devdmaaddress5: RW<u32>,
  /// [856]: Device DMA Channel Control Register (n = 5)
  pub devdmacontrol5: RW<u32>,
  /// [860]: Device DMA Channel Status Register (n = 5)
  pub devdmastatus5: RW<u32>,
  /// [864]: Device DMA Channel Next Descriptor Address Register (n = 6)
  pub devdmanxtdsc6: RW<u32>,
  /// [868]: Device DMA Channel Address Register (n = 6)
  pub devdmaaddress6: RW<u32>,
  /// [872]: Device DMA Channel Control Register (n = 6)
  pub devdmacontrol6: RW<u32>,
  /// [876]: Device DMA Channel Status Register (n = 6)
  pub devdmastatus6: RW<u32>,
  /// [880]: Device DMA Channel Next Descriptor Address Register (n = 7)
  pub devdmanxtdsc7: RW<u32>,
  /// [884]: Device DMA Channel Address Register (n = 7)
  pub devdmaaddress7: RW<u32>,
  /// [888]: Device DMA Channel Control Register (n = 7)
  pub devdmacontrol7: RW<u32>,
  /// [892]: Device DMA Channel Status Register (n = 7)
  pub devdmastatus7: RW<u32>,
  reserved0x380: u32,
  reserved0x384: u32,
  reserved0x388: u32,
  reserved0x38c: u32,
  reserved0x390: u32,
  reserved0x394: u32,
  reserved0x398: u32,
  reserved0x39c: u32,
  reserved0x3a0: u32,
  reserved0x3a4: u32,
  reserved0x3a8: u32,
  reserved0x3ac: u32,
  reserved0x3b0: u32,
  reserved0x3b4: u32,
  reserved0x3b8: u32,
  reserved0x3bc: u32,
  reserved0x3c0: u32,
  reserved0x3c4: u32,
  reserved0x3c8: u32,
  reserved0x3cc: u32,
  reserved0x3d0: u32,
  reserved0x3d4: u32,
  reserved0x3d8: u32,
  reserved0x3dc: u32,
  reserved0x3e0: u32,
  reserved0x3e4: u32,
  reserved0x3e8: u32,
  reserved0x3ec: u32,
  reserved0x3f0: u32,
  reserved0x3f4: u32,
  reserved0x3f8: u32,
  reserved0x3fc: u32,
  /// [1024]: Host General Control Register
  pub hstctrl: RW<u32>,
  /// [1028]: Host Global Interrupt Status Register
  pub hstisr: RO<u32>,
  /// [1032]: Host Global Interrupt Clear Register
  pub hsticr: WO<u32>,
  /// [1036]: Host Global Interrupt Set Register
  pub hstifr: WO<u32>,
  /// [1040]: Host Global Interrupt Mask Register
  pub hstimr: RO<u32>,
  /// [1044]: Host Global Interrupt Disable Register
  pub hstidr: WO<u32>,
  /// [1048]: Host Global Interrupt Enable Register
  pub hstier: WO<u32>,
  /// [1052]: Host Pipe Register
  pub hstpip: RW<u32>,
  /// [1056]: Host Frame Number Register
  pub hstfnum: RW<u32>,
  /// [1060]: Host Address 1 Register
  pub hstaddr1: RW<u32>,
  /// [1064]: Host Address 2 Register
  pub hstaddr2: RW<u32>,
  /// [1068]: Host Address 3 Register
  pub hstaddr3: RW<u32>,
  reserved0x430: u32,
  reserved0x434: u32,
  reserved0x438: u32,
  reserved0x43c: u32,
  reserved0x440: u32,
  reserved0x444: u32,
  reserved0x448: u32,
  reserved0x44c: u32,
  reserved0x450: u32,
  reserved0x454: u32,
  reserved0x458: u32,
  reserved0x45c: u32,
  reserved0x460: u32,
  reserved0x464: u32,
  reserved0x468: u32,
  reserved0x46c: u32,
  reserved0x470: u32,
  reserved0x474: u32,
  reserved0x478: u32,
  reserved0x47c: u32,
  reserved0x480: u32,
  reserved0x484: u32,
  reserved0x488: u32,
  reserved0x48c: u32,
  reserved0x490: u32,
  reserved0x494: u32,
  reserved0x498: u32,
  reserved0x49c: u32,
  reserved0x4a0: u32,
  reserved0x4a4: u32,
  reserved0x4a8: u32,
  reserved0x4ac: u32,
  reserved0x4b0: u32,
  reserved0x4b4: u32,
  reserved0x4b8: u32,
  reserved0x4bc: u32,
  reserved0x4c0: u32,
  reserved0x4c4: u32,
  reserved0x4c8: u32,
  reserved0x4cc: u32,
  reserved0x4d0: u32,
  reserved0x4d4: u32,
  reserved0x4d8: u32,
  reserved0x4dc: u32,
  reserved0x4e0: u32,
  reserved0x4e4: u32,
  reserved0x4e8: u32,
  reserved0x4ec: u32,
  reserved0x4f0: u32,
  reserved0x4f4: u32,
  reserved0x4f8: u32,
  reserved0x4fc: u32,
  /// [1280]: Host Pipe Configuration Register (n = 0)
  pub hstpipcfgs: RW<u32>,
  reserved0x504: u32,
  reserved0x508: u32,
  reserved0x50c: u32,
  reserved0x510: u32,
  reserved0x514: u32,
  reserved0x518: u32,
  reserved0x51c: u32,
  reserved0x520: u32,
  reserved0x524: u32,
  reserved0x528: u32,
  reserved0x52c: u32,
  /// [1328]: Host Pipe Status Register (n = 0)
  pub hstpipisrs: RO<u32>,
  reserved0x534: u32,
  reserved0x538: u32,
  reserved0x53c: u32,
  reserved0x540: u32,
  reserved0x544: u32,
  reserved0x548: u32,
  reserved0x54c: u32,
  reserved0x550: u32,
  reserved0x554: u32,
  reserved0x558: u32,
  reserved0x55c: u32,
  /// [1376]: Host Pipe Clear Register (n = 0)
  pub hstpipicrs: WO<u32>,
  reserved0x564: u32,
  reserved0x568: u32,
  reserved0x56c: u32,
  reserved0x570: u32,
  reserved0x574: u32,
  reserved0x578: u32,
  reserved0x57c: u32,
  reserved0x580: u32,
  reserved0x584: u32,
  reserved0x588: u32,
  reserved0x58c: u32,
  /// [1424]: Host Pipe Set Register (n = 0)
  pub hstpipifrs: WO<u32>,
  reserved0x594: u32,
  reserved0x598: u32,
  reserved0x59c: u32,
  reserved0x5a0: u32,
  reserved0x5a4: u32,
  reserved0x5a8: u32,
  reserved0x5ac: u32,
  reserved0x5b0: u32,
  reserved0x5b4: u32,
  reserved0x5b8: u32,
  reserved0x5bc: u32,
  /// [1472]: Host Pipe Mask Register (n = 0)
  pub hstpipimrs: RO<u32>,
  reserved0x5c4: u32,
  reserved0x5c8: u32,
  reserved0x5cc: u32,
  reserved0x5d0: u32,
  reserved0x5d4: u32,
  reserved0x5d8: u32,
  reserved0x5dc: u32,
  reserved0x5e0: u32,
  reserved0x5e4: u32,
  reserved0x5e8: u32,
  reserved0x5ec: u32,
  /// [1520]: Host Pipe Enable Register (n = 0)
  pub hstpipiers: WO<u32>,
  reserved0x5f4: u32,
  reserved0x5f8: u32,
  reserved0x5fc: u32,
  reserved0x600: u32,
  reserved0x604: u32,
  reserved0x608: u32,
  reserved0x60c: u32,
  reserved0x610: u32,
  reserved0x614: u32,
  reserved0x618: u32,
  reserved0x61c: u32,
  /// [1568]: Host Pipe Disable Register (n = 0)
  pub hstpipidrs: WO<u32>,
  reserved0x624: u32,
  reserved0x628: u32,
  reserved0x62c: u32,
  reserved0x630: u32,
  reserved0x634: u32,
  reserved0x638: u32,
  reserved0x63c: u32,
  reserved0x640: u32,
  reserved0x644: u32,
  reserved0x648: u32,
  reserved0x64c: u32,
  /// [1616]: Host Pipe IN Request Register (n = 0)
  pub hstpipinrqs: RW<u32>,
  reserved0x654: u32,
  reserved0x658: u32,
  reserved0x65c: u32,
  reserved0x660: u32,
  reserved0x664: u32,
  reserved0x668: u32,
  reserved0x66c: u32,
  reserved0x670: u32,
  reserved0x674: u32,
  reserved0x678: u32,
  reserved0x67c: u32,
  /// [1664]: Host Pipe Error Register (n = 0)
  pub hstpiperrs: RW<u32>,
  reserved0x684: u32,
  reserved0x688: u32,
  reserved0x68c: u32,
  reserved0x690: u32,
  reserved0x694: u32,
  reserved0x698: u32,
  reserved0x69c: u32,
  reserved0x6a0: u32,
  reserved0x6a4: u32,
  reserved0x6a8: u32,
  reserved0x6ac: u32,
  reserved0x6b0: u32,
  reserved0x6b4: u32,
  reserved0x6b8: u32,
  reserved0x6bc: u32,
  reserved0x6c0: u32,
  reserved0x6c4: u32,
  reserved0x6c8: u32,
  reserved0x6cc: u32,
  reserved0x6d0: u32,
  reserved0x6d4: u32,
  reserved0x6d8: u32,
  reserved0x6dc: u32,
  reserved0x6e0: u32,
  reserved0x6e4: u32,
  reserved0x6e8: u32,
  reserved0x6ec: u32,
  reserved0x6f0: u32,
  reserved0x6f4: u32,
  reserved0x6f8: u32,
  reserved0x6fc: u32,
  reserved0x700: u32,
  reserved0x704: u32,
  reserved0x708: u32,
  reserved0x70c: u32,
  /// [1808]: Host DMA Channel Next Descriptor Address Register (n = 1)
  pub hstdmanxtdsc1: RW<u32>,
  /// [1812]: Host DMA Channel Address Register (n = 1)
  pub hstdmaaddress1: RW<u32>,
  /// [1816]: Host DMA Channel Control Register (n = 1)
  pub hstdmacontrol1: RW<u32>,
  /// [1820]: Host DMA Channel Status Register (n = 1)
  pub hstdmastatus1: RW<u32>,
  /// [1824]: Host DMA Channel Next Descriptor Address Register (n = 2)
  pub hstdmanxtdsc2: RW<u32>,
  /// [1828]: Host DMA Channel Address Register (n = 2)
  pub hstdmaaddress2: RW<u32>,
  /// [1832]: Host DMA Channel Control Register (n = 2)
  pub hstdmacontrol2: RW<u32>,
  /// [1836]: Host DMA Channel Status Register (n = 2)
  pub hstdmastatus2: RW<u32>,
  /// [1840]: Host DMA Channel Next Descriptor Address Register (n = 3)
  pub hstdmanxtdsc3: RW<u32>,
  /// [1844]: Host DMA Channel Address Register (n = 3)
  pub hstdmaaddress3: RW<u32>,
  /// [1848]: Host DMA Channel Control Register (n = 3)
  pub hstdmacontrol3: RW<u32>,
  /// [1852]: Host DMA Channel Status Register (n = 3)
  pub hstdmastatus3: RW<u32>,
  /// [1856]: Host DMA Channel Next Descriptor Address Register (n = 4)
  pub hstdmanxtdsc4: RW<u32>,
  /// [1860]: Host DMA Channel Address Register (n = 4)
  pub hstdmaaddress4: RW<u32>,
  /// [1864]: Host DMA Channel Control Register (n = 4)
  pub hstdmacontrol4: RW<u32>,
  /// [1868]: Host DMA Channel Status Register (n = 4)
  pub hstdmastatus4: RW<u32>,
  /// [1872]: Host DMA Channel Next Descriptor Address Register (n = 5)
  pub hstdmanxtdsc5: RW<u32>,
  /// [1876]: Host DMA Channel Address Register (n = 5)
  pub hstdmaaddress5: RW<u32>,
  /// [1880]: Host DMA Channel Control Register (n = 5)
  pub hstdmacontrol5: RW<u32>,
  /// [1884]: Host DMA Channel Status Register (n = 5)
  pub hstdmastatus5: RW<u32>,
  /// [1888]: Host DMA Channel Next Descriptor Address Register (n = 6)
  pub hstdmanxtdsc6: RW<u32>,
  /// [1892]: Host DMA Channel Address Register (n = 6)
  pub hstdmaaddress6: RW<u32>,
  /// [1896]: Host DMA Channel Control Register (n = 6)
  pub hstdmacontrol6: RW<u32>,
  /// [1900]: Host DMA Channel Status Register (n = 6)
  pub hstdmastatus6: RW<u32>,
  /// [1904]: Host DMA Channel Next Descriptor Address Register (n = 7)
  pub hstdmanxtdsc7: RW<u32>,
  /// [1908]: Host DMA Channel Address Register (n = 7)
  pub hstdmaaddress7: RW<u32>,
  /// [1912]: Host DMA Channel Control Register (n = 7)
  pub hstdmacontrol7: RW<u32>,
  /// [1916]: Host DMA Channel Status Register (n = 7)
  pub hstdmastatus7: RW<u32>,
  reserved0x780: u32,
  reserved0x784: u32,
  reserved0x788: u32,
  reserved0x78c: u32,
  reserved0x790: u32,
  reserved0x794: u32,
  reserved0x798: u32,
  reserved0x79c: u32,
  reserved0x7a0: u32,
  reserved0x7a4: u32,
  reserved0x7a8: u32,
  reserved0x7ac: u32,
  reserved0x7b0: u32,
  reserved0x7b4: u32,
  reserved0x7b8: u32,
  reserved0x7bc: u32,
  reserved0x7c0: u32,
  reserved0x7c4: u32,
  reserved0x7c8: u32,
  reserved0x7cc: u32,
  reserved0x7d0: u32,
  reserved0x7d4: u32,
  reserved0x7d8: u32,
  reserved0x7dc: u32,
  reserved0x7e0: u32,
  reserved0x7e4: u32,
  reserved0x7e8: u32,
  reserved0x7ec: u32,
  reserved0x7f0: u32,
  reserved0x7f4: u32,
  reserved0x7f8: u32,
  reserved0x7fc: u32,
  /// [2048]: General Control Register
  pub ctrl: RW<u32>,
  /// [2052]: General Status Register
  pub sr: RO<u32>,
  /// [2056]: General Status Clear Register
  pub scr: WO<u32>,
  /// [2060]: General Status Set Register
  pub sfr: WO<u32>,
  reserved0x810: u32,
  reserved0x814: u32,
  reserved0x818: u32,
  reserved0x81c: u32,
  reserved0x820: u32,
  reserved0x824: u32,
  reserved0x828: u32,
  /// [2092]: General Finite State Machine Register
  pub fsm: RO<u32>,
}

pub fn uotghs() -> *mut UOTGHS {
  1074446336 as *mut UOTGHS
}

#[repr(C)]
/// Ethernet MAC 10/100
pub struct EMAC {
  /// [0]: Network Control Register
  pub ncr: RW<u32>,
  /// [4]: Network Configuration Register
  pub ncfgr: RW<u32>,
  /// [8]: Network Status Register
  pub nsr: RO<u32>,
  reserved0xc: u32,
  reserved0x10: u32,
  /// [20]: Transmit Status Register
  pub tsr: RW<u32>,
  /// [24]: Receive Buffer Queue Pointer Register
  pub rbqp: RW<u32>,
  /// [28]: Transmit Buffer Queue Pointer Register
  pub tbqp: RW<u32>,
  /// [32]: Receive Status Register
  pub rsr: RW<u32>,
  /// [36]: Interrupt Status Register
  pub isr: RW<u32>,
  /// [40]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [44]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [48]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [52]: Phy Maintenance Register
  pub man: RW<u32>,
  /// [56]: Pause Time Register
  pub ptr: RW<u32>,
  /// [60]: Pause Frames Received Register
  pub pfr: RW<u32>,
  /// [64]: Frames Transmitted Ok Register
  pub fto: RW<u32>,
  /// [68]: Single Collision Frames Register
  pub scf: RW<u32>,
  /// [72]: Multiple Collision Frames Register
  pub mcf: RW<u32>,
  /// [76]: Frames Received Ok Register
  pub fro: RW<u32>,
  /// [80]: Frame Check Sequence Errors Register
  pub fcse: RW<u32>,
  /// [84]: Alignment Errors Register
  pub ale: RW<u32>,
  /// [88]: Deferred Transmission Frames Register
  pub dtf: RW<u32>,
  /// [92]: Late Collisions Register
  pub lcol: RW<u32>,
  /// [96]: Excessive Collisions Register
  pub ecol: RW<u32>,
  /// [100]: Transmit Underrun Errors Register
  pub tund: RW<u32>,
  /// [104]: Carrier Sense Errors Register
  pub cse: RW<u32>,
  /// [108]: Receive Resource Errors Register
  pub rre: RW<u32>,
  /// [112]: Receive Overrun Errors Register
  pub rov: RW<u32>,
  /// [116]: Receive Symbol Errors Register
  pub rse: RW<u32>,
  /// [120]: Excessive Length Errors Register
  pub ele: RW<u32>,
  /// [124]: Receive Jabbers Register
  pub rja: RW<u32>,
  /// [128]: Undersize Frames Register
  pub usf: RW<u32>,
  /// [132]: SQE Test Errors Register
  pub ste: RW<u32>,
  /// [136]: Received Length Field Mismatch Register
  pub rle: RW<u32>,
  reserved0x8c: u32,
  /// [144]: Hash Register Bottom [31:0] Register
  pub hrb: RW<u32>,
  /// [148]: Hash Register Top [63:32] Register
  pub hrt: RW<u32>,
  /// [152]: Specific Address 1 Bottom Register
  pub sa1b: RW<u32>,
  /// [156]: Specific Address 1 Top Register
  pub sa1t: RW<u32>,
  /// [160]: Specific Address 2 Bottom Register
  pub sa2b: RW<u32>,
  /// [164]: Specific Address 2 Top Register
  pub sa2t: RW<u32>,
  /// [168]: Specific Address 3 Bottom Register
  pub sa3b: RW<u32>,
  /// [172]: Specific Address 3 Top Register
  pub sa3t: RW<u32>,
  /// [176]: Specific Address 4 Bottom Register
  pub sa4b: RW<u32>,
  /// [180]: Specific Address 4 Top Register
  pub sa4t: RW<u32>,
  /// [184]: Type ID Checking Register
  pub tid: RW<u32>,
  reserved0xbc: u32,
  /// [192]: User Input/Output Register
  pub usrio: RW<u32>,
}

pub fn emac() -> *mut EMAC {
  1074462720 as *mut EMAC
}

#[repr(C)]
/// Controller Area Network 0
pub struct CAN0 {
  /// [0]: Mode Register
  pub mr: RW<u32>,
  /// [4]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [8]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [12]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [16]: Status Register
  pub sr: RO<u32>,
  /// [20]: Baudrate Register
  pub br: RW<u32>,
  /// [24]: Timer Register
  pub tim: RO<u32>,
  /// [28]: Timestamp Register
  pub timestp: RO<u32>,
  /// [32]: Error Counter Register
  pub ecr: RO<u32>,
  /// [36]: Transfer Command Register
  pub tcr: WO<u32>,
  /// [40]: Abort Command Register
  pub acr: WO<u32>,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  reserved0x100: u32,
  reserved0x104: u32,
  reserved0x108: u32,
  reserved0x10c: u32,
  reserved0x110: u32,
  reserved0x114: u32,
  reserved0x118: u32,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  reserved0x180: u32,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  reserved0x190: u32,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  reserved0x1e4: u32,
  reserved0x1e8: u32,
  reserved0x1ec: u32,
  reserved0x1f0: u32,
  reserved0x1f4: u32,
  reserved0x1f8: u32,
  reserved0x1fc: u32,
  /// [512]: Mailbox Mode Register (MB = 0)
  pub mmr0: RW<u32>,
  /// [516]: Mailbox Acceptance Mask Register (MB = 0)
  pub mam0: RW<u32>,
  /// [520]: Mailbox ID Register (MB = 0)
  pub mid0: RW<u32>,
  /// [524]: Mailbox Family ID Register (MB = 0)
  pub mfid0: RO<u32>,
  /// [528]: Mailbox Status Register (MB = 0)
  pub msr0: RO<u32>,
  /// [532]: Mailbox Data Low Register (MB = 0)
  pub mdl0: RW<u32>,
  /// [536]: Mailbox Data High Register (MB = 0)
  pub mdh0: RW<u32>,
  /// [540]: Mailbox Control Register (MB = 0)
  pub mcr0: WO<u32>,
  /// [544]: Mailbox Mode Register (MB = 1)
  pub mmr1: RW<u32>,
  /// [548]: Mailbox Acceptance Mask Register (MB = 1)
  pub mam1: RW<u32>,
  /// [552]: Mailbox ID Register (MB = 1)
  pub mid1: RW<u32>,
  /// [556]: Mailbox Family ID Register (MB = 1)
  pub mfid1: RO<u32>,
  /// [560]: Mailbox Status Register (MB = 1)
  pub msr1: RO<u32>,
  /// [564]: Mailbox Data Low Register (MB = 1)
  pub mdl1: RW<u32>,
  /// [568]: Mailbox Data High Register (MB = 1)
  pub mdh1: RW<u32>,
  /// [572]: Mailbox Control Register (MB = 1)
  pub mcr1: WO<u32>,
  /// [576]: Mailbox Mode Register (MB = 2)
  pub mmr2: RW<u32>,
  /// [580]: Mailbox Acceptance Mask Register (MB = 2)
  pub mam2: RW<u32>,
  /// [584]: Mailbox ID Register (MB = 2)
  pub mid2: RW<u32>,
  /// [588]: Mailbox Family ID Register (MB = 2)
  pub mfid2: RO<u32>,
  /// [592]: Mailbox Status Register (MB = 2)
  pub msr2: RO<u32>,
  /// [596]: Mailbox Data Low Register (MB = 2)
  pub mdl2: RW<u32>,
  /// [600]: Mailbox Data High Register (MB = 2)
  pub mdh2: RW<u32>,
  /// [604]: Mailbox Control Register (MB = 2)
  pub mcr2: WO<u32>,
  /// [608]: Mailbox Mode Register (MB = 3)
  pub mmr3: RW<u32>,
  /// [612]: Mailbox Acceptance Mask Register (MB = 3)
  pub mam3: RW<u32>,
  /// [616]: Mailbox ID Register (MB = 3)
  pub mid3: RW<u32>,
  /// [620]: Mailbox Family ID Register (MB = 3)
  pub mfid3: RO<u32>,
  /// [624]: Mailbox Status Register (MB = 3)
  pub msr3: RO<u32>,
  /// [628]: Mailbox Data Low Register (MB = 3)
  pub mdl3: RW<u32>,
  /// [632]: Mailbox Data High Register (MB = 3)
  pub mdh3: RW<u32>,
  /// [636]: Mailbox Control Register (MB = 3)
  pub mcr3: WO<u32>,
  /// [640]: Mailbox Mode Register (MB = 4)
  pub mmr4: RW<u32>,
  /// [644]: Mailbox Acceptance Mask Register (MB = 4)
  pub mam4: RW<u32>,
  /// [648]: Mailbox ID Register (MB = 4)
  pub mid4: RW<u32>,
  /// [652]: Mailbox Family ID Register (MB = 4)
  pub mfid4: RO<u32>,
  /// [656]: Mailbox Status Register (MB = 4)
  pub msr4: RO<u32>,
  /// [660]: Mailbox Data Low Register (MB = 4)
  pub mdl4: RW<u32>,
  /// [664]: Mailbox Data High Register (MB = 4)
  pub mdh4: RW<u32>,
  /// [668]: Mailbox Control Register (MB = 4)
  pub mcr4: WO<u32>,
  /// [672]: Mailbox Mode Register (MB = 5)
  pub mmr5: RW<u32>,
  /// [676]: Mailbox Acceptance Mask Register (MB = 5)
  pub mam5: RW<u32>,
  /// [680]: Mailbox ID Register (MB = 5)
  pub mid5: RW<u32>,
  /// [684]: Mailbox Family ID Register (MB = 5)
  pub mfid5: RO<u32>,
  /// [688]: Mailbox Status Register (MB = 5)
  pub msr5: RO<u32>,
  /// [692]: Mailbox Data Low Register (MB = 5)
  pub mdl5: RW<u32>,
  /// [696]: Mailbox Data High Register (MB = 5)
  pub mdh5: RW<u32>,
  /// [700]: Mailbox Control Register (MB = 5)
  pub mcr5: WO<u32>,
  /// [704]: Mailbox Mode Register (MB = 6)
  pub mmr6: RW<u32>,
  /// [708]: Mailbox Acceptance Mask Register (MB = 6)
  pub mam6: RW<u32>,
  /// [712]: Mailbox ID Register (MB = 6)
  pub mid6: RW<u32>,
  /// [716]: Mailbox Family ID Register (MB = 6)
  pub mfid6: RO<u32>,
  /// [720]: Mailbox Status Register (MB = 6)
  pub msr6: RO<u32>,
  /// [724]: Mailbox Data Low Register (MB = 6)
  pub mdl6: RW<u32>,
  /// [728]: Mailbox Data High Register (MB = 6)
  pub mdh6: RW<u32>,
  /// [732]: Mailbox Control Register (MB = 6)
  pub mcr6: WO<u32>,
  /// [736]: Mailbox Mode Register (MB = 7)
  pub mmr7: RW<u32>,
  /// [740]: Mailbox Acceptance Mask Register (MB = 7)
  pub mam7: RW<u32>,
  /// [744]: Mailbox ID Register (MB = 7)
  pub mid7: RW<u32>,
  /// [748]: Mailbox Family ID Register (MB = 7)
  pub mfid7: RO<u32>,
  /// [752]: Mailbox Status Register (MB = 7)
  pub msr7: RO<u32>,
  /// [756]: Mailbox Data Low Register (MB = 7)
  pub mdl7: RW<u32>,
  /// [760]: Mailbox Data High Register (MB = 7)
  pub mdh7: RW<u32>,
  /// [764]: Mailbox Control Register (MB = 7)
  pub mcr7: WO<u32>,
}

pub fn can0() -> *mut CAN0 {
  1074479104 as *mut CAN0
}

#[repr(C)]
/// Controller Area Network 1
pub struct CAN1 {
  /// [0]: Mode Register
  pub mr: RW<u32>,
  /// [4]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [8]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [12]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [16]: Status Register
  pub sr: RO<u32>,
  /// [20]: Baudrate Register
  pub br: RW<u32>,
  /// [24]: Timer Register
  pub tim: RO<u32>,
  /// [28]: Timestamp Register
  pub timestp: RO<u32>,
  /// [32]: Error Counter Register
  pub ecr: RO<u32>,
  /// [36]: Transfer Command Register
  pub tcr: WO<u32>,
  /// [40]: Abort Command Register
  pub acr: WO<u32>,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  reserved0x100: u32,
  reserved0x104: u32,
  reserved0x108: u32,
  reserved0x10c: u32,
  reserved0x110: u32,
  reserved0x114: u32,
  reserved0x118: u32,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  reserved0x180: u32,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  reserved0x190: u32,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  reserved0x1e4: u32,
  reserved0x1e8: u32,
  reserved0x1ec: u32,
  reserved0x1f0: u32,
  reserved0x1f4: u32,
  reserved0x1f8: u32,
  reserved0x1fc: u32,
  /// [512]: Mailbox Mode Register (MB = 0)
  pub mmr0: RW<u32>,
  /// [516]: Mailbox Acceptance Mask Register (MB = 0)
  pub mam0: RW<u32>,
  /// [520]: Mailbox ID Register (MB = 0)
  pub mid0: RW<u32>,
  /// [524]: Mailbox Family ID Register (MB = 0)
  pub mfid0: RO<u32>,
  /// [528]: Mailbox Status Register (MB = 0)
  pub msr0: RO<u32>,
  /// [532]: Mailbox Data Low Register (MB = 0)
  pub mdl0: RW<u32>,
  /// [536]: Mailbox Data High Register (MB = 0)
  pub mdh0: RW<u32>,
  /// [540]: Mailbox Control Register (MB = 0)
  pub mcr0: WO<u32>,
  /// [544]: Mailbox Mode Register (MB = 1)
  pub mmr1: RW<u32>,
  /// [548]: Mailbox Acceptance Mask Register (MB = 1)
  pub mam1: RW<u32>,
  /// [552]: Mailbox ID Register (MB = 1)
  pub mid1: RW<u32>,
  /// [556]: Mailbox Family ID Register (MB = 1)
  pub mfid1: RO<u32>,
  /// [560]: Mailbox Status Register (MB = 1)
  pub msr1: RO<u32>,
  /// [564]: Mailbox Data Low Register (MB = 1)
  pub mdl1: RW<u32>,
  /// [568]: Mailbox Data High Register (MB = 1)
  pub mdh1: RW<u32>,
  /// [572]: Mailbox Control Register (MB = 1)
  pub mcr1: WO<u32>,
  /// [576]: Mailbox Mode Register (MB = 2)
  pub mmr2: RW<u32>,
  /// [580]: Mailbox Acceptance Mask Register (MB = 2)
  pub mam2: RW<u32>,
  /// [584]: Mailbox ID Register (MB = 2)
  pub mid2: RW<u32>,
  /// [588]: Mailbox Family ID Register (MB = 2)
  pub mfid2: RO<u32>,
  /// [592]: Mailbox Status Register (MB = 2)
  pub msr2: RO<u32>,
  /// [596]: Mailbox Data Low Register (MB = 2)
  pub mdl2: RW<u32>,
  /// [600]: Mailbox Data High Register (MB = 2)
  pub mdh2: RW<u32>,
  /// [604]: Mailbox Control Register (MB = 2)
  pub mcr2: WO<u32>,
  /// [608]: Mailbox Mode Register (MB = 3)
  pub mmr3: RW<u32>,
  /// [612]: Mailbox Acceptance Mask Register (MB = 3)
  pub mam3: RW<u32>,
  /// [616]: Mailbox ID Register (MB = 3)
  pub mid3: RW<u32>,
  /// [620]: Mailbox Family ID Register (MB = 3)
  pub mfid3: RO<u32>,
  /// [624]: Mailbox Status Register (MB = 3)
  pub msr3: RO<u32>,
  /// [628]: Mailbox Data Low Register (MB = 3)
  pub mdl3: RW<u32>,
  /// [632]: Mailbox Data High Register (MB = 3)
  pub mdh3: RW<u32>,
  /// [636]: Mailbox Control Register (MB = 3)
  pub mcr3: WO<u32>,
  /// [640]: Mailbox Mode Register (MB = 4)
  pub mmr4: RW<u32>,
  /// [644]: Mailbox Acceptance Mask Register (MB = 4)
  pub mam4: RW<u32>,
  /// [648]: Mailbox ID Register (MB = 4)
  pub mid4: RW<u32>,
  /// [652]: Mailbox Family ID Register (MB = 4)
  pub mfid4: RO<u32>,
  /// [656]: Mailbox Status Register (MB = 4)
  pub msr4: RO<u32>,
  /// [660]: Mailbox Data Low Register (MB = 4)
  pub mdl4: RW<u32>,
  /// [664]: Mailbox Data High Register (MB = 4)
  pub mdh4: RW<u32>,
  /// [668]: Mailbox Control Register (MB = 4)
  pub mcr4: WO<u32>,
  /// [672]: Mailbox Mode Register (MB = 5)
  pub mmr5: RW<u32>,
  /// [676]: Mailbox Acceptance Mask Register (MB = 5)
  pub mam5: RW<u32>,
  /// [680]: Mailbox ID Register (MB = 5)
  pub mid5: RW<u32>,
  /// [684]: Mailbox Family ID Register (MB = 5)
  pub mfid5: RO<u32>,
  /// [688]: Mailbox Status Register (MB = 5)
  pub msr5: RO<u32>,
  /// [692]: Mailbox Data Low Register (MB = 5)
  pub mdl5: RW<u32>,
  /// [696]: Mailbox Data High Register (MB = 5)
  pub mdh5: RW<u32>,
  /// [700]: Mailbox Control Register (MB = 5)
  pub mcr5: WO<u32>,
  /// [704]: Mailbox Mode Register (MB = 6)
  pub mmr6: RW<u32>,
  /// [708]: Mailbox Acceptance Mask Register (MB = 6)
  pub mam6: RW<u32>,
  /// [712]: Mailbox ID Register (MB = 6)
  pub mid6: RW<u32>,
  /// [716]: Mailbox Family ID Register (MB = 6)
  pub mfid6: RO<u32>,
  /// [720]: Mailbox Status Register (MB = 6)
  pub msr6: RO<u32>,
  /// [724]: Mailbox Data Low Register (MB = 6)
  pub mdl6: RW<u32>,
  /// [728]: Mailbox Data High Register (MB = 6)
  pub mdh6: RW<u32>,
  /// [732]: Mailbox Control Register (MB = 6)
  pub mcr6: WO<u32>,
  /// [736]: Mailbox Mode Register (MB = 7)
  pub mmr7: RW<u32>,
  /// [740]: Mailbox Acceptance Mask Register (MB = 7)
  pub mam7: RW<u32>,
  /// [744]: Mailbox ID Register (MB = 7)
  pub mid7: RW<u32>,
  /// [748]: Mailbox Family ID Register (MB = 7)
  pub mfid7: RO<u32>,
  /// [752]: Mailbox Status Register (MB = 7)
  pub msr7: RO<u32>,
  /// [756]: Mailbox Data Low Register (MB = 7)
  pub mdl7: RW<u32>,
  /// [760]: Mailbox Data High Register (MB = 7)
  pub mdh7: RW<u32>,
  /// [764]: Mailbox Control Register (MB = 7)
  pub mcr7: WO<u32>,
}

pub fn can1() -> *mut CAN1 {
  1074495488 as *mut CAN1
}

#[repr(C)]
/// True Random Number Generator
pub struct TRNG {
  /// [0]: Control Register
  pub cr: WO<u32>,
  reserved0x4: u32,
  reserved0x8: u32,
  reserved0xc: u32,
  /// [16]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [20]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [24]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [28]: Interrupt Status Register
  pub isr: RO<u32>,
  reserved0x20: u32,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  /// [80]: Output Data Register
  pub odata: RO<u32>,
}

pub fn trng() -> *mut TRNG {
  1074511872 as *mut TRNG
}

#[repr(C)]
/// Analog-to-digital Converter
pub struct ADC {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Channel Sequence Register 1
  pub seqr1: RW<u32>,
  /// [12]: Channel Sequence Register 2
  pub seqr2: RW<u32>,
  /// [16]: Channel Enable Register
  pub cher: WO<u32>,
  /// [20]: Channel Disable Register
  pub chdr: WO<u32>,
  /// [24]: Channel Status Register
  pub chsr: RO<u32>,
  reserved0x1c: u32,
  /// [32]: Last Converted Data Register
  pub lcdr: RO<u32>,
  /// [36]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [40]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [44]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [48]: Interrupt Status Register
  pub isr: RO<u32>,
  reserved0x34: u32,
  reserved0x38: u32,
  /// [60]: Overrun Status Register
  pub over: RO<u32>,
  /// [64]: Extended Mode Register
  pub emr: RW<u32>,
  /// [68]: Compare Window Register
  pub cwr: RW<u32>,
  /// [72]: Channel Gain Register
  pub cgr: RW<u32>,
  /// [76]: Channel Offset Register
  pub cor: RW<u32>,
  /// [80]: Channel Data Register
  pub cdrs: RO<u32>,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  /// [148]: Analog Control Register
  pub acr: RW<u32>,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Receive Pointer Register
  pub rpr: RW<u32>,
  /// [260]: Receive Counter Register
  pub rcr: RW<u32>,
  reserved0x108: u32,
  reserved0x10c: u32,
  /// [272]: Receive Next Pointer Register
  pub rnpr: RW<u32>,
  /// [276]: Receive Next Counter Register
  pub rncr: RW<u32>,
  reserved0x118: u32,
  reserved0x11c: u32,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn adc() -> *mut ADC {
  1074528256 as *mut ADC
}

#[repr(C)]
/// DMA Controller
pub struct DMAC {
  /// [0]: DMAC Global Configuration Register
  pub gcfg: RW<u32>,
  /// [4]: DMAC Enable Register
  pub en: RW<u32>,
  /// [8]: DMAC Software Single Request Register
  pub sreq: RW<u32>,
  /// [12]: DMAC Software Chunk Transfer Request Register
  pub creq: RW<u32>,
  /// [16]: DMAC Software Last Transfer Flag Register
  pub last: RW<u32>,
  reserved0x14: u32,
  /// [24]: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register.
  pub ebcier: WO<u32>,
  /// [28]: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register.
  pub ebcidr: WO<u32>,
  /// [32]: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register.
  pub ebcimr: RO<u32>,
  /// [36]: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register.
  pub ebcisr: RO<u32>,
  /// [40]: DMAC Channel Handler Enable Register
  pub cher: WO<u32>,
  /// [44]: DMAC Channel Handler Disable Register
  pub chdr: WO<u32>,
  /// [48]: DMAC Channel Handler Status Register
  pub chsr: RO<u32>,
  reserved0x34: u32,
  reserved0x38: u32,
  /// [60]: DMAC Channel Source Address Register (ch_num = 0)
  pub saddr0: RW<u32>,
  /// [64]: DMAC Channel Destination Address Register (ch_num = 0)
  pub daddr0: RW<u32>,
  /// [68]: DMAC Channel Descriptor Address Register (ch_num = 0)
  pub dscr0: RW<u32>,
  /// [72]: DMAC Channel Control A Register (ch_num = 0)
  pub ctrla0: RW<u32>,
  /// [76]: DMAC Channel Control B Register (ch_num = 0)
  pub ctrlb0: RW<u32>,
  /// [80]: DMAC Channel Configuration Register (ch_num = 0)
  pub cfg0: RW<u32>,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  /// [100]: DMAC Channel Source Address Register (ch_num = 1)
  pub saddr1: RW<u32>,
  /// [104]: DMAC Channel Destination Address Register (ch_num = 1)
  pub daddr1: RW<u32>,
  /// [108]: DMAC Channel Descriptor Address Register (ch_num = 1)
  pub dscr1: RW<u32>,
  /// [112]: DMAC Channel Control A Register (ch_num = 1)
  pub ctrla1: RW<u32>,
  /// [116]: DMAC Channel Control B Register (ch_num = 1)
  pub ctrlb1: RW<u32>,
  /// [120]: DMAC Channel Configuration Register (ch_num = 1)
  pub cfg1: RW<u32>,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  /// [140]: DMAC Channel Source Address Register (ch_num = 2)
  pub saddr2: RW<u32>,
  /// [144]: DMAC Channel Destination Address Register (ch_num = 2)
  pub daddr2: RW<u32>,
  /// [148]: DMAC Channel Descriptor Address Register (ch_num = 2)
  pub dscr2: RW<u32>,
  /// [152]: DMAC Channel Control A Register (ch_num = 2)
  pub ctrla2: RW<u32>,
  /// [156]: DMAC Channel Control B Register (ch_num = 2)
  pub ctrlb2: RW<u32>,
  /// [160]: DMAC Channel Configuration Register (ch_num = 2)
  pub cfg2: RW<u32>,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  /// [180]: DMAC Channel Source Address Register (ch_num = 3)
  pub saddr3: RW<u32>,
  /// [184]: DMAC Channel Destination Address Register (ch_num = 3)
  pub daddr3: RW<u32>,
  /// [188]: DMAC Channel Descriptor Address Register (ch_num = 3)
  pub dscr3: RW<u32>,
  /// [192]: DMAC Channel Control A Register (ch_num = 3)
  pub ctrla3: RW<u32>,
  /// [196]: DMAC Channel Control B Register (ch_num = 3)
  pub ctrlb3: RW<u32>,
  /// [200]: DMAC Channel Configuration Register (ch_num = 3)
  pub cfg3: RW<u32>,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  /// [220]: DMAC Channel Source Address Register (ch_num = 4)
  pub saddr4: RW<u32>,
  /// [224]: DMAC Channel Destination Address Register (ch_num = 4)
  pub daddr4: RW<u32>,
  /// [228]: DMAC Channel Descriptor Address Register (ch_num = 4)
  pub dscr4: RW<u32>,
  /// [232]: DMAC Channel Control A Register (ch_num = 4)
  pub ctrla4: RW<u32>,
  /// [236]: DMAC Channel Control B Register (ch_num = 4)
  pub ctrlb4: RW<u32>,
  /// [240]: DMAC Channel Configuration Register (ch_num = 4)
  pub cfg4: RW<u32>,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  reserved0x100: u32,
  /// [260]: DMAC Channel Source Address Register (ch_num = 5)
  pub saddr5: RW<u32>,
  /// [264]: DMAC Channel Destination Address Register (ch_num = 5)
  pub daddr5: RW<u32>,
  /// [268]: DMAC Channel Descriptor Address Register (ch_num = 5)
  pub dscr5: RW<u32>,
  /// [272]: DMAC Channel Control A Register (ch_num = 5)
  pub ctrla5: RW<u32>,
  /// [276]: DMAC Channel Control B Register (ch_num = 5)
  pub ctrlb5: RW<u32>,
  /// [280]: DMAC Channel Configuration Register (ch_num = 5)
  pub cfg5: RW<u32>,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  reserved0x180: u32,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  reserved0x190: u32,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  /// [484]: DMAC Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [488]: DMAC Write Protect Status Register
  pub wpsr: RO<u32>,
}

pub fn dmac() -> *mut DMAC {
  1074544640 as *mut DMAC
}

#[repr(C)]
/// Digital-to-Analog Converter Controller
pub struct DACC {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  reserved0x8: u32,
  reserved0xc: u32,
  /// [16]: Channel Enable Register
  pub cher: WO<u32>,
  /// [20]: Channel Disable Register
  pub chdr: WO<u32>,
  /// [24]: Channel Status Register
  pub chsr: RO<u32>,
  reserved0x1c: u32,
  /// [32]: Conversion Data Register
  pub cdr: WO<u32>,
  /// [36]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [40]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [44]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [48]: Interrupt Status Register
  pub isr: RO<u32>,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  /// [148]: Analog Current Register
  pub acr: RW<u32>,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status register
  pub wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  reserved0x100: u32,
  reserved0x104: u32,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  reserved0x110: u32,
  reserved0x114: u32,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn dacc() -> *mut DACC {
  1074561024 as *mut DACC
}

#[repr(C)]
/// Static Memory Controller
pub struct SMC {
  /// [0]: SMC NFC Configuration Register
  pub cfg: RW<u32>,
  /// [4]: SMC NFC Control Register
  pub ctrl: WO<u32>,
  /// [8]: SMC NFC Status Register
  pub sr: RO<u32>,
  /// [12]: SMC NFC Interrupt Enable Register
  pub ier: WO<u32>,
  /// [16]: SMC NFC Interrupt Disable Register
  pub idr: WO<u32>,
  /// [20]: SMC NFC Interrupt Mask Register
  pub imr: RO<u32>,
  /// [24]: SMC NFC Address Cycle Zero Register
  pub addr: RW<u32>,
  /// [28]: SMC Bank Address Register
  pub bank: RW<u32>,
  /// [32]: SMC ECC Control Register
  pub ecc_ctrl: WO<u32>,
  /// [36]: SMC ECC Mode Register
  pub ecc_md: RW<u32>,
  /// [40]: SMC ECC Status 1 Register
  pub ecc_sr1: RO<u32>,
  /// [44]: SMC ECC Parity 0 Register
  pub ecc_pr0_w8bit: RO<u32>,
  /// [48]: SMC ECC parity 1 Register
  pub ecc_pr1_w8bit: RO<u32>,
  /// [52]: SMC ECC status 2 Register
  pub ecc_sr2: RO<u32>,
  /// [56]: SMC ECC parity 2 Register
  pub ecc_pr2_w8bit: RO<u32>,
  /// [60]: SMC ECC parity 3 Register
  pub ecc_pr3_w8bit: RO<u32>,
  /// [64]: SMC ECC parity 4 Register
  pub ecc_pr4_w8bit: RO<u32>,
  /// [68]: SMC ECC parity 5 Register
  pub ecc_pr5_w8bit: RO<u32>,
  /// [72]: SMC ECC parity 6 Register
  pub ecc_pr6_w8bit: RO<u32>,
  /// [76]: SMC ECC parity 7 Register
  pub ecc_pr7_w8bit: RO<u32>,
  /// [80]: SMC ECC parity 8 Register
  pub ecc_pr8: RO<u32>,
  /// [84]: SMC ECC parity 9 Register
  pub ecc_pr9: RO<u32>,
  /// [88]: SMC ECC parity 10 Register
  pub ecc_pr10: RO<u32>,
  /// [92]: SMC ECC parity 11 Register
  pub ecc_pr11: RO<u32>,
  /// [96]: SMC ECC parity 12 Register
  pub ecc_pr12: RO<u32>,
  /// [100]: SMC ECC parity 13 Register
  pub ecc_pr13: RO<u32>,
  /// [104]: SMC ECC parity 14 Register
  pub ecc_pr14: RO<u32>,
  /// [108]: SMC ECC parity 15 Register
  pub ecc_pr15: RO<u32>,
  /// [112]: SMC Setup Register (CS_number = 0)
  pub setup0: RW<u32>,
  /// [116]: SMC Pulse Register (CS_number = 0)
  pub pulse0: RW<u32>,
  /// [120]: SMC Cycle Register (CS_number = 0)
  pub cycle0: RW<u32>,
  /// [124]: SMC Timings Register (CS_number = 0)
  pub timings0: RW<u32>,
  /// [128]: SMC Mode Register (CS_number = 0)
  pub mode0: RW<u32>,
  /// [132]: SMC Setup Register (CS_number = 1)
  pub setup1: RW<u32>,
  /// [136]: SMC Pulse Register (CS_number = 1)
  pub pulse1: RW<u32>,
  /// [140]: SMC Cycle Register (CS_number = 1)
  pub cycle1: RW<u32>,
  /// [144]: SMC Timings Register (CS_number = 1)
  pub timings1: RW<u32>,
  /// [148]: SMC Mode Register (CS_number = 1)
  pub mode1: RW<u32>,
  /// [152]: SMC Setup Register (CS_number = 2)
  pub setup2: RW<u32>,
  /// [156]: SMC Pulse Register (CS_number = 2)
  pub pulse2: RW<u32>,
  /// [160]: SMC Cycle Register (CS_number = 2)
  pub cycle2: RW<u32>,
  /// [164]: SMC Timings Register (CS_number = 2)
  pub timings2: RW<u32>,
  /// [168]: SMC Mode Register (CS_number = 2)
  pub mode2: RW<u32>,
  /// [172]: SMC Setup Register (CS_number = 3)
  pub setup3: RW<u32>,
  /// [176]: SMC Pulse Register (CS_number = 3)
  pub pulse3: RW<u32>,
  /// [180]: SMC Cycle Register (CS_number = 3)
  pub cycle3: RW<u32>,
  /// [184]: SMC Timings Register (CS_number = 3)
  pub timings3: RW<u32>,
  /// [188]: SMC Mode Register (CS_number = 3)
  pub mode3: RW<u32>,
  /// [192]: SMC Setup Register (CS_number = 4)
  pub setup4: RW<u32>,
  /// [196]: SMC Pulse Register (CS_number = 4)
  pub pulse4: RW<u32>,
  /// [200]: SMC Cycle Register (CS_number = 4)
  pub cycle4: RW<u32>,
  /// [204]: SMC Timings Register (CS_number = 4)
  pub timings4: RW<u32>,
  /// [208]: SMC Mode Register (CS_number = 4)
  pub mode4: RW<u32>,
  /// [212]: SMC Setup Register (CS_number = 5)
  pub setup5: RW<u32>,
  /// [216]: SMC Pulse Register (CS_number = 5)
  pub pulse5: RW<u32>,
  /// [220]: SMC Cycle Register (CS_number = 5)
  pub cycle5: RW<u32>,
  /// [224]: SMC Timings Register (CS_number = 5)
  pub timings5: RW<u32>,
  /// [228]: SMC Mode Register (CS_number = 5)
  pub mode5: RW<u32>,
  /// [232]: SMC Setup Register (CS_number = 6)
  pub setup6: RW<u32>,
  /// [236]: SMC Pulse Register (CS_number = 6)
  pub pulse6: RW<u32>,
  /// [240]: SMC Cycle Register (CS_number = 6)
  pub cycle6: RW<u32>,
  /// [244]: SMC Timings Register (CS_number = 6)
  pub timings6: RW<u32>,
  /// [248]: SMC Mode Register (CS_number = 6)
  pub mode6: RW<u32>,
  /// [252]: SMC Setup Register (CS_number = 7)
  pub setup7: RW<u32>,
  /// [256]: SMC Pulse Register (CS_number = 7)
  pub pulse7: RW<u32>,
  /// [260]: SMC Cycle Register (CS_number = 7)
  pub cycle7: RW<u32>,
  /// [264]: SMC Timings Register (CS_number = 7)
  pub timings7: RW<u32>,
  /// [268]: SMC Mode Register (CS_number = 7)
  pub mode7: RW<u32>,
  /// [272]: SMC OCMS Register
  pub ocms: RW<u32>,
  /// [276]: SMC OCMS KEY1 Register
  pub key1: WO<u32>,
  /// [280]: SMC OCMS KEY2 Register
  pub key2: WO<u32>,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  reserved0x180: u32,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  reserved0x190: u32,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  /// [484]: Write Protection Control Register
  pub wpcr: WO<u32>,
  /// [488]: Write Protection Status Register
  pub wpsr: RO<u32>,
}

pub fn smc() -> *mut SMC {
  1074659328 as *mut SMC
}

#[repr(C)]
/// AHB Bus Matrix
pub struct MATRIX {
  /// [0]: Master Configuration Register
  pub matrix_mcfgs: RW<u32>,
  reserved0x4: u32,
  reserved0x8: u32,
  reserved0xc: u32,
  reserved0x10: u32,
  reserved0x14: u32,
  reserved0x18: u32,
  reserved0x1c: u32,
  reserved0x20: u32,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: Slave Configuration Register
  pub matrix_scfgs: RW<u32>,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: Priority Register A for Slave 0
  pub matrix_pras0: RW<u32>,
  reserved0x84: u32,
  /// [136]: Priority Register A for Slave 1
  pub matrix_pras1: RW<u32>,
  reserved0x8c: u32,
  /// [144]: Priority Register A for Slave 2
  pub matrix_pras2: RW<u32>,
  reserved0x94: u32,
  /// [152]: Priority Register A for Slave 3
  pub matrix_pras3: RW<u32>,
  reserved0x9c: u32,
  /// [160]: Priority Register A for Slave 4
  pub matrix_pras4: RW<u32>,
  reserved0xa4: u32,
  /// [168]: Priority Register A for Slave 5
  pub matrix_pras5: RW<u32>,
  reserved0xac: u32,
  /// [176]: Priority Register A for Slave 6
  pub matrix_pras6: RW<u32>,
  reserved0xb4: u32,
  /// [184]: Priority Register A for Slave 7
  pub matrix_pras7: RW<u32>,
  reserved0xbc: u32,
  /// [192]: Priority Register A for Slave 8
  pub matrix_pras8: RW<u32>,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  reserved0xe4: u32,
  reserved0xe8: u32,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Master Remap Control Register
  pub matrix_mrcr: RW<u32>,
  reserved0x104: u32,
  reserved0x108: u32,
  reserved0x10c: u32,
  reserved0x110: u32,
  /// [276]: System I/O Configuration register
  pub ccfg_sysio: RW<u32>,
  reserved0x118: u32,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  reserved0x180: u32,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  reserved0x190: u32,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  /// [484]: Write Protect Mode Register
  pub matrix_wpmr: RW<u32>,
  /// [488]: Write Protect Status Register
  pub matrix_wpsr: RO<u32>,
}

pub fn matrix() -> *mut MATRIX {
  1074660352 as *mut MATRIX
}

#[repr(C)]
/// Power Management Controller
pub struct PMC {
  /// [0]: System Clock Enable Register
  pub pmc_scer: WO<u32>,
  /// [4]: System Clock Disable Register
  pub pmc_scdr: WO<u32>,
  /// [8]: System Clock Status Register
  pub pmc_scsr: RO<u32>,
  reserved0xc: u32,
  /// [16]: Peripheral Clock Enable Register 0
  pub pmc_pcer0: WO<u32>,
  /// [20]: Peripheral Clock Disable Register 0
  pub pmc_pcdr0: WO<u32>,
  /// [24]: Peripheral Clock Status Register 0
  pub pmc_pcsr0: RO<u32>,
  /// [28]: UTMI Clock Register
  pub ckgr_uckr: RW<u32>,
  /// [32]: Main Oscillator Register
  pub ckgr_mor: RW<u32>,
  /// [36]: Main Clock Frequency Register
  pub ckgr_mcfr: RO<u32>,
  /// [40]: PLLA Register
  pub ckgr_pllar: RW<u32>,
  reserved0x2c: u32,
  /// [48]: Master Clock Register
  pub pmc_mckr: RW<u32>,
  reserved0x34: u32,
  /// [56]: USB Clock Register
  pub pmc_usb: RW<u32>,
  reserved0x3c: u32,
  /// [64]: Programmable Clock 0 Register
  pub pmc_pcks: RW<u32>,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  /// [96]: Interrupt Enable Register
  pub pmc_ier: WO<u32>,
  /// [100]: Interrupt Disable Register
  pub pmc_idr: WO<u32>,
  /// [104]: Status Register
  pub pmc_sr: RO<u32>,
  /// [108]: Interrupt Mask Register
  pub pmc_imr: RO<u32>,
  /// [112]: Fast Startup Mode Register
  pub pmc_fsmr: RW<u32>,
  /// [116]: Fast Startup Polarity Register
  pub pmc_fspr: RW<u32>,
  /// [120]: Fault Output Clear Register
  pub pmc_focr: WO<u32>,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub pmc_wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub pmc_wpsr: RO<u32>,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Peripheral Clock Enable Register 1
  pub pmc_pcer1: WO<u32>,
  /// [260]: Peripheral Clock Disable Register 1
  pub pmc_pcdr1: WO<u32>,
  /// [264]: Peripheral Clock Status Register 1
  pub pmc_pcsr1: RO<u32>,
  /// [268]: Peripheral Control Register
  pub pmc_pcr: RW<u32>,
}

pub fn pmc() -> *mut PMC {
  1074660864 as *mut PMC
}

#[repr(C)]
/// Universal Asynchronous Receiver Transmitter
pub struct UART {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [12]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [16]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [20]: Status Register
  pub sr: RO<u32>,
  /// [24]: Receive Holding Register
  pub rhr: RO<u32>,
  /// [28]: Transmit Holding Register
  pub thr: WO<u32>,
  /// [32]: Baud Rate Generator Register
  pub brgr: RW<u32>,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  reserved0xe4: u32,
  reserved0xe8: u32,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Receive Pointer Register
  pub rpr: RW<u32>,
  /// [260]: Receive Counter Register
  pub rcr: RW<u32>,
  /// [264]: Transmit Pointer Register
  pub tpr: RW<u32>,
  /// [268]: Transmit Counter Register
  pub tcr: RW<u32>,
  /// [272]: Receive Next Pointer Register
  pub rnpr: RW<u32>,
  /// [276]: Receive Next Counter Register
  pub rncr: RW<u32>,
  /// [280]: Transmit Next Pointer Register
  pub tnpr: RW<u32>,
  /// [284]: Transmit Next Counter Register
  pub tncr: RW<u32>,
  /// [288]: Transfer Control Register
  pub ptcr: WO<u32>,
  /// [292]: Transfer Status Register
  pub ptsr: RO<u32>,
}

pub fn uart() -> *mut UART {
  1074661376 as *mut UART
}

#[repr(C)]
/// Chip Identifier
pub struct CHIPID {
  /// [0]: Chip ID Register
  pub cidr: RO<u32>,
  /// [4]: Chip ID Extension Register
  pub exid: RO<u32>,
}

pub fn chipid() -> *mut CHIPID {
  1074661696 as *mut CHIPID
}

#[repr(C)]
/// Embedded Flash Controller 0
pub struct EFC0 {
  /// [0]: EEFC Flash Mode Register
  pub fmr: RW<u32>,
  /// [4]: EEFC Flash Command Register
  pub fcr: WO<u32>,
  /// [8]: EEFC Flash Status Register
  pub fsr: RO<u32>,
  /// [12]: EEFC Flash Result Register
  pub frr: RO<u32>,
}

pub fn efc0() -> *mut EFC0 {
  1074661888 as *mut EFC0
}

#[repr(C)]
/// Embedded Flash Controller 1
pub struct EFC1 {
  /// [0]: EEFC Flash Mode Register
  pub fmr: RW<u32>,
  /// [4]: EEFC Flash Command Register
  pub fcr: WO<u32>,
  /// [8]: EEFC Flash Status Register
  pub fsr: RO<u32>,
  /// [12]: EEFC Flash Result Register
  pub frr: RO<u32>,
}

pub fn efc1() -> *mut EFC1 {
  1074662400 as *mut EFC1
}

#[repr(C)]
/// Parallel Input/Output Controller A
pub struct PIOA {
  /// [0]: PIO Enable Register
  pub per: WO<u32>,
  /// [4]: PIO Disable Register
  pub pdr: WO<u32>,
  /// [8]: PIO Status Register
  pub psr: RO<u32>,
  reserved0xc: u32,
  /// [16]: Output Enable Register
  pub oer: WO<u32>,
  /// [20]: Output Disable Register
  pub odr: WO<u32>,
  /// [24]: Output Status Register
  pub osr: RO<u32>,
  reserved0x1c: u32,
  /// [32]: Glitch Input Filter Enable Register
  pub ifer: WO<u32>,
  /// [36]: Glitch Input Filter Disable Register
  pub ifdr: WO<u32>,
  /// [40]: Glitch Input Filter Status Register
  pub ifsr: RO<u32>,
  reserved0x2c: u32,
  /// [48]: Set Output Data Register
  pub sodr: WO<u32>,
  /// [52]: Clear Output Data Register
  pub codr: WO<u32>,
  /// [56]: Output Data Status Register
  pub odsr: RW<u32>,
  /// [60]: Pin Data Status Register
  pub pdsr: RO<u32>,
  /// [64]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [68]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [72]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [76]: Interrupt Status Register
  pub isr: RO<u32>,
  /// [80]: Multi-driver Enable Register
  pub mder: WO<u32>,
  /// [84]: Multi-driver Disable Register
  pub mddr: WO<u32>,
  /// [88]: Multi-driver Status Register
  pub mdsr: RO<u32>,
  reserved0x5c: u32,
  /// [96]: Pull-up Disable Register
  pub pudr: WO<u32>,
  /// [100]: Pull-up Enable Register
  pub puer: WO<u32>,
  /// [104]: Pad Pull-up Status Register
  pub pusr: RO<u32>,
  reserved0x6c: u32,
  /// [112]: Peripheral AB Select Register
  pub absr: RW<u32>,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: System Clock Glitch Input Filter Select Register
  pub scifsr: WO<u32>,
  /// [132]: Debouncing Input Filter Select Register
  pub difsr: WO<u32>,
  /// [136]: Glitch or Debouncing Input Filter Clock Selection Status Register
  pub ifdgsr: RO<u32>,
  /// [140]: Slow Clock Divider Debouncing Register
  pub scdr: RW<u32>,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  /// [160]: Output Write Enable
  pub ower: WO<u32>,
  /// [164]: Output Write Disable
  pub owdr: WO<u32>,
  /// [168]: Output Write Status Register
  pub owsr: RO<u32>,
  reserved0xac: u32,
  /// [176]: Additional Interrupt Modes Enable Register
  pub aimer: WO<u32>,
  /// [180]: Additional Interrupt Modes Disables Register
  pub aimdr: WO<u32>,
  /// [184]: Additional Interrupt Modes Mask Register
  pub aimmr: RO<u32>,
  reserved0xbc: u32,
  /// [192]: Edge Select Register
  pub esr: WO<u32>,
  /// [196]: Level Select Register
  pub lsr: WO<u32>,
  /// [200]: Edge/Level Status Register
  pub elsr: RO<u32>,
  reserved0xcc: u32,
  /// [208]: Falling Edge/Low Level Select Register
  pub fellsr: WO<u32>,
  /// [212]: Rising Edge/ High Level Select Register
  pub rehlsr: WO<u32>,
  /// [216]: Fall/Rise - Low/High Status Register
  pub frlhsr: RO<u32>,
  reserved0xdc: u32,
  /// [224]: Lock Status
  pub locksr: RO<u32>,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
}

pub fn pioa() -> *mut PIOA {
  1074662912 as *mut PIOA
}

#[repr(C)]
/// Parallel Input/Output Controller B
pub struct PIOB {
  /// [0]: PIO Enable Register
  pub per: WO<u32>,
  /// [4]: PIO Disable Register
  pub pdr: WO<u32>,
  /// [8]: PIO Status Register
  pub psr: RO<u32>,
  reserved0xc: u32,
  /// [16]: Output Enable Register
  pub oer: WO<u32>,
  /// [20]: Output Disable Register
  pub odr: WO<u32>,
  /// [24]: Output Status Register
  pub osr: RO<u32>,
  reserved0x1c: u32,
  /// [32]: Glitch Input Filter Enable Register
  pub ifer: WO<u32>,
  /// [36]: Glitch Input Filter Disable Register
  pub ifdr: WO<u32>,
  /// [40]: Glitch Input Filter Status Register
  pub ifsr: RO<u32>,
  reserved0x2c: u32,
  /// [48]: Set Output Data Register
  pub sodr: WO<u32>,
  /// [52]: Clear Output Data Register
  pub codr: WO<u32>,
  /// [56]: Output Data Status Register
  pub odsr: RW<u32>,
  /// [60]: Pin Data Status Register
  pub pdsr: RO<u32>,
  /// [64]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [68]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [72]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [76]: Interrupt Status Register
  pub isr: RO<u32>,
  /// [80]: Multi-driver Enable Register
  pub mder: WO<u32>,
  /// [84]: Multi-driver Disable Register
  pub mddr: WO<u32>,
  /// [88]: Multi-driver Status Register
  pub mdsr: RO<u32>,
  reserved0x5c: u32,
  /// [96]: Pull-up Disable Register
  pub pudr: WO<u32>,
  /// [100]: Pull-up Enable Register
  pub puer: WO<u32>,
  /// [104]: Pad Pull-up Status Register
  pub pusr: RO<u32>,
  reserved0x6c: u32,
  /// [112]: Peripheral AB Select Register
  pub absr: RW<u32>,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: System Clock Glitch Input Filter Select Register
  pub scifsr: WO<u32>,
  /// [132]: Debouncing Input Filter Select Register
  pub difsr: WO<u32>,
  /// [136]: Glitch or Debouncing Input Filter Clock Selection Status Register
  pub ifdgsr: RO<u32>,
  /// [140]: Slow Clock Divider Debouncing Register
  pub scdr: RW<u32>,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  /// [160]: Output Write Enable
  pub ower: WO<u32>,
  /// [164]: Output Write Disable
  pub owdr: WO<u32>,
  /// [168]: Output Write Status Register
  pub owsr: RO<u32>,
  reserved0xac: u32,
  /// [176]: Additional Interrupt Modes Enable Register
  pub aimer: WO<u32>,
  /// [180]: Additional Interrupt Modes Disables Register
  pub aimdr: WO<u32>,
  /// [184]: Additional Interrupt Modes Mask Register
  pub aimmr: RO<u32>,
  reserved0xbc: u32,
  /// [192]: Edge Select Register
  pub esr: WO<u32>,
  /// [196]: Level Select Register
  pub lsr: WO<u32>,
  /// [200]: Edge/Level Status Register
  pub elsr: RO<u32>,
  reserved0xcc: u32,
  /// [208]: Falling Edge/Low Level Select Register
  pub fellsr: WO<u32>,
  /// [212]: Rising Edge/ High Level Select Register
  pub rehlsr: WO<u32>,
  /// [216]: Fall/Rise - Low/High Status Register
  pub frlhsr: RO<u32>,
  reserved0xdc: u32,
  /// [224]: Lock Status
  pub locksr: RO<u32>,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
}

pub fn piob() -> *mut PIOB {
  1074663424 as *mut PIOB
}

#[repr(C)]
/// Parallel Input/Output Controller C
pub struct PIOC {
  /// [0]: PIO Enable Register
  pub per: WO<u32>,
  /// [4]: PIO Disable Register
  pub pdr: WO<u32>,
  /// [8]: PIO Status Register
  pub psr: RO<u32>,
  reserved0xc: u32,
  /// [16]: Output Enable Register
  pub oer: WO<u32>,
  /// [20]: Output Disable Register
  pub odr: WO<u32>,
  /// [24]: Output Status Register
  pub osr: RO<u32>,
  reserved0x1c: u32,
  /// [32]: Glitch Input Filter Enable Register
  pub ifer: WO<u32>,
  /// [36]: Glitch Input Filter Disable Register
  pub ifdr: WO<u32>,
  /// [40]: Glitch Input Filter Status Register
  pub ifsr: RO<u32>,
  reserved0x2c: u32,
  /// [48]: Set Output Data Register
  pub sodr: WO<u32>,
  /// [52]: Clear Output Data Register
  pub codr: WO<u32>,
  /// [56]: Output Data Status Register
  pub odsr: RW<u32>,
  /// [60]: Pin Data Status Register
  pub pdsr: RO<u32>,
  /// [64]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [68]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [72]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [76]: Interrupt Status Register
  pub isr: RO<u32>,
  /// [80]: Multi-driver Enable Register
  pub mder: WO<u32>,
  /// [84]: Multi-driver Disable Register
  pub mddr: WO<u32>,
  /// [88]: Multi-driver Status Register
  pub mdsr: RO<u32>,
  reserved0x5c: u32,
  /// [96]: Pull-up Disable Register
  pub pudr: WO<u32>,
  /// [100]: Pull-up Enable Register
  pub puer: WO<u32>,
  /// [104]: Pad Pull-up Status Register
  pub pusr: RO<u32>,
  reserved0x6c: u32,
  /// [112]: Peripheral AB Select Register
  pub absr: RW<u32>,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: System Clock Glitch Input Filter Select Register
  pub scifsr: WO<u32>,
  /// [132]: Debouncing Input Filter Select Register
  pub difsr: WO<u32>,
  /// [136]: Glitch or Debouncing Input Filter Clock Selection Status Register
  pub ifdgsr: RO<u32>,
  /// [140]: Slow Clock Divider Debouncing Register
  pub scdr: RW<u32>,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  /// [160]: Output Write Enable
  pub ower: WO<u32>,
  /// [164]: Output Write Disable
  pub owdr: WO<u32>,
  /// [168]: Output Write Status Register
  pub owsr: RO<u32>,
  reserved0xac: u32,
  /// [176]: Additional Interrupt Modes Enable Register
  pub aimer: WO<u32>,
  /// [180]: Additional Interrupt Modes Disables Register
  pub aimdr: WO<u32>,
  /// [184]: Additional Interrupt Modes Mask Register
  pub aimmr: RO<u32>,
  reserved0xbc: u32,
  /// [192]: Edge Select Register
  pub esr: WO<u32>,
  /// [196]: Level Select Register
  pub lsr: WO<u32>,
  /// [200]: Edge/Level Status Register
  pub elsr: RO<u32>,
  reserved0xcc: u32,
  /// [208]: Falling Edge/Low Level Select Register
  pub fellsr: WO<u32>,
  /// [212]: Rising Edge/ High Level Select Register
  pub rehlsr: WO<u32>,
  /// [216]: Fall/Rise - Low/High Status Register
  pub frlhsr: RO<u32>,
  reserved0xdc: u32,
  /// [224]: Lock Status
  pub locksr: RO<u32>,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
}

pub fn pioc() -> *mut PIOC {
  1074663936 as *mut PIOC
}

#[repr(C)]
/// Parallel Input/Output Controller D
pub struct PIOD {
  /// [0]: PIO Enable Register
  pub per: WO<u32>,
  /// [4]: PIO Disable Register
  pub pdr: WO<u32>,
  /// [8]: PIO Status Register
  pub psr: RO<u32>,
  reserved0xc: u32,
  /// [16]: Output Enable Register
  pub oer: WO<u32>,
  /// [20]: Output Disable Register
  pub odr: WO<u32>,
  /// [24]: Output Status Register
  pub osr: RO<u32>,
  reserved0x1c: u32,
  /// [32]: Glitch Input Filter Enable Register
  pub ifer: WO<u32>,
  /// [36]: Glitch Input Filter Disable Register
  pub ifdr: WO<u32>,
  /// [40]: Glitch Input Filter Status Register
  pub ifsr: RO<u32>,
  reserved0x2c: u32,
  /// [48]: Set Output Data Register
  pub sodr: WO<u32>,
  /// [52]: Clear Output Data Register
  pub codr: WO<u32>,
  /// [56]: Output Data Status Register
  pub odsr: RW<u32>,
  /// [60]: Pin Data Status Register
  pub pdsr: RO<u32>,
  /// [64]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [68]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [72]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [76]: Interrupt Status Register
  pub isr: RO<u32>,
  /// [80]: Multi-driver Enable Register
  pub mder: WO<u32>,
  /// [84]: Multi-driver Disable Register
  pub mddr: WO<u32>,
  /// [88]: Multi-driver Status Register
  pub mdsr: RO<u32>,
  reserved0x5c: u32,
  /// [96]: Pull-up Disable Register
  pub pudr: WO<u32>,
  /// [100]: Pull-up Enable Register
  pub puer: WO<u32>,
  /// [104]: Pad Pull-up Status Register
  pub pusr: RO<u32>,
  reserved0x6c: u32,
  /// [112]: Peripheral AB Select Register
  pub absr: RW<u32>,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: System Clock Glitch Input Filter Select Register
  pub scifsr: WO<u32>,
  /// [132]: Debouncing Input Filter Select Register
  pub difsr: WO<u32>,
  /// [136]: Glitch or Debouncing Input Filter Clock Selection Status Register
  pub ifdgsr: RO<u32>,
  /// [140]: Slow Clock Divider Debouncing Register
  pub scdr: RW<u32>,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  /// [160]: Output Write Enable
  pub ower: WO<u32>,
  /// [164]: Output Write Disable
  pub owdr: WO<u32>,
  /// [168]: Output Write Status Register
  pub owsr: RO<u32>,
  reserved0xac: u32,
  /// [176]: Additional Interrupt Modes Enable Register
  pub aimer: WO<u32>,
  /// [180]: Additional Interrupt Modes Disables Register
  pub aimdr: WO<u32>,
  /// [184]: Additional Interrupt Modes Mask Register
  pub aimmr: RO<u32>,
  reserved0xbc: u32,
  /// [192]: Edge Select Register
  pub esr: WO<u32>,
  /// [196]: Level Select Register
  pub lsr: WO<u32>,
  /// [200]: Edge/Level Status Register
  pub elsr: RO<u32>,
  reserved0xcc: u32,
  /// [208]: Falling Edge/Low Level Select Register
  pub fellsr: WO<u32>,
  /// [212]: Rising Edge/ High Level Select Register
  pub rehlsr: WO<u32>,
  /// [216]: Fall/Rise - Low/High Status Register
  pub frlhsr: RO<u32>,
  reserved0xdc: u32,
  /// [224]: Lock Status
  pub locksr: RO<u32>,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
  /// [232]: Write Protect Status Register
  pub wpsr: RO<u32>,
}

pub fn piod() -> *mut PIOD {
  1074664448 as *mut PIOD
}

#[repr(C)]
/// Reset Controller
pub struct RSTC {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Status Register
  pub sr: RO<u32>,
  /// [8]: Mode Register
  pub mr: RW<u32>,
}

pub fn rstc() -> *mut RSTC {
  1074665984 as *mut RSTC
}

#[repr(C)]
/// Supply Controller
pub struct SUPC {
  /// [0]: Supply Controller Control Register
  pub cr: WO<u32>,
  /// [4]: Supply Controller Supply Monitor Mode Register
  pub smmr: RW<u32>,
  /// [8]: Supply Controller Mode Register
  pub mr: RW<u32>,
  /// [12]: Supply Controller Wake Up Mode Register
  pub wumr: RW<u32>,
  /// [16]: Supply Controller Wake Up Inputs Register
  pub wuir: RW<u32>,
  /// [20]: Supply Controller Status Register
  pub sr: RO<u32>,
}

pub fn supc() -> *mut SUPC {
  1074666000 as *mut SUPC
}

#[repr(C)]
/// Real-time Timer
pub struct RTT {
  /// [0]: Mode Register
  pub mr: RW<u32>,
  /// [4]: Alarm Register
  pub ar: RW<u32>,
  /// [8]: Value Register
  pub vr: RO<u32>,
  /// [12]: Status Register
  pub sr: RO<u32>,
}

pub fn rtt() -> *mut RTT {
  1074666032 as *mut RTT
}

#[repr(C)]
/// Watchdog Timer
pub struct WDT {
  /// [0]: Control Register
  pub cr: WO<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Status Register
  pub sr: RO<u32>,
}

pub fn wdt() -> *mut WDT {
  1074666064 as *mut WDT
}

#[repr(C)]
/// Real-time Clock
pub struct RTC {
  /// [0]: Control Register
  pub cr: RW<u32>,
  /// [4]: Mode Register
  pub mr: RW<u32>,
  /// [8]: Time Register
  pub timr: RW<u32>,
  /// [12]: Calendar Register
  pub calr: RW<u32>,
  /// [16]: Time Alarm Register
  pub timalr: RW<u32>,
  /// [20]: Calendar Alarm Register
  pub calalr: RW<u32>,
  /// [24]: Status Register
  pub sr: RO<u32>,
  /// [28]: Status Clear Command Register
  pub sccr: WO<u32>,
  /// [32]: Interrupt Enable Register
  pub ier: WO<u32>,
  /// [36]: Interrupt Disable Register
  pub idr: WO<u32>,
  /// [40]: Interrupt Mask Register
  pub imr: RO<u32>,
  /// [44]: Valid Entry Register
  pub ver: RO<u32>,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  /// [228]: Write Protect Mode Register
  pub wpmr: RW<u32>,
}

pub fn rtc() -> *mut RTC {
  1074666080 as *mut RTC
}

#[repr(C)]
/// General Purpose Backup Register
pub struct GPBR {
  /// [0]: General Purpose Backup Register
  pub gpbrs: RW<u32>,
}

pub fn gpbr() -> *mut GPBR {
  1074666128 as *mut GPBR
}
