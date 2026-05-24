use rand::TryRngCore;
use rand::rngs::OsRng;

// 64 Zeichen (2^6) → kein Modulo-Bias mit u8 % 64
const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz23456789!@#$%^&*";

#[tauri::command]
pub fn generate_password(length: usize) -> Result<String, String> {
    if length < 8 || length > 128 {
        return Err("Length must be between 8 and 128".to_string());
    }

    let mut rng = OsRng;
    let mut buf = vec![0u8; length];
    rng.try_fill_bytes(&mut buf)
        .map_err(|e| format!("RNG error: {e}"))?;

    let password: String = buf
        .iter()
        .map(|&b| CHARSET[b as usize % CHARSET.len()] as char)
        .collect();

    Ok(password)
}
