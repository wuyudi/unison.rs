pub mod base32hex;
pub mod frame;
pub mod ir_exec;
pub mod ir_runtime;
pub mod pattern;
pub mod stack;
pub mod trace;
pub mod types;

pub fn pack(env: &types::RuntimeEnv, hash: &str) -> String {
    let encoded: Vec<u8> = bincode::serialize(&(env, hash)).unwrap();
    base64::encode(&encoded)
}

pub fn unpack(data: &str) -> (types::RuntimeEnv, String) {
    let raw = base64::decode(data).expect("Cannot base64 unpack");
    bincode::deserialize(&raw).expect("Unable to deserialize")
}
