use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::crypto::{cipher, key_derivation};
use crate::models::database::Database;

/// Signature de fichier pour identifier notre format
const FILE_HEADER: &[u8] = b"KEEPRUST";
/// Version du format de fichier
const FILE_VERSION: u8 = 1;

/// Enumération des erreurs possibles lors des opérations de repository
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Erreur d'entrée/sortie: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Erreur de sérialisation: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Erreur de chiffrement: {0}")]
    EncryptionError(#[from] cipher::CipherError),
    
    #[error("Erreur de dérivation de clé: {0}")]
    KeyDerivationError(#[from] key_derivation::KeyDerivationError),
    
    #[error("Format de fichier invalide")]
    InvalidFileFormat,
    
    #[error("Version de fichier non supportée")]
    UnsupportedFileVersion,
    
    #[error("Mot de passe invalide")]
    InvalidPassword,
}

/// En-tête du fichier contenant les métadonnées nécessaires pour le déchiffrement
#[derive(Debug, Serialize, Deserialize)]
struct FileHeader {
    salt: String,
    iterations: u32,
}

/// Gère la persistance de la base de données
pub struct Repository {
    path: PathBuf,
}

impl Repository {
    /// Crée un nouveau repository pointant vers le chemin spécifié
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
    
    /// Sauvegarde la base de données dans un fichier chiffré
    pub fn save(&self, db: &Database, password: &str) -> Result<(), RepositoryError> {
        // Sérialiser la base de données en JSON
        let json_data = serde_json::to_string(db)?;
        
        // Dériver une clé à partir du mot de passe
        let iterations = 100_000; // Un nombre élevé d'itérations pour une meilleure sécurité
        let master_key = key_derivation::derive_key_from_password(password, iterations)?;
        
        // Chiffrer les données JSON
        let encrypted_data = cipher::encrypt(json_data.as_bytes(), &master_key.key)?;
        
        // Créer un en-tête avec le sel et les itérations
        let header = FileHeader {
            salt: master_key.salt.clone(),
            iterations,
        };
        
        // Sérialiser l'en-tête
        let header_json = serde_json::to_string(&header)?;
        let header_length = header_json.len() as u16;
        
        // Écrire dans le fichier
        let mut file = File::create(&self.path)?;
        
        // Écrire la signature magique
        file.write_all(FILE_HEADER)?;
        
        // Écrire la version
        file.write_all(&[FILE_VERSION])?;
        
        // Écrire la longueur de l'en-tête (2 octets en big-endian)
        file.write_all(&header_length.to_be_bytes())?;
        
        // Écrire l'en-tête
        file.write_all(header_json.as_bytes())?;
        
        // Écrire les données chiffrées
        let encrypted_json = serde_json::to_string(&encrypted_data)?;
        file.write_all(encrypted_json.as_bytes())?;
        
        Ok(())
    }
    
    /// Charge la base de données à partir d'un fichier chiffré
    pub fn load(&self, password: &str) -> Result<Database, RepositoryError> {
        // Lire le fichier
        let mut file = File::open(&self.path)?;
        
        // Lire et vérifier la signature magique
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)?;
        
        if &magic != FILE_HEADER {
            return Err(RepositoryError::InvalidFileFormat);
        }
        
        // Lire et vérifier la version
        let mut version = [0u8; 1];
        file.read_exact(&mut version)?;
        
        if version[0] != FILE_VERSION {
            return Err(RepositoryError::UnsupportedFileVersion);
        }
        
        // Lire la longueur de l'en-tête
        let mut header_len_bytes = [0u8; 2];
        file.read_exact(&mut header_len_bytes)?;
        let header_len = u16::from_be_bytes(header_len_bytes) as usize;
        
        // Lire l'en-tête
        let mut header_bytes = vec![0u8; header_len];
        file.read_exact(&mut header_bytes)?;
        
        let header: FileHeader = serde_json::from_slice(&header_bytes)?;
        
        // Lire le reste du fichier (données chiffrées)
        let mut encrypted_bytes = Vec::new();
        file.read_to_end(&mut encrypted_bytes)?;
        
        let encrypted_data: cipher::EncryptedData = serde_json::from_slice(&encrypted_bytes)?;
        
        // Dériver la clé à partir du mot de passe et du sel
        let master_key = key_derivation::verify_and_derive_key(
            password, 
            &header.salt, 
            header.iterations
        )?;
        
        // Déchiffrer les données
        let json_data = match cipher::decrypt(&encrypted_data, &master_key.key) {
            Ok(data) => data,
            Err(_) => return Err(RepositoryError::InvalidPassword),
        };
        
        // Désérialiser en Database
        let db: Database = serde_json::from_slice(&json_data)?;
        
        Ok(db)
    }
    
    /// Vérifie si le fichier de base de données existe
    #[allow(dead_code)]
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}