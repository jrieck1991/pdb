use rand::random;
use sgx_isa::{Attributes, ErrorCode, Miscselect};

// Mock enclave functions to facilitate local development

// returned key for seal_key and unseal_key functions
// use the same key so that the "encryption" is consistent
const KEY: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

#[derive(Debug)]
pub struct SealData {
    rand: [u8; 16],
    isvsvn: u16,
    cpusvn: [u8; 16],
    attributes: Attributes,
    miscselect: Miscselect,
}

pub fn seal_key(_label: [u8; 16]) -> ([u8; 16], SealData) {
    let seal_data = SealData {
        rand: random(),

        isvsvn: 5,
        cpusvn: random(),
        attributes: Attributes::default(),
        miscselect: Miscselect::default(),
    };

    (KEY, seal_data)
}

pub fn unseal_key(_label: [u8; 16], _seal_data: SealData) -> Result<[u8; 16], ErrorCode> {
    Ok(KEY)
}
