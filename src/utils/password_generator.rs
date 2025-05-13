use rand::{Rng, thread_rng};
use thiserror::Error;

/// Enumération des erreurs possibles lors de la génération de mots de passe
#[derive(Error, Debug)]
pub enum PasswordGenError {
    #[error("Longueur de mot de passe invalide: {0}")]
    InvalidLength(String),
    
    #[error("Aucun jeu de caractères n'est activé")]
    NoCharsetEnabled,
}

/// Configuration pour le générateur de mots de passe
#[derive(Debug, Clone)]
pub struct PasswordGenerator {
    /// Longueur du mot de passe
    length: usize,
    
    /// Utiliser des lettres majuscules (A-Z)
    use_uppercase: bool,
    
    /// Utiliser des lettres minuscules (a-z)
    use_lowercase: bool,
    
    /// Utiliser des chiffres (0-9)
    use_digits: bool,
    
    /// Utiliser des symboles (!@#$%^&*...)
    use_symbols: bool,
    
    /// Exclure les caractères similaires (I, l, 1, O, 0, etc.)
    exclude_similar: bool,
    
    /// Exclure les caractères ambigus ({}[]()/\'"`~,;:.<>)
    exclude_ambiguous: bool,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            length: 16,
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_symbols: true,
            exclude_similar: false,
            exclude_ambiguous: false,
        }
    }
}

