use clipboard::{ClipboardContext, ClipboardProvider};
use std::thread;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Erreur d'accès au presse-papiers: {0}")]
    AccessError(String),
    
    #[error("Presse-papiers non disponible")]
    NotAvailable,
}

/// Gestionnaire du presse-papiers avec effacement automatique
pub struct SecureClipboard {
    context: ClipboardContext,
}

impl SecureClipboard {
    /// Crée un nouveau gestionnaire de presse-papiers
    pub fn new() -> Result<Self, ClipboardError> {
        let context = ClipboardProvider::new()
            .map_err(|e| ClipboardError::AccessError(e.to_string()))?;
        
        Ok(Self { context })
    }
    
    /// Copie un texte dans le presse-papiers
    pub fn copy(&mut self, text: &str) -> Result<(), ClipboardError> {
        self.context.set_contents(text.to_owned())
            .map_err(|e| ClipboardError::AccessError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Copie un texte dans le presse-papiers et l'efface après un délai
    pub fn copy_with_timeout(&mut self, text: &str, timeout_seconds: u64) -> Result<(), ClipboardError> {
        // Copier le texte
        self.copy(text)?;
        
        println!("Texte copié dans le presse-papiers.");
        if timeout_seconds > 0 {
            println!("Le presse-papiers sera effacé dans {} secondes.", timeout_seconds);
            
            // Lancer un thread pour effacer le presse-papiers après le délai
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(timeout_seconds));
                
                // Créer un nouveau contexte pour le thread
                let mut context: ClipboardContext = match ClipboardProvider::new() {
                    Ok(ctx) => ctx,
                    Err(_) => {
                        eprintln!("Impossible d'accéder au presse-papiers pour l'effacement");
                        return;
                    }
                };
                
                // Effacer le presse-papiers
                if let Err(e) = context.set_contents(String::new()) {
                    eprintln!("Erreur lors de l'effacement du presse-papiers: {}", e);
                } else {
                    println!("Presse-papiers effacé automatiquement.");
                }
            });
        }
        
        Ok(())
    }
    
    /// Lit le contenu du presse-papiers
    pub fn read(&mut self) -> Result<String, ClipboardError> {
        self.context.get_contents()
            .map_err(|e| ClipboardError::AccessError(e.to_string()))
    }
    
    /// Efface le presse-papiers
    pub fn clear(&mut self) -> Result<(), ClipboardError> {
        self.context.set_contents(String::new())
            .map_err(|e| ClipboardError::AccessError(e.to_string()))?;
        
        println!("Presse-papiers effacé.");
        Ok(())
    }
}

/// Copie un mot de passe dans le presse-papiers de manière sécurisée
pub fn copy_password_securely(password: &str, timeout_seconds: u64) -> Result<(), ClipboardError> {
    let mut clipboard = SecureClipboard::new()?;
    clipboard.copy_with_timeout(password, timeout_seconds)?;
    Ok(())
}

/// Copie un texte dans le presse-papiers sans effacement automatique
pub fn copy_to_clipboard(text: &str) -> Result<(), ClipboardError> {
    let mut clipboard = SecureClipboard::new()?;
    clipboard.copy(text)?;
    println!("Texte copié dans le presse-papiers.");
    Ok(())
}