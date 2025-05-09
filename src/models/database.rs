use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::group::Group;
use super::entry::Entry;

/// Contient les métadonnées de la base de données
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    /// Nom de la base de données
    pub name: String,
    
    /// Description de la base de données
    pub description: String,
    
    /// Date et heure de création
    pub created_at: DateTime<Utc>,
    
    /// Date et heure de dernière modification
    pub updated_at: DateTime<Utc>,
    
    /// Version du format de la base de données
    pub version: String,
}

/// Représente une base de données complète de mots de passe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    /// Métadonnées de la base de données
    pub metadata: DatabaseMetadata,
    
    /// Groupe racine contenant tous les autres groupes et entrées
    pub root_group: Group,
    
    /// Indique si la base de données a été modifiée depuis le dernier enregistrement
    #[serde(skip)]
    pub is_modified: bool,
}

impl Database {
    /// Crée une nouvelle base de données vide
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            metadata: DatabaseMetadata {
                name,
                description,
                created_at: now,
                updated_at: now,
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            root_group: Group::new("Root".to_string(), None),
            is_modified: true,
        }
    }
    
    /// Recherche une entrée par son ID dans toute la base de données
    pub fn find_entry<'a>(&'a self, entry_id: &str) -> Option<&'a Entry> {
        self.find_entry_in_group(&self.root_group, entry_id)
    }
    
    /// Fonction récursive pour rechercher une entrée dans un groupe et ses sous-groupes
    fn find_entry_in_group<'a>(&self, group: &'a Group, entry_id: &str) -> Option<&'a Entry> {
        // Recherche dans les entrées directes du groupe
        if let Some(entry) = group.entries.get(entry_id) {
            return Some(entry);
        }
        
        // Recherche récursive dans les sous-groupes
        for subgroup in group.subgroups.values() {
            if let Some(entry) = self.find_entry_in_group(subgroup, entry_id) {
                return Some(entry);
            }
        }
        
        None
    }
    
    /// Recherche un groupe par son ID dans toute la base de données
    pub fn find_group<'a>(&'a self, group_id: &str) -> Option<&'a Group> {
        if self.root_group.id == group_id {
            return Some(&self.root_group);
        }
        self.find_group_in_group(&self.root_group, group_id)
    }
    
    /// Fonction récursive pour rechercher un groupe dans un groupe et ses sous-groupes
    fn find_group_in_group<'a>(&self, group: &'a Group, group_id: &str) -> Option<&'a Group> {
        // Recherche dans les sous-groupes directs
        if let Some(found_group) = group.subgroups.get(group_id) {
            return Some(found_group);
        }
        
        // Recherche récursive dans les sous-groupes
        for subgroup in group.subgroups.values() {
            if let Some(found_group) = self.find_group_in_group(subgroup, group_id) {
                return Some(found_group);
            }
        }
        
        None
    }
    
    /// Recherche un groupe mutable par son ID
    pub fn find_group_mut(&mut self, group_id: &str) -> Option<&mut Group> {
        // Cas spécial pour le groupe racine
        if self.root_group.id == group_id {
            return Some(&mut self.root_group);
        }
        
        // Une approche plus simple avec des vecteurs intermédiaires
        // 1. Trouver le chemin au groupe (en IDs)
        let path = self.find_path_to_group(&self.root_group, group_id, Vec::new())?;
        
        // 2. Suivre ce chemin en version mutable
        let mut current = &mut self.root_group;
        for id in path.iter().take(path.len() - 1) {  // Tous sauf le dernier
            current = current.subgroups.get_mut(id)?;
        }
        
        // 3. Obtenir le groupe cible (dernier ID du chemin)
        current.subgroups.get_mut(path.last()?)
    }

    /// Trouve le chemin (en IDs) vers un groupe spécifique
    fn find_path_to_group(&self, group: &Group, target_id: &str, mut path: Vec<String>) -> Option<Vec<String>> {
        // Vérifier les sous-groupes directs
        if group.subgroups.contains_key(target_id) {
            path.push(target_id.to_string());
            return Some(path);
        }
        
        // Recherche récursive dans les sous-groupes
        for (id, subgroup) in &group.subgroups {
            let mut new_path = path.clone();
            new_path.push(id.clone());
            
            if let Some(result) = self.find_path_to_group(subgroup, target_id, new_path) {
                return Some(result);
            }
        }
        
        None
    }
    
    /// Marque la base de données comme modifiée et met à jour l'horodatage
    pub fn mark_as_modified(&mut self) {
        self.is_modified = true;
        self.metadata.updated_at = Utc::now();
    }
}