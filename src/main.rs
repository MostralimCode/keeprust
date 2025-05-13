mod cli;
mod crypto;
mod models;
mod utils;

use models::entry::Entry;
use models::group::Group;
use models::database::Database;
use crypto::{cipher, key_derivation};
use models::repository::Repository;
use std::path::Path;
use serde_json;

fn main() {
	println!("KeepRust - Gestionnaire de mots de passe sécurisé");

	// Test de persistance (repository)
    println!("\nTest de persistance:");
    
    // Créer une base de données avec quelques entrées
    let mut db = Database::new(
        "Ma base de mots de passe".to_string(),
        "Base de données personnelle pour mes mots de passe".to_string()
    );
    
    // Ajouter un groupe pour les emails
    let email_group = Group::new("Email".to_string(), Some(db.root_group.id.clone()));
    let email_group_id = email_group.id.clone();
    db.root_group.add_subgroup(email_group);
    
    // Ajouter une entrée Gmail
    let gmail_entry = Entry::new(
        "Gmail".to_string(),
        "utilisateur@gmail.com".to_string(),
        "mot_de_passe_secure_123".to_string(),
        "https://gmail.com".to_string(),
        "Mon compte email principal".to_string()
    );
    
    if let Some(group) = db.find_group_mut(&email_group_id) {
        group.add_entry(gmail_entry);
        println!("Entrée Gmail ajoutée");
    }
    
    // Ajouter un groupe pour les réseaux sociaux
    let social_group = Group::new("Réseaux sociaux".to_string(), Some(db.root_group.id.clone()));
    let social_group_id = social_group.id.clone();
    db.root_group.add_subgroup(social_group);
    
    // Ajouter une entrée Facebook
    let facebook_entry = Entry::new(
        "Facebook".to_string(),
        "user@example.com".to_string(),
        "facebook_pwd_123!".to_string(),
        "https://facebook.com".to_string(),
        "Mon compte Facebook personnel".to_string()
    );
    
    if let Some(group) = db.find_group_mut(&social_group_id) {
        group.add_entry(facebook_entry);
        println!("Entrée Facebook ajoutée");
    }
    
    // Chemin du fichier de test
    let test_file_path = Path::new("test_database.krs");
    
    // Créer un repository
    let repo = Repository::new(test_file_path);
    
    // Mot de passe pour le test
    let password = "mot_de_passe_maitre_123!";
    
    // Sauvegarder la base de données
    match repo.save(&db, password) {
        Ok(_) => println!("Base de données sauvegardée avec succès dans {:?}", test_file_path),
        Err(e) => {
            println!("Erreur lors de la sauvegarde: {}", e);
            return;
        }
    }
    
    // Charger la base de données
    let loaded_db = match repo.load(password) {
        Ok(db) => {
            println!("Base de données chargée avec succès");
            db
        },
        Err(e) => {
            println!("Erreur lors du chargement: {}", e);
            return;
        }
    };
    
    // Vérifier que les données sont identiques
    println!("\nVérification des données après chargement:");
    println!("Nom: {}", loaded_db.metadata.name);
    println!("Description: {}", loaded_db.metadata.description);
    
    // Vérifier les groupes
    println!("\nGroupes et entrées:");
    for (id, subgroup) in &loaded_db.root_group.subgroups {
        println!("  Groupe: {} (ID: {})", subgroup.name, id);
        
        for (entry_id, entry) in &subgroup.entries {
            println!("    Entrée: {} (ID: {})", entry.title, entry_id);
            println!("      Utilisateur: {}", entry.username);
        }
    }
    
    // Test avec un mauvais mot de passe (devrait échouer)
    println!("\nTest avec un mauvais mot de passe (devrait échouer):");
    match repo.load("mauvais_mot_de_passe") {
        Ok(_) => println!("ERREUR: La base a été déchiffrée avec un mauvais mot de passe!"),
        Err(e) => println!("Échec attendu: {}", e),
    }
    
    println!("\nTest de persistance terminé avec succès!");
    
    // Nettoyage: supprimer le fichier de test
    if let Err(e) = std::fs::remove_file(test_file_path) {
        println!("Impossible de supprimer le fichier de test: {}", e);
    }
} 