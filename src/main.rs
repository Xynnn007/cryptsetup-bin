use std::path::Path;

use libcryptsetup_rs::{
    CryptInit, CryptParamsLuks2, CryptParamsLuks2Ref,
    consts::{flags::CryptVolumeKey, vals::EncryptionFormat},
};

const SECTOR_SIZE: u32 = 4096;

fn main() {
    let path = Path::new("/tmp/1");
    let mut device = CryptInit::init(path).unwrap();

    let params = CryptParamsLuks2 {
        pbkdf: None,
        integrity: None,
        integrity_params: None,
        data_alignment: 0,
        data_device: None,
        sector_size: SECTOR_SIZE,
        label: None,
        subsystem: None,
    };

    device
        .context_handle()
        .format(
            EncryptionFormat::Luks2,
            ("aes", "xts-plain"),
            None,
            libcryptsetup_rs::Either::Right(256 / 8),
            Some(&mut TryInto::<CryptParamsLuks2Ref>::try_into(&params).unwrap()),
        )
        .unwrap();
    device
        .keyslot_handle()
        .add_by_key(None, None, b"test-passphrase", CryptVolumeKey::empty())
        .unwrap();
}
