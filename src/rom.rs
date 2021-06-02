use eyre::ensure;

#[derive(Debug)]
pub struct Rom {
    pub prg: [u8; 0x8000],
}

impl Rom {
    pub fn from_ines(ines: impl AsRef<[u8]>) -> eyre::Result<Self> {
        use std::convert::TryInto;

        const INES_MAGIC: &[u8] = b"NES\x1A";

        let ines = ines.as_ref();
        ensure!(ines.len() == 16 + 0x8000 + 0x8000, "iNES ROM size mismatch");
        ensure!(ines.starts_with(INES_MAGIC), "iNES magic not found");

        let prg = ines[16..][..0x8000].try_into().unwrap();

        Ok(Self { prg })
    }
}
