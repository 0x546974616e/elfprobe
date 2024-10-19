use std::fmt;

use elfprobe_macro::Pod;

use crate::utils::{define_constants, display_table};

use super::identification::ElfIdentification;
use super::magic::Magic;
use super::types::ElfType;

define_constants! {
  e_version(usize) "Object file version.",
  EV_NONE = 0 "Invalid version",
  EV_CURRENT = 1 "Current version",
}

define_constants! {
  e_type(u16) "Object file types",
  ET_NONE = 0 "No file type",
  ET_REL = 1 "Relocatable object file",
  ET_EXEC = 2 "Executable file",
  ET_DYN = 3 "Shared object file",
  ET_CORE = 4 "Core file",
  [ ET_LOOS, ET_HIOS ] = [ 0xFE00, 0xFEFF ] "Environment-specific use",
  [ ET_LOPROC, ET_HIPROC ] = [ 0xFF00, 0xFFFF ] "Processor-specific use",
}

///
/// Constants retrieved from:
/// - `/usr/include{/linux,}/elf.h`
/// - <https://github.com/torvalds/linux/blob/master/include/uapi/linux/elf-em.h\n">
/// - <https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html\n">
/// - <https://en.wikipedia.org/wiki/Executable_and_Linkable_Format\n">
///
define_constants! {
  e_machine(u16) "ELF target machines",
  EM_NONE = 0x0 "No machine",
  EM_M32 = 0x1 "AT&T WE 32100",
  EM_SPARC = 0x2 "SUN SPARC",
  EM_386 = 0x3 "Intel 80386",
  EM_68K = 0x4 "Motorola m68k family",
  EM_88K = 0x5 "Motorola m88k family",
  EM_IAMCU = 0x6 "Intel MCU",
  EM_860 = 0x7 "Intel 80860",
  EM_MIPS = 0x8 "MIPS R3000 big-endian",
  EM_S370 = 0x9 "IBM System/370",
  EM_MIPS_RS3_LE = 0xA "MIPS R3000 little-endian",
  EM_PARISC = 0xF "HPPA",
  EM_VPP500 = 0x11 "Fujitsu VPP500",
  EM_SPARC32PLUS = 0x12 "Sun's \"v8plus\"",
  EM_960 = 0x13 "Intel 80960",
  EM_PPC = 0x14 "PowerPC",
  EM_PPC64 = 0x15 "PowerPC 64-bit",
  EM_S390 = 0x16 "IBM S390",
  EM_SPU = 0x17 "IBM SPU/SPC",
  EM_V800 = 0x24 "NEC V800 series",
  EM_FR20 = 0x25 "Fujitsu FR20",
  EM_RH32 = 0x26 "TRW RH-32",
  EM_RCE = 0x27 "Motorola RCE",
  EM_ARM = 0x28 "ARM",
  EM_FAKE_ALPHA = 0x29 "Digital Alpha",
  EM_SH = 0x2A "Hitachi SH",
  EM_SPARCV9 = 0x2B "SPARC v9 64-bit",
  EM_TRICORE = 0x2C "Siemens Tricore",
  EM_ARC = 0x2D "Argonaut RISC Core",
  EM_H8_300 = 0x2E "Hitachi H8/300",
  EM_H8_300H = 0x2F "Hitachi H8/300H",
  EM_H8S = 0x30 "Hitachi H8S",
  EM_H8_500 = 0x31 "Hitachi H8/500",
  EM_IA_64 = 0x32 "Intel Merced",
  EM_MIPS_X = 0x33 "Stanford MIPS-X",
  EM_COLDFIRE = 0x34 "Motorola Coldfire",
  EM_68HC12 = 0x35 "Motorola M68HC12",
  EM_MMA = 0x36 "Fujitsu MMA Multimedia Accelerator",
  EM_PCP = 0x37 "Siemens PCP",
  EM_NCPU = 0x38 "Sony nCPU embeeded RISC",
  EM_NDR1 = 0x39 "Denso NDR1 microprocessor",
  EM_STARCORE = 0x3A "Motorola Start*Core processor",
  EM_ME16 = 0x3B "Toyota ME16 processor",
  EM_ST100 = 0x3C "STMicroelectronic ST100 processor",
  EM_TINYJ = 0x3D "Advanced Logic Corp. Tinyj emb.fam",
  EM_X86_64 = 0x3E "AMD x86-64 architecture",
  EM_PDSP = 0x3F "Sony DSP Processor",
  EM_PDP10 = 0x40 "Digital PDP-10",
  EM_PDP11 = 0x41 "Digital PDP-11",
  EM_FX66 = 0x42 "Siemens FX66 microcontroller",
  EM_ST9PLUS = 0x43 "STMicroelectronics ST9+ 8/16 mc",
  EM_ST7 = 0x44 "STmicroelectronics ST7 8 bit mc",
  EM_68HC16 = 0x45 "Motorola MC68HC16 microcontroller",
  EM_68HC11 = 0x46 "Motorola MC68HC11 microcontroller",
  EM_68HC08 = 0x47 "Motorola MC68HC08 microcontroller",
  EM_68HC05 = 0x48 "Motorola MC68HC05 microcontroller",
  EM_SVX = 0x49 "Silicon Graphics SVx",
  EM_ST19 = 0x4A "STMicroelectronics ST19 8 bit mc",
  EM_VAX = 0x4B "Digital VAX",
  EM_CRIS = 0x4C "Axis Communications 32-bit emb.proc",
  EM_JAVELIN = 0x4D "Infineon Technologies 32-bit emb.proc",
  EM_FIREPATH = 0x4E "Element 14 64-bit DSP Processor",
  EM_ZSP = 0x4F "LSI Logic 16-bit DSP Processor",
  EM_MMIX = 0x50 "Donald Knuth's educational 64-bit proc",
  EM_HUANY = 0x51 "Harvard University machine-independent object files",
  EM_PRISM = 0x52 "SiTera Prism",
  EM_AVR = 0x53 "Atmel AVR 8-bit microcontroller",
  EM_FR30 = 0x54 "Fujitsu FR30",
  EM_D10V = 0x55 "Mitsubishi D10V",
  EM_D30V = 0x56 "Mitsubishi D30V",
  EM_V850 = 0x57 "NEC v850",
  EM_M32R = 0x58 "Mitsubishi M32R",
  EM_MN10300 = 0x59 "Matsushita MN10300",
  EM_MN10200 = 0x5A "Matsushita MN10200",
  EM_PJ = 0x5B "picoJava",
  EM_OPENRISC = 0x5C "OpenRISC 32-bit embedded processor",
  EM_ARC_COMPACT = 0x5D "ARC International ARCompact",
  EM_XTENSA = 0x5E "Tensilica Xtensa Architecture",
  EM_VIDEOCORE = 0x5F "Alphamosaic VideoCore",
  EM_TMM_GPP = 0x60 "Thompson Multimedia General Purpose Proc",
  EM_NS32K = 0x61 "National Semi. 32000",
  EM_TPC = 0x62 "Tenor Network TPC",
  EM_SNP1K = 0x63 "Trebia SNP 1000",
  EM_ST200 = 0x64 "STMicroelectronics ST200",
  EM_IP2K = 0x65 "Ubicom IP2xxx",
  EM_MAX = 0x66 "MAX processor",
  EM_CR = 0x67 "National Semi. CompactRISC",
  EM_F2MC16 = 0x68 "Fujitsu F2MC16",
  EM_MSP430 = 0x69 "Texas Instruments msp430",
  EM_BLACKFIN = 0x6A "Analog Devices Blackfin DSP",
  EM_SE_C33 = 0x6B "Seiko Epson S1C33 family",
  EM_SEP = 0x6C "Sharp embedded microprocessor",
  EM_ARCA = 0x6D "Arca RISC",
  EM_UNICORE = 0x6E "PKU-Unity & MPRC Peking Uni. mc series",
  EM_EXCESS = 0x6F "eXcess configurable cpu",
  EM_DXP = 0x70 "Icera Semi. Deep Execution Processor",
  EM_ALTERA_NIOS2 = 0x71 "Altera Nios II",
  EM_CRX = 0x72 "National Semi. CompactRISC CRX",
  EM_XGATE = 0x73 "Motorola XGATE",
  EM_C166 = 0x74 "Infineon C16x/XC16x",
  EM_M16C = 0x75 "Renesas M16C",
  EM_DSPIC30F = 0x76 "Microchip Technology dsPIC30F",
  EM_CE = 0x77 "Freescale Communication Engine RISC",
  EM_M32C = 0x78 "Renesas M32C",
  EM_TSK3000 = 0x83 "Altium TSK3000",
  EM_RS08 = 0x84 "Freescale RS08",
  EM_SHARC = 0x85 "Analog Devices SHARC family",
  EM_ECOG2 = 0x86 "Cyan Technology eCOG2",
  EM_SCORE7 = 0x87 "Sunplus S+core7 RISC",
  EM_DSP24 = 0x88 "New Japan Radio (NJR) 24-bit DSP",
  EM_VIDEOCORE3 = 0x89 "Broadcom VideoCore III",
  EM_LATTICEMICO32 = 0x8A "RISC for Lattice FPGA",
  EM_SE_C17 = 0x8B "Seiko Epson C17",
  EM_TI_C6000 = 0x8C "Texas Instruments TMS320C6000 DSP",
  EM_TI_C2000 = 0x8D "Texas Instruments TMS320C2000 DSP",
  EM_TI_C5500 = 0x8E "Texas Instruments TMS320C55x DSP",
  EM_TI_ARP32 = 0x8F "Texas Instruments App. Specific RISC",
  EM_TI_PRU = 0x90 "Texas Instruments Prog. Realtime Unit",
  EM_MMDSP_PLUS = 0xA0 "STMicroelectronics 64bit VLIW DSP",
  EM_CYPRESS_M8C = 0xA1 "Cypress M8C",
  EM_R32C = 0xA2 "Renesas R32C",
  EM_TRIMEDIA = 0xA3 "NXP Semi. TriMedia",
  EM_QDSP6 = 0xA4 "QUALCOMM DSP6",
  EM_8051 = 0xA5 "Intel 8051 and variants",
  EM_STXP7X = 0xA6 "STMicroelectronics STxP7x",
  EM_NDS32 = 0xA7 "Andes Tech. compact code emb. RISC",
  EM_ECOG1X = 0xA8 "Cyan Technology eCOG1X",
  EM_MAXQ30 = 0xA9 "Dallas Semi. MAXQ30 mc",
  EM_XIMO16 = 0xAA "New Japan Radio (NJR) 16-bit DSP",
  EM_MANIK = 0xAB "M2000 Reconfigurable RISC",
  EM_CRAYNV2 = 0xAC "Cray NV2 vector architecture",
  EM_RX = 0xAD "Renesas RX",
  EM_METAG = 0xAE "Imagination Tech. META",
  EM_MCST_ELBRUS = 0xAF "MCST Elbrus",
  EM_ECOG16 = 0xB0 "Cyan Technology eCOG16",
  EM_CR16 = 0xB1 "National Semi. CompactRISC CR16",
  EM_ETPU = 0xB2 "Freescale Extended Time Processing Unit",
  EM_SLE9X = 0xB3 "Infineon Tech. SLE9X",
  EM_L10M = 0xB4 "Intel L10M",
  EM_K10M = 0xB5 "Intel K10M",
  EM_AARCH64 = 0xB7 "ARM AARCH64",
  EM_AVR32 = 0xB9 "Amtel 32-bit microprocessor",
  EM_STM8 = 0xBA "STMicroelectronics STM8",
  EM_TILE64 = 0xBB "Tileta TILE64",
  EM_TILEPRO = 0xBC "Tilera TILEPro",
  EM_MICROBLAZE = 0xBD "Xilinx MicroBlaze",
  EM_CUDA = 0xBE "NVIDIA CUDA",
  EM_TILEGX = 0xBF "Tilera TILE-Gx",
  EM_CLOUDSHIELD = 0xC0 "CloudShield",
  EM_COREA_1ST = 0xC1 "KIPO-KAIST Core-A 1st gen.",
  EM_COREA_2ND = 0xC2 "KIPO-KAIST Core-A 2nd gen.",
  EM_ARC_COMPACT2 = 0xC3 "Synopsys ARCompact V2",
  EM_OPEN8 = 0xC4 "Open8 RISC",
  EM_RL78 = 0xC5 "Renesas RL78",
  EM_VIDEOCORE5 = 0xC6 "Broadcom VideoCore V",
  EM_78KOR = 0xC7 "Renesas 78KOR",
  EM_56800EX = 0xC8 "Freescale 56800EX DSC",
  EM_BA1 = 0xC9 "Beyond BA1",
  EM_BA2 = 0xCA "Beyond BA2",
  EM_XCORE = 0xCB "XMOS xCORE",
  EM_MCHP_PIC = 0xCC "Microchip 8-bit PIC(r)",
  EM_KM32 = 0xD2 "KM211 KM32",
  EM_KMX32 = 0xD3 "KM211 KMX32",
  EM_EMX16 = 0xD4 "KM211 KMX16",
  EM_EMX8 = 0xD5 "KM211 KMX8",
  EM_KVARC = 0xD6 "KM211 KVARC",
  EM_CDP = 0xD7 "Paneve CDP",
  EM_COGE = 0xD8 "Cognitive Smart Memory Processor",
  EM_COOL = 0xD9 "Bluechip CoolEngine",
  EM_NORC = 0xDA "Nanoradio Optimized RISC",
  EM_CSR_KALIMBA = 0xDB "CSR Kalimba",
  EM_Z80 = 0xDC "Zilog Z80",
  EM_VISIUM = 0xDD "Controls and Data Services VISIUMcore",
  EM_FT32 = 0xDE "FTDI Chip FT32",
  EM_MOXIE = 0xDF "Moxie processor",
  EM_AMDGPU = 0xE0 "AMD GPU",
  EM_RISCV = 0xF3 "RISC-V",
  EM_BPF = 0xF7 "Linux BPF -- in-kernel virtual machine",
  EM_CSKY = 0xFC "C-SKY",
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfHeader<ElfType: self::ElfType> {
  /// Identify the file as an ELF object file, and provide information
  /// about the data representation of the object file structures.
  pub e_ident: ElfIdentification<ElfType>,

  /// Identifies the [object file type][e_type].
  pub e_type: ElfType::Half,

  /// Identifies the [target architecture][e_machine].
  pub e_machine: ElfType::Half,

  /// Identifies the [version][e_version] of the object file format
  pub e_version: ElfType::Word,

  pub e_entry: ElfType::Addr,

  pub e_phoff: ElfType::Off,

  pub e_shoff: ElfType::Off,

  pub e_flags: ElfType::Word,

  pub e_ehsize: ElfType::Half,

  pub e_phentsize: ElfType::Half,

  pub e_phnum: ElfType::Half,

  pub e_shentsize: ElfType::Half,

  pub e_shnum: ElfType::Half,

  pub e_shstrndx: ElfType::Half,
}

impl<ElfType: self::ElfType> fmt::Display for ElfHeader<ElfType> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    use super::identification::{ei_class, ei_data, ei_osabi, ei_version};

    display_table!(
      formatter, "ELF Header" =>
      [ "Magic:", Magic::from(&self.e_ident) ],
      [ "Class:", ei_class::into_constant(self.e_ident.ei_class) ],
      [ "Data:", ei_data::into_constant(self.e_ident.ei_data) ],
      [ "Version:", ei_version::into_constant(self.e_ident.ei_version) ],
      [ "OS/ABI:", ei_osabi::into_constant(self.e_ident.ei_osabi) ],
      [ "ABI Version:", self.e_ident.ei_abiversion ],
      [ "Type:", e_type::into_constant(self.e_type) ],
      [ "Machine:", e_machine::into_constant(self.e_machine) ],
      [ "Version:", self.e_version ],
    )
  }
}
