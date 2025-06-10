use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use super::entry::Entry;

/// Représente un groupe ou dossier pour organiser les entrées
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Identifiant unique pour ce groupe
    pub id: String,
    
    /// Nom du groupe
    pub name: String,
    
    /// Entrées contenues dans ce groupe
    pub entries: HashMap<String, Entry>,
    
    /// Sous-groupes contenus dans ce groupe
    pub subgroups: HashMap<String, Group>,
    
    /// ID du groupe parent (None pour le groupe racine)
    pub parent_id: Option<String>,
    
    /// Date et heure de création du groupe
    pub created_at: DateTime<Utc>,
    
    /// Date et heure de dernière modification
    pub updated_at: DateTime<Utc>,
}

impl Group {
    /// Crée un nouveau groupe avec le nom spécifié
    pub fn new(name: String, parent_id: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            entries: HashMap::new(),
            subgroups: HashMap::new(),
            parent_id,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Ajoute une entrée au groupe
    pub fn add_entry(&mut self, entry: Entry) -> &Entry {
        let id = entry.id.clone();
        self.entries.insert(id.clone(), entry);
        self.updated_at = Utc::now();
        self.entries.get(&id).unwrap()
    }
    
    /// Ajoute un sous-groupe à ce groupe
    pub fn add_subgroup(&mut self, group: Group) -> &Group {
        let id = group.id.clone();
        self.subgroups.insert(id.clone(), group);
        self.updated_at = Utc::now();
        self.subgroups.get(&id).unwrap()
    }
    
    /// Récupère une entrée par son ID
    #[allow(dead_code)]
    pub fn get_entry(&self, id: &str) -> Option<&Entry> {
        self.entries.get(id)
    }
    
    /// Récupère une entrée mutable par son ID
    #[allow(dead_code)]
    pub fn get_entry_mut(&mut self, id: &str) -> Option<&mut Entry> {
        self.entries.get_mut(id)
    }
    
    /// Supprime une entrée par son ID
    #[allow(dead_code)]
    pub fn remove_entry(&mut self, id: &str) -> Option<Entry> {
        let result = self.entries.remove(id);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }
    
    /// Récupère un sous-groupe par son ID
    #[allow(dead_code)]
    pub fn get_subgroup(&self, id: &str) -> Option<&Group> {
        self.subgroups.get(id)
    }
    
    /// Récupère un sous-groupe mutable par son ID
    #[allow(dead_code)]
    pub fn get_subgroup_mut(&mut self, id: &str) -> Option<&mut Group> {
        self.subgroups.get_mut(id)
    }
    
    /// Supprime un sous-groupe par son ID
    #[allow(dead_code)]
    pub fn remove_subgroup(&mut self, id: &str) -> Option<Group> {
        let result = self.subgroups.remove(id);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }
}