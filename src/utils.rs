use web3::types::{Bytes, H160, H256, H64, U256};

pub fn format_nonce(h: H64) -> String {
    return format!("'{:?}'", h);
}

pub fn format_hash(h: H256) -> String {
    return format!("'{:?}'", h);
}

pub fn format_address(h: H160) -> String {
    return format!("'{:?}'", h);
}

pub fn format_bytes(b: &Bytes) -> String {
    return format!("'{}'", serde_json::to_string(b).unwrap().replace("\"", ""));
}

pub fn format_number(n: U256) -> String {
    return format!("'{}'", n.to_string());
}
