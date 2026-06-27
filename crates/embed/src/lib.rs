use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

const MAGIC: &[u8; 10] = b"IMPBUNDLE\0";
const TRAILER_LEN: usize = 14;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bundle {
    pub entry: String,
    pub modules: HashMap<String, String>,
    #[serde(default)]
    pub original_paths: HashMap<String, String>,
}

pub fn read_embedded() -> Option<Bundle> {
    let exe = std::env::current_exe().ok()?;
    let mut f = File::open(exe).ok()?;
    let len = f.metadata().ok()?.len();
    if len < TRAILER_LEN as u64 {
        return None;
    }

    f.seek(SeekFrom::End(-(TRAILER_LEN as i64))).ok()?;
    let mut magic = [0u8; 10];
    f.read_exact(&mut magic).ok()?;
    if &magic != MAGIC {
        return None;
    }

    let mut len_buf = [0u8; 4];
    f.read_exact(&mut len_buf).ok()?;
    let json_len = u32::from_le_bytes(len_buf) as u64;
    if json_len > len - TRAILER_LEN as u64 {
        panic!(
            "Corrupted bundle trailer: json_len {} exceeds file size",
            json_len
        );
    }

    f.seek(SeekFrom::End(-(TRAILER_LEN as i64 + json_len as i64)))
        .ok()?;
    let mut json_bytes = vec![0u8; json_len as usize];
    f.read_exact(&mut json_bytes).ok()?;

    serde_json::from_slice(&json_bytes).ok()
}

pub fn write_embedded(exe: &Path, output: &Path, bundle: &Bundle) -> std::io::Result<()> {
    let exe_bytes = fs::read(exe)?;
    fs::write(output, &exe_bytes)?;
    let json = serde_json::to_string(bundle).unwrap();

    if json.len() > u32::MAX as usize {
        panic!(
            "Bundle too large: {} bytes, max is {}",
            json.len(),
            u32::MAX
        );
    }

    let mut f = File::options().append(true).open(output)?;
    f.write_all(json.as_bytes())?;
    f.write_all(MAGIC)?;
    f.write_all(&(json.len() as u32).to_le_bytes())?;
    Ok(())
}
