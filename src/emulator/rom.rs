#[derive(Debug, Clone, Copy)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

pub struct Rom {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: u8,
    screen_mirroring: Mirroring,
}

const NES_TAG_START_INDX: usize = 0;
const NES_TAG_SIZE: usize = 4;
const NES_TAG: [u8; NES_TAG_SIZE] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A];
const PRG_ROM_SIZE_INDX: usize = 4;
const CHR_ROM_SIZE_INDX: usize = 5;
const FLAGS_6_INDX: usize = 6;
const FLAGS_7_INDX: usize = 7;
const FLAGS_8_INDX: usize = 8;
const FLAGS_9_INDX: usize = 9;
const FLAGS_10_INDX: usize = 10;

const UPPER_NYBLE: u8 = 0xF0;
const INES_VER_MASK: u8 = 0b1100;
const FOUR_SCREEN_MASK: u8 = 0b1000;
const VER_MIRROR_MASK: u8 = 0b1;
const TRAINER_MASK: u8 = 0b100;

const KB: usize = 1024;
const PRG_ROM_PAGE_SIZE: usize = 16 * KB;
const CHR_ROM_PAGE_SIZE: usize = 8 * KB;
const HEADER_SIZE: usize = 16;
const TRAINER_SIZE: usize = 512;

impl Rom {
    pub fn new(raw: &[u8]) -> Result<Rom, String> {
        if Self::get_header_tag(raw) != NES_TAG {
            return Err("File is not in iNES file format".to_string());
        }

        if Self::get_nes_ver(raw) != 0 {
            return Err("NES2.0 format is not supported".to_string());
        }

        let prg_rom_size = Self::prg_rom_size(raw);
        let chr_rom_size = Self::chr_rom_szie(raw);

        let prg_rom_start = Self::prg_start_offset(raw);
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let prg_rom_end = prg_rom_start + prg_rom_size;
        let chr_rom_end = chr_rom_start + chr_rom_size;

        Ok(Rom {
            prg_rom: raw[prg_rom_start..prg_rom_end].to_vec(),
            chr_rom: raw[chr_rom_start..chr_rom_end].to_vec(),
            mapper: Self::get_mapper(raw),
            screen_mirroring: Self::get_screen_mirroring(raw),
        })
    }

    pub fn read_prg(&self, addr: u16) -> u8 {
        self.prg_rom[addr as usize]
    }

    pub fn reag_chr(&self, addr: u16) -> u8 {
        self.chr_rom[addr as usize]
    }

    pub fn prg_size(&self) -> usize {
        self.prg_rom.len()
    }

    pub fn chr_size(&self) -> usize {
        self.chr_rom.len()
    }

    pub fn clone_prg_rom(&self) -> Vec<u8> {
        self.prg_rom.clone()
    }

    pub fn clone_chr_rom(&self) -> Vec<u8> {
        self.chr_rom.clone()
    }

    pub fn get_mirroring(&self) -> Mirroring {
        self.screen_mirroring
    }

    fn get_header_tag(header: &[u8]) -> &[u8] {
        &header[NES_TAG_START_INDX..NES_TAG_SIZE]
    }

    fn get_mapper(header: &[u8]) -> u8 {
        let mapper_lower_nybble = (header[FLAGS_6_INDX] & UPPER_NYBLE) >> 4;
        let mapper_upper_nybble = header[FLAGS_7_INDX] & UPPER_NYBLE;
        mapper_upper_nybble | mapper_lower_nybble
    }

    fn get_nes_ver(header: &[u8]) -> u8 {
        header[FLAGS_7_INDX] & INES_VER_MASK
    }

    fn get_screen_mirroring(header: &[u8]) -> Mirroring {
        let four_screen = (header[FLAGS_6_INDX] & FOUR_SCREEN_MASK) != 0;
        let vertical_mirroring = (header[FLAGS_6_INDX] & VER_MIRROR_MASK) != 0;
        match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        }
    }

    fn prg_rom_size(header: &[u8]) -> usize {
        header[PRG_ROM_SIZE_INDX] as usize * PRG_ROM_PAGE_SIZE
    }

    fn chr_rom_szie(header: &[u8]) -> usize {
        header[CHR_ROM_SIZE_INDX] as usize * CHR_ROM_PAGE_SIZE
    }

    fn prg_start_offset(header: &[u8]) -> usize {
        let skip_trainer = (header[FLAGS_6_INDX] & TRAINER_MASK) != 0;
        HEADER_SIZE + if skip_trainer { TRAINER_SIZE } else { 0 }
    }
}
