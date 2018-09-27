pub const HEADER_SIZE: u16 = 0x10;
pub const PRG_SIZE: u16    = 0x4000;
pub const CHR_SIZE: u16    = 0x2000;

#[derive(Debug)]
pub enum Mirroring {
    Horizontal,
    Vertical
}

#[derive(Debug)]
pub struct Flag6 {
    pub mirroring: Mirroring,
    pub is_persistent: bool,
    pub is_trainer: bool,
    pub is_above_mirroring: bool,
}

impl Flag6 {
    fn parse_from(bytes: &u8) -> Flag6 {
        Flag6 {
            mirroring: if bytes & 0x1 == 0x01 { Mirroring::Vertical } else { Mirroring::Horizontal },
            is_persistent: bytes & 0x2 == 0x01,
            is_trainer: bytes & 0x4 == 0x1,
            is_above_mirroring: bytes & 0x8 == 0x01,
        }
    }
}

#[derive(Debug)]
pub struct NesHeader {
    pub size_of_prg_rom_in_16kb: u8,
    pub size_of_chr_rom_in_16kb: u8,

    // Flag6
    pub flag6: Flag6,

    pub mapper: u8,

    pub flag7: u8,

    pub size_of_prg_rom_in_8kb: u8,

    pub flag9: u8,
    pub flag10: u8,
}

impl NesHeader {
    fn parse_from(buf: &[u8]) -> NesHeader {
        NesHeader {
            size_of_prg_rom_in_16kb: buf[4],
            size_of_chr_rom_in_16kb: buf[5],

            // Flag6
            flag6: Flag6::parse_from(&buf[6]),
            mapper: buf[6] >> 4,

            flag7: buf[7],
            
            size_of_prg_rom_in_8kb: buf[8],
            flag9: buf[9],
            flag10: buf[10],
        }
    }
}

pub struct INes {
    pub header: NesHeader,
    pub program: Vec<u8>,
    pub character: Vec<u8>
}

impl INes {
    pub fn parse_from(buffer: &[u8]) -> INes {
        // Header
        let mut start: usize = 0;
        let mut end: usize   = HEADER_SIZE as usize;

        let header = &buffer[start..end];
        println!("{:?}", header.len());

        let nes: String = String::from_utf8((&header[0..4]).to_vec()).unwrap();
        println!("{}", nes);

        let nes_header = NesHeader::parse_from(&header);
        println!("{:?}", nes_header);
    
        // Program
        start = end;
        end   = start + (PRG_SIZE * nes_header.size_of_prg_rom_in_16kb  as u16) as usize;
        let program = &buffer[start..end];
        println!("{:?}", program.len());

        // Character
        start = end;
        end   = start + (CHR_SIZE * nes_header.size_of_chr_rom_in_16kb  as u16) as usize;
        let character = &buffer[start..end];

        INes {
            header: nes_header,
            program: program.to_vec(),
            character: character.to_vec()
        }
    }
}
