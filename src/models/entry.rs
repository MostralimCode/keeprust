use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;
use std::fmt;
use std::collections::VecDeque;


// Modifier la structure Entry pour inclure l'historique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHistory {
    pub password: String,
    pub changed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// Identifiant unique pour cette entrée
    pub id: String,
    
    /// Titre descriptif de l'entrée (ex: "Gmail", "Facebook", etc.)
    pub title: String,
    
    /// Nom d'utilisateur ou email associé
    pub username: String,
    
    /// Mot de passe actuel
    pub password: String,
    
    /// Historique des anciens mots de passe (limité aux 10 derniers)
    #[serde(default)]
    pub password_history: VecDeque<PasswordHistory>,
    
    /// URL du site ou du service
    pub url: String,
    
    /// Notes supplémentaires
    pub notes: String,
    
    /// Date et heure de création de l'entrée
    pub created_at: DateTime<Utc>,
    
    /// Date et heure de dernière modification
    pub updated_at: DateTime<Utc>,
}

impl Entry {
    /// Crée une nouvelle entrée avec les informations fournies
    pub fn new(title: String, username: String, password: String, url: String, notes: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            username,
            password,
            password_history: VecDeque::new(),
            url,
            notes,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Met à jour l'entrée et actualise l'horodatage de modification
    #[allow(dead_code)]
    pub fn update(&mut self, title: Option<String>, username: Option<String>, 
                  password: Option<String>, url: Option<String>, _notes: Option<String>) {
        if let Some(title) = title {
            self.title = title;
        }
        
        if let Some(username) = username {
            self.username = username;
        }
        
        if let Some(mut new_password) = password {
            // Ajouter l'ancien mot de passe à l'historique
            if !self.password.is_empty() {
                let old_password = PasswordHistory {
                    password: self.password.clone(),
                    changed_at: Utc::now(),
                };
                
                self.password_history.push_front(old_password);
                
                // Limiter l'historique à 10 entrées
                if self.password_history.len() > 10 {
                    if let Some(mut removed) = self.password_history.pop_back() {
                        removed.password.zeroize();
                    }
                }
            }
            
            // Effacer l'ancien mot de passe de manière sécurisée
            self.password.zeroize();
            // Remplacer par le nouveau
            self.password = new_password.clone();
            // Effacer la copie locale
            new_password.zeroize();
        }
        
        if let Some(url) = url {
            self.url = url;
        }
        
        if let Some(notes) = _notes {
            self.notes = notes;
        }
        
        // Mise à jour de l'horodatage
        self.updated_at = Utc::now();
    }
    
    /// Vérifie si un mot de passe a déjà été utilisé
    #[allow(dead_code)]
    pub fn has_used_password(&self, password: &str) -> bool {
        if self.password == password {
            return true;
        }
        
        self.password_history.iter().any(|h| h.password == password)
    }
    
    /// Efface le mot de passe de manière sécurisée
    #[allow(dead_code)]
    pub fn clear_password(&mut self) {
        self.password.zeroize();
    }
    
    /// Efface tout l'historique des mots de passe
    pub fn clear_password_history(&mut self) {
        for mut history_entry in self.password_history.drain(..) {
            history_entry.password.zeroize();
        }
    }
}

impl Drop for Entry {
    /// Efface le mot de passe de la mémoire lorsque l'entrée est détruite
    fn drop(&mut self) {
        self.password.zeroize();
        self.clear_password_history();
    }
}

impl fmt::Display for Entry {
    /// Formate l'entrée pour l'affichage (sans révéler le mot de passe)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Titre: {}\nUtilisateur: {}\nURL: {}\nCréé le: {}\nModifié le: {}",
            self.title,
            self.username,
            self.url,
            self.created_at.format("%d-%m-%Y %H:%M:%S"),
            self.updated_at.format("%d-%m-%Y %H:%M:%S")
        )
    }
}