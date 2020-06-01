use rand::random;
use sgx_isa::{Attributes, ErrorCode, Miscselect};

// info about how sealing key was derived.
#[derive(Debug)]
pub struct SealData {
    rand: [u8; 16],
    isvsvn: u16,
    cpusvn: [u8; 16],
    attributes: Attributes,
    miscselect: Miscselect,
}

// get a key for sealing data
// returned key may be used for AE
// use different labels for different types of data
// SealData should be stored alongsize the ciphertext
pub fn seal_key(label: [u8; 16]) -> ([u8; 16], SealData) {
    
    let seal_data = SealData {
        rand: random(),

        isvsvn: 5,
        cpusvn: random(),
        attributes: Attributes::default(),
        miscselect: Miscselect::default(),
    };

    (random(), seal_data)
}

// get a key for unsealing data
// returned key can be used for AD
// pass in same label used to get the sealing key
// pass in the seal_data returned with the label
pub fn unseal_key(label: [u8; 16], seal_data: SealData) -> Result<[u8; 16], ErrorCode> {
    Ok(random())
}
