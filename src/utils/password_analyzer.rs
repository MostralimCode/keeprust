use std::collections::HashSet;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordAnalysisError {
    #[error("Erreur d'analyse: {0}")]
    #[allow(dead_code)]
    AnalysisError(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

#[derive(Debug, Clone)]
pub struct PasswordAnalysis {
    pub strength: PasswordStrength,
    pub score: u8, // 0-100
    pub issues: Vec<String>,
    pub suggestions: Vec<String>,
}

pub struct PasswordAnalyzer {
    common_passwords: HashSet<String>,
}

impl PasswordAnalyzer {
    pub fn new() -> Self {
        let mut common_passwords = HashSet::new();
        
        // Ajouter des mots de passe communs
        let common = vec![
            "password", "123456", "password123", "admin", "qwerty",
            "letmein", "welcome", "monkey", "1234567890", "abc123",
            "password1", "123456789", "welcome123", "admin123",
            "root", "toor", "pass", "test", "guest", "user",
            "azerty", "motdepasse", "secret", "changeme", "default"
        ];
        
        for pwd in common {
            common_passwords.insert(pwd.to_lowercase());
        }
        
        Self { common_passwords }
    }
    
    pub fn analyze(&self, password: &str) -> PasswordAnalysis {
        let mut score = 0u8;
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();
        
        // Vérifier la longueur
        if password.len() < 8 {
            issues.push("Mot de passe trop court (moins de 8 caractères)".to_string());
            suggestions.push("Utilisez au moins 8 caractères".to_string());
        } else if password.len() >= 8 {
            score += 20;
        }
        
        if password.len() >= 12 {
            score += 10;
        }
        
        if password.len() >= 16 {
            score += 10;
        }
        
        // Vérifier la présence de différents types de caractères
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_digits = password.chars().any(|c| c.is_ascii_digit());
        let has_symbols = password.chars().any(|c| !c.is_alphanumeric());
        
        let char_types = [has_lowercase, has_uppercase, has_digits, has_symbols]
            .iter()
            .filter(|&&x| x)
            .count();
        
        match char_types {
            1 => {
                issues.push("Utilise seulement un type de caractères".to_string());
                suggestions.push("Mélangez majuscules, minuscules, chiffres et symboles".to_string());
            },
            2 => {
                score += 15;
                suggestions.push("Ajoutez des chiffres et/ou des symboles".to_string());
            },
            3 => {
                score += 25;
                suggestions.push("Ajoutez des symboles pour plus de sécurité".to_string());
            },
            4 => {
                score += 35;
            },
            _ => {}
        }
        
        // Vérifier les mots de passe communs
        if self.common_passwords.contains(&password.to_lowercase()) {
            score = 0; // Score à zéro pour les mots de passe communs
            issues.push("Mot de passe très commun et facilement devinable".to_string());
            suggestions.push("Utilisez le générateur pour créer un mot de passe unique".to_string());
        }
        
        // Vérifier les répétitions
        if self.has_repetitions(password) {
            score = score.saturating_sub(15);
            issues.push("Contient des répétitions de caractères".to_string());
            suggestions.push("Évitez les répétitions comme 'aaa' ou '111'".to_string());
        }
        
        // Vérifier les séquences
        if self.has_sequences(password) {
            score = score.saturating_sub(10);
            issues.push("Contient des séquences prévisibles".to_string());
            suggestions.push("Évitez les séquences comme 'abc' ou '123'".to_string());
        }
        
        // Déterminer la force (utiliser _ pour couvrir tous les cas restants)
        let strength = match score {
            0..=20 => PasswordStrength::VeryWeak,
            21..=40 => PasswordStrength::Weak,
            41..=60 => PasswordStrength::Medium,
            61..=80 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong, // Couvre 81 et plus
        };
        
        PasswordAnalysis {
            strength,
            score,
            issues,
            suggestions,
        }
    }
    
    fn has_repetitions(&self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();
        for window in chars.windows(3) {
            if window[0] == window[1] && window[1] == window[2] {
                return true;
            }
        }
        false
    }
    
    fn has_sequences(&self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();
        for window in chars.windows(3) {
            let c1 = window[0] as u8;
            let c2 = window[1] as u8;
            let c3 = window[2] as u8;
            
            // Vérifier les séquences croissantes ou décroissantes
            if (c2 == c1 + 1 && c3 == c2 + 1) || (c2 == c1 - 1 && c3 == c2 - 1) {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordStrength::VeryWeak => write!(f, "Très faible"),
            PasswordStrength::Weak => write!(f, "Faible"),
            PasswordStrength::Medium => write!(f, "Moyen"),
            PasswordStrength::Strong => write!(f, "Fort"),
            PasswordStrength::VeryStrong => write!(f, "Très fort"),
        }
    }
}

impl fmt::Display for PasswordAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Force: {} (Score: {}/100)", self.strength, self.score)?;
        
        if !self.issues.is_empty() {
            writeln!(f, "\nProblèmes détectés:")?;
            for issue in &self.issues {
                writeln!(f, "  ⚠️  {}", issue)?;
            }
        }
        
        if !self.suggestions.is_empty() {
            writeln!(f, "\nSuggestions:")?;
            for suggestion in &self.suggestions {
                writeln!(f, "  💡 {}", suggestion)?;
            }
        }
        
        Ok(())
    }
}