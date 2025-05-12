use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key
};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Enumération des erreurs possibles lors du chiffrement/déchiffrement
#[derive(Error, Debug)]
pub enum CipherError {
    #[error("Erreur de chiffrement: {0}")]
    EncryptionError(String),
    
    #[error("Erreur de déchiffrement: {0}")]
    DecryptionError(String),
    
    #[error("Format invalide: {0}")]
    FormatError(String),
}

/// Structure contenant les données chiffrées
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Nonce encodé en base64
    pub nonce: String,
    
    /// Données chiffrées encodées en base64
    pub ciphertext: String,
}

/// Chiffre des données avec AES-256-GCM
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<EncryptedData, CipherError> {
    // Créer la clé AES
    let key = Key::<Aes256Gcm>::from_slice(key);
    
    // Créer la cipher AES-GCM
    let cipher = Aes256Gcm::new(key);
    
    // Générer un nonce aléatoire
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    // Chiffrer les données
    let ciphertext = cipher.encrypt(&nonce, plaintext)
        .map_err(|e| CipherError::EncryptionError(e.to_string()))?;
    
    // Encoder le nonce et les données chiffrées en base64 pour le stockage
    let nonce_b64 = general_purpose::STANDARD.encode(nonce);
    let ciphertext_b64 = general_purpose::STANDARD.encode(ciphertext);
    
    Ok(EncryptedData {
        nonce: nonce_b64,
        ciphertext: ciphertext_b64,
    })
}

/// Déchiffre des données avec AES-256-GCM
pub fn decrypt(encrypted_data: &EncryptedData, key: &[u8; 32]) -> Result<Vec<u8>, CipherError> {
    // Créer la clé AES
    let key = Key::<Aes256Gcm>::from_slice(key);
    
    // Créer la cipher AES-GCM
    let cipher = Aes256Gcm::new(key);
    
    // Décoder le nonce et les données chiffrées de base64
    let nonce_bytes = general_purpose::STANDARD.decode(&encrypted_data.nonce)
        .map_err(|e| CipherError::FormatError(format!("Format de nonce invalide: {}", e)))?;
    
    let ciphertext = general_purpose::STANDARD.decode(&encrypted_data.ciphertext)
        .map_err(|e| CipherError::FormatError(format!("Format de texte chiffré invalide: {}", e)))?;
    
    // Vérifier la taille du nonce (12 octets pour AES-GCM)
    if nonce_bytes.len() != 12 {
        return Err(CipherError::FormatError("Taille de nonce invalide".to_string()));
    }
    
    // Déchiffrer les données avec le nonce
    let plaintext = cipher.decrypt(nonce_bytes.as_slice().into(), ciphertext.as_ref())
        .map_err(|e| CipherError::DecryptionError(e.to_string()))?;
    
    Ok(plaintext)
}