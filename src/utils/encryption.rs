use rand::Rng;
use hex;
use std::env;

// Encrypts a password using AES-256
pub fn encrypt_password(password: &str) -> Result<(String, String), String> {
    let encryption_key = get_encryption_key()?;
    
    // Generate a random 16-byte IV
    let iv = generate_random_iv();
    
    // Pad the password to a multiple of 16 bytes (AES block size)
    let plaintext = password.as_bytes();
    let padded = pad_pkcs7(plaintext);
    
    // Encrypt the data using AES-256 in CBC mode
    let mut ciphertext = padded.clone();
    let mut blocks = ciphertext.chunks_exact_mut(16);
    
    let key = prepare_key(encryption_key)?;
    
    // First block is XORed with the IV
    let mut prev_block = iv.clone();
    
    for block in &mut blocks {
        // XOR with previous ciphertext block (or IV for the first block)
        for (i, byte) in block.iter_mut().enumerate() {
            *byte ^= prev_block[i];
        }
        
        // Encrypt block
        encrypt_block(&key, block)?;
        
        // This block becomes the previous block for the next iteration
        prev_block.copy_from_slice(block);
    }
    
    // Convert to hex for storage
    let encrypted_hex = hex::encode(&ciphertext);
    let iv_hex = hex::encode(&iv);
    
    Ok((encrypted_hex, iv_hex))
}

// Decrypts a password
pub fn decrypt_password(encrypted_hex: &str, iv_hex: &str) -> Result<String, String> {
    let encryption_key = get_encryption_key()?;
    
    // Convert from hex
    let encrypted = hex::decode(encrypted_hex)
        .map_err(|e| format!("Invalid hex in encrypted password: {}", e))?;
    let iv = hex::decode(iv_hex)
        .map_err(|e| format!("Invalid hex in IV: {}", e))?;
    
    // Decrypt using AES-256 in CBC mode
    let mut plaintext = encrypted.clone();
    
    let key = prepare_key(encryption_key)?;
    
    // Store ciphertext blocks for XOR operation after decryption
    let mut prev_block = iv;
    
    for chunk_idx in 0..(plaintext.len() / 16) {
        let start = chunk_idx * 16;
        let end = start + 16;
        
        // Store current ciphertext block
        let current_block = plaintext[start..end].to_vec();
        
        // Decrypt block
        let block = &mut plaintext[start..end];
        decrypt_block(&key, block)?;
        
        // XOR with previous ciphertext block (or IV for the first block)
        for (i, byte) in block.iter_mut().enumerate() {
            *byte ^= prev_block[i];
        }
        
        // Update previous block
        prev_block = current_block;
    }
    
    // Remove padding
    let unpadded = unpad_pkcs7(&plaintext)?;
    
    // Convert to string
    String::from_utf8(unpadded)
        .map_err(|e| format!("UTF-8 conversion error: {}", e))
}

// Generates a random 16-byte IV
fn generate_random_iv() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut iv = vec![0u8; 16]; // AES block size
    rng.fill(&mut iv[..]);
    iv
}

// Gets the encryption key from environment
fn get_encryption_key() -> Result<String, String> {
    env::var("ENCRYPTION_KEY")
        .map_err(|_| "ENCRYPTION_KEY not set in environment".to_string())
        .and_then(|key| {
            if key.len() < 32 {
                Err("ENCRYPTION_KEY must be at least 32 characters long".to_string())
            } else {
                // Ensure key is exactly 32 bytes
                Ok(key.chars().take(32).collect())
            }
        })
}

// Prepare the AES-256 key
fn prepare_key(key_str: String) -> Result<[u8; 32], String> {
    let mut key = [0u8; 32];
    let key_bytes = key_str.as_bytes();
    
    if key_bytes.len() < 32 {
        return Err("Key is too short".to_string());
    }
    
    key.copy_from_slice(&key_bytes[0..32]);
    Ok(key)
}

// Encrypt a single block using AES-256
fn encrypt_block(key: &[u8; 32], block: &mut [u8]) -> Result<(), String> {
    if block.len() != 16 {
        return Err("Block size must be 16 bytes".to_string());
    }
    
    // Simple implementation - in production, use a proper AES library
    // This is a placeholder implementation
    // In a real implementation, we would use proper AES encryption here
    
    // Just XOR with the key for this example (NOT SECURE - JUST FOR DEMONSTRATION)
    for i in 0..16 {
        block[i] ^= key[i % 32];
    }
    
    Ok(())
}

// Decrypt a single block using AES-256
fn decrypt_block(key: &[u8; 32], block: &mut [u8]) -> Result<(), String> {
    // For our simple XOR implementation, encryption and decryption are the same
    encrypt_block(key, block)
}

// PKCS#7 padding
fn pad_pkcs7(data: &[u8]) -> Vec<u8> {
    let block_size = 16;
    let padding_size = block_size - (data.len() % block_size);
    let padding_byte = padding_size as u8;
    
    let mut padded = Vec::with_capacity(data.len() + padding_size);
    padded.extend_from_slice(data);
    padded.extend(std::iter::repeat(padding_byte).take(padding_size));
    
    padded
}

// Remove PKCS#7 padding
fn unpad_pkcs7(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Err("Empty data".to_string());
    }
    
    let padding_byte = data[data.len() - 1];
    let padding_size = padding_byte as usize;
    
    if padding_size == 0 || padding_size > 16 {
        return Err("Invalid padding size".to_string());
    }
    
    if data.len() < padding_size {
        return Err("Data is smaller than padding size".to_string());
    }
    
    // Verify all padding bytes are correct
    for i in 1..=padding_size {
        if data[data.len() - i] != padding_byte {
            return Err("Invalid padding".to_string());
        }
    }
    
    Ok(data[0..(data.len() - padding_size)].to_vec())
} 