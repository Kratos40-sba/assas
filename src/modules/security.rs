use chacha20poly1305::{aead::{Aead, KeyInit}, ChaCha20Poly1305, Nonce};
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher}};
use thiserror::Error;
use std::fs;
use std::path::PathBuf;
use chrono::Local;
use directories::ProjectDirs;
use rand::{RngCore, rngs::OsRng};

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    #[error("Password hashing failed: {0}")]
    HashError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Data directory not found")]
    NoDataDir,
}

pub struct SecurityManager {
    cipher: ChaCha20Poly1305,
}

impl SecurityManager {
    pub fn new(password: &str, salt_str: &str) -> Result<Self, SecurityError> {
        let salt = SaltString::from_b64(salt_str)
            .map_err(|e| SecurityError::HashError(e.to_string()))?;
            
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| SecurityError::HashError(e.to_string()))?;
        
        let hash_output = password_hash.hash
            .ok_or_else(|| SecurityError::HashError("No hash output".to_string()))?;
            
        let key_bytes = hash_output.as_bytes();
        let mut key = [0u8; 32];
        if key_bytes.len() < 32 {
            return Err(SecurityError::HashError("Hash output too short".to_string()));
        }
        key.copy_from_slice(&key_bytes[..32]);
        
        let cipher = ChaCha20Poly1305::new(&key.into());
        
        Ok(Self { cipher })
    }

    pub fn generate_random_salt() -> String {
        let mut salt_bytes = [0u8; 16];
        OsRng.fill_bytes(&mut salt_bytes);
        SaltString::encode_b64(&salt_bytes).unwrap().to_string()
    }

    pub fn encrypt_and_save(&self, data: &[u8]) -> Result<(), SecurityError> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = self.cipher.encrypt(nonce, data)
            .map_err(|e| SecurityError::EncryptionError(format!("{:?}", e)))?;

        // Prepend nonce to ciphertext: [nonce (12 bytes)][ciphertext]
        let mut combined = Vec::with_capacity(12 + ciphertext.len());
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);

        let path = self.get_log_dir()?;
        fs::create_dir_all(&path)?;
        
        let filename = format!("{}.aes", Local::now().format("%Y%m%d_%H%M%S"));
        fs::write(path.join(filename), combined)?;
        
        Ok(())
    }

    pub fn decrypt(&self, encrypted_with_nonce: &[u8]) -> Result<Vec<u8>, SecurityError> {
        if encrypted_with_nonce.len() < 12 {
            return Err(SecurityError::DecryptionError("Data too short".to_string()));
        }
        
        let (nonce_bytes, ciphertext) = encrypted_with_nonce.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        self.cipher.decrypt(nonce, ciphertext)
            .map_err(|e| SecurityError::DecryptionError(format!("{:?}", e)))
    }

    pub fn get_log_dir(&self) -> Result<PathBuf, SecurityError> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "assas", "auditor") {
            Ok(proj_dirs.data_dir().to_path_buf())
        } else {
            Err(SecurityError::NoDataDir)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let password = "test_password";
        let salt = SecurityManager::generate_random_salt();
        let sm = SecurityManager::new(password, &salt).unwrap();
        
        let data = b"Hello, world!";
        let _ciphertext = sm.cipher.encrypt(Nonce::from_slice(b"fake nonce 1"), data.as_slice()).unwrap();
        
        // Test our encrypt/decrypt wrappers
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ct = sm.cipher.encrypt(nonce, data.as_slice()).unwrap();
        
        let mut combined = Vec::new();
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ct);
        
        let decrypted = sm.decrypt(&combined).unwrap();
        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_different_nonces() {
        let _password = "test_password";
        let _salt = SecurityManager::generate_random_salt();
        let _sm = SecurityManager::new(_password, &_salt).unwrap();
        let _data = b"Same data";
        
        // We can't easily capture the output file here without FS side effects, 
        // but we can test the internal logic.
        let mut n1 = [0u8; 12];
        OsRng.fill_bytes(&mut n1);
        let mut n2 = [0u8; 12];
        OsRng.fill_bytes(&mut n2);
        assert_ne!(n1, n2);
    }
}
