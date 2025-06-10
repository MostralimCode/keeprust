use ring::{pbkdf2, rand::{SecureRandom, SystemRandom}};
use std::num::NonZeroU32;
use thiserror::Error;
use base64::{Engine, engine::general_purpose};

/// Enumération des erreurs possibles lors de la dérivation de clé
#[derive(Error, Debug)]
pub enum KeyDerivationError {
    #[error("Erreur lors de la dérivation de clé: {0}")]
    DerivationError(String),
}

/// Structure contenant la clé dérivée et le sel utilisé
#[derive(Debug)]
#[allow(dead_code)]
pub struct MasterKey {
    /// Sel utilisé pour la dérivation (encodé en base64)
    pub salt: String,
    
    /// Clé dérivée (32 octets)
    pub key: [u8; 32],
    
    /// Nombre d'itérations utilisées
    pub iterations: u32,
}

/// Dérive une clé à partir d'un mot de passe
pub fn derive_key_from_password(password: &str, iterations: u32) -> Result<MasterKey, KeyDerivationError> {
    // Générer un sel aléatoire
    let rng = SystemRandom::new();
    let mut salt_bytes = [0u8; 16];
    rng.fill(&mut salt_bytes).map_err(|_| KeyDerivationError::DerivationError("Échec de génération de sel aléatoire".to_string()))?;
    
    // Convertir le sel en base64 pour le stockage
    let salt = general_purpose::STANDARD.encode(salt_bytes);
    
    // Dériver la clé
    let mut key = [0u8; 32]; // 256 bits
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        &salt_bytes,
        password.as_bytes(),
        &mut key,
    );
    
    Ok(MasterKey {
        salt,
        key,
        iterations,
    })
}

/// Vérifie un mot de passe et dérive la clé correspondante
pub fn verify_and_derive_key(password: &str, salt: &str, iterations: u32) -> Result<MasterKey, KeyDerivationError> {
    // Décoder le sel de base64
    let salt_bytes = general_purpose::STANDARD.decode(salt)
        .map_err(|e| KeyDerivationError::DerivationError(format!("Format de sel invalide: {}", e)))?;
    
    // Dériver la clé
    let mut key = [0u8; 32]; // 256 bits
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        &salt_bytes,
        password.as_bytes(),
        &mut key,
    );
    
    Ok(MasterKey {
        salt: salt.to_string(),
        key,
        iterations,
    })
}