impl PasswordGenerator {
    /// Crée un nouveau générateur avec la configuration par défaut
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Définit la longueur du mot de passe
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }
    
    /// Active ou désactive l'utilisation des lettres majuscules
    pub fn uppercase(mut self, use_uppercase: bool) -> Self {
        self.use_uppercase = use_uppercase;
        self
    }
    
    /// Active ou désactive l'utilisation des lettres minuscules
    pub fn lowercase(mut self, use_lowercase: bool) -> Self {
        self.use_lowercase = use_lowercase;
        self
    }
    
    /// Active ou désactive l'utilisation des chiffres
    pub fn digits(mut self, use_digits: bool) -> Self {
        self.use_digits = use_digits;
        self
    }
    
    /// Active ou désactive l'utilisation des symboles
    pub fn symbols(mut self, use_symbols: bool) -> Self {
        self.use_symbols = use_symbols;
        self
    }
    
    /// Exclut ou inclut les caractères similaires
    pub fn exclude_similar(mut self, exclude: bool) -> Self {
        self.exclude_similar = exclude;
        self
    }
    
    /// Exclut ou inclut les caractères ambigus
    pub fn exclude_ambiguous(mut self, exclude: bool) -> Self {
        self.exclude_ambiguous = exclude;
        self
    }
    
    /// Génère un mot de passe avec la configuration actuelle
    pub fn generate(&self) -> Result<String, PasswordGenError> {
        // Vérifier la longueur minimale
        if self.length == 0 {
            return Err(PasswordGenError::InvalidLength("La longueur doit être supérieure à 0".to_string()));
        }
        
        // Construire l'ensemble de caractères
        let mut charset = String::new();
        
        // Lettres majuscules
        if self.use_uppercase {
            if self.exclude_similar {
                charset.push_str("ABCDEFGHJKLMNPQRSTUVWXYZ"); // Sans I et O
            } else {
                charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            }
        }
        
        // Lettres minuscules
        if self.use_lowercase {
            if self.exclude_similar {
                charset.push_str("abcdefghijkmnpqrstuvwxyz"); // Sans l et o
            } else {
                charset.push_str("abcdefghijklmnopqrstuvwxyz");
            }
        }
        
        // Chiffres
        if self.use_digits {
            if self.exclude_similar {
                charset.push_str("23456789"); // Sans 0 et 1
            } else {
                charset.push_str("0123456789");
            }
        }
        
        // Symboles
        if self.use_symbols {
            if self.exclude_ambiguous {
                charset.push_str("!@#$%^&*-_=+"); // Symboles non ambigus
            } else {
                charset.push_str("!@#$%^&*()-_=+[]{};:'\",.<>/?\\|`~");
            }
        }
        
        // Vérifier qu'au moins un jeu de caractères est activé
        if charset.is_empty() {
            return Err(PasswordGenError::NoCharsetEnabled);
        }
        
        // Convertir en vecteur de caractères pour un accès aléatoire efficace
        let charset: Vec<char> = charset.chars().collect();
        
        // Générer le mot de passe
        let mut rng = thread_rng();
        let password: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx]
            })
            .collect();
        
        Ok(password)
    }
    
    /// Génère un mot de passe respectant certaines règles de complexité
    pub fn generate_complex(&self) -> Result<String, PasswordGenError> {
        // Vérifier que les options minimales sont activées
        if !self.use_uppercase && !self.use_lowercase && !self.use_digits && !self.use_symbols {
            return Err(PasswordGenError::NoCharsetEnabled);
        }
        
        // Vérifier la longueur minimale pour la complexité
        if self.length < 8 {
            return Err(PasswordGenError::InvalidLength("La longueur doit être d'au moins 8 caractères pour un mot de passe complexe".to_string()));
        }
        
        // Essayer de générer un mot de passe complexe jusqu'à 10 tentatives
        for _ in 0..10 {
            let password = self.generate()?;
            
            // Vérifier que le mot de passe contient au moins:
            let has_uppercase = self.use_uppercase && password.chars().any(|c| c.is_ascii_uppercase());
            let has_lowercase = self.use_lowercase && password.chars().any(|c| c.is_ascii_lowercase());
            let has_digit = self.use_digits && password.chars().any(|c| c.is_ascii_digit());
            let has_symbol = self.use_symbols && password.chars().any(|c| !c.is_alphanumeric());
            
            // Calculer le nombre de conditions activées et satisfaites
            let conditions_enabled = [self.use_uppercase, self.use_lowercase, self.use_digits, self.use_symbols]
                .iter().filter(|&&c| c).count();
            
            let conditions_met = [has_uppercase, has_lowercase, has_digit, has_symbol]
                .iter().filter(|&&c| c).count();
            
            // Si toutes les conditions activées sont satisfaites, retourner le mot de passe
            if conditions_met == conditions_enabled {
                return Ok(password);
            }
        }
        
        // Si on arrive ici, c'est qu'on n'a pas réussi à générer un mot de passe complexe
        // Générer un simple mot de passe (sans garanties de complexité)
        self.generate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_generator() {
        let generator = PasswordGenerator::default();
        let password = generator.generate().unwrap();
        
        assert_eq!(password.len(), 16);
    }
    
    #[test]
    fn test_custom_length() {
        let generator = PasswordGenerator::new().length(20);
        let password = generator.generate().unwrap();
        
        assert_eq!(password.len(), 20);
    }
    
    #[test]
    fn test_only_lowercase() {
        let generator = PasswordGenerator::new()
            .uppercase(false)
            .digits(false)
            .symbols(false);
        
        let password = generator.generate().unwrap();
        
        assert!(password.chars().all(|c| c.is_ascii_lowercase()));
    }
    
    #[test]
    fn test_complex_password() {
        let generator = PasswordGenerator::new().length(12);
        let password = generator.generate_complex().unwrap();
        
        assert_eq!(password.len(), 12);
        assert!(password.chars().any(|c| c.is_ascii_uppercase()));
        assert!(password.chars().any(|c| c.is_ascii_lowercase()));
        assert!(password.chars().any(|c| c.is_ascii_digit()));
        assert!(password.chars().any(|c| !c.is_alphanumeric()));
    }
    
    #[test]
    fn test_exclude_similar() {
        let generator = PasswordGenerator::new().exclude_similar(true);
        let password = generator.generate().unwrap();
        
        assert!(!password.contains(|c| "IlO01".contains(c)));
    }
}