mod cli;
mod crypto;
mod models;
mod utils;

use models::entry::Entry;
use models::group::Group;
use models::database::Database;

fn main() {
	println!("KeepRust - Gestionnaire de mots de passe sécurisé");

	 // Créer une nouvelle base de données
	 let mut db = Database::new(
        "Ma base de mots de passe".to_string(),
        "Base de données personnelle pour mes mots de passe".to_string()
    );
    
    // Afficher les métadonnées
    println!("\nBase de données créée:");
    println!("Nom: {}", db.metadata.name);
    println!("Description: {}", db.metadata.description);
    println!("Créée le: {}", db.metadata.created_at.format("%d-%m-%Y %H:%M:%S"));
    println!("Version: {}", db.metadata.version);
    
    // Créer un groupe pour les emails
    let email_group = Group::new("Email".to_string(), Some(db.root_group.id.clone()));
    let email_group_id = email_group.id.clone();
    
    // Ajouter le groupe des emails directement à la racine
    db.root_group.add_subgroup(email_group);
    
    // Créer une entrée Gmail
    let gmail_entry = Entry::new(
        "Gmail".to_string(),
        "utilisateur@gmail.com".to_string(),
        "mot_de_passe_secure_123".to_string(),
        "https://gmail.com".to_string(),
        "Mon compte email principal".to_string()
    );
    let gmail_id = gmail_entry.id.clone();
    
    // Ajouter l'entrée au groupe des emails
    if let Some(email_group) = db.find_group_mut(&email_group_id) {
        email_group.add_entry(gmail_entry);
        println!("\nEntrée Gmail ajoutée au groupe Email");
    } else {
        println!("\nGroupe Email non trouvé!");
    }
    
    // Créer un groupe pour les réseaux sociaux
    let social_group = Group::new("Réseaux sociaux".to_string(), Some(db.root_group.id.clone()));
    let social_group_id = social_group.id.clone();
    
    // Ajouter le groupe des réseaux sociaux à la racine
    db.root_group.add_subgroup(social_group);
    
    // Créer une entrée Facebook
    let facebook_entry = Entry::new(
        "Facebook".to_string(),
        "user@example.com".to_string(),
        "facebook_pwd_123!".to_string(),
        "https://facebook.com".to_string(),
        "Mon compte Facebook personnel".to_string()
    );
    
    // Ajouter l'entrée au groupe des réseaux sociaux
    if let Some(social_group) = db.find_group_mut(&social_group_id) {
        social_group.add_entry(facebook_entry);
        println!("Entrée Facebook ajoutée au groupe Réseaux sociaux");
    } else {
        println!("Groupe Réseaux sociaux non trouvé!");
    }
    
    // Afficher la structure de la base de données
    println!("\nStructure de la base de données:");
    println!("Groupe racine: {}", db.root_group.name);
    
    for (id, subgroup) in &db.root_group.subgroups {
        println!("  Groupe: {} (ID: {})", subgroup.name, id);
        
        for (entry_id, entry) in &subgroup.entries {
            println!("    Entrée: {} (ID: {})", entry.title, entry_id);
            println!("      Utilisateur: {}", entry.username);
        }
    }
    
    // Démonstration de la fonction de recherche
    println!("\nRecherche d'entrée par ID:");
    if let Some(entry) = db.find_entry(&gmail_id) {
        println!("Entrée trouvée: {}", entry.title);
        println!("{}", entry);
    } else {
        println!("Entrée non trouvée");
    }
    
    // Marquer comme modifié et vérifier
    db.mark_as_modified();
    println!("\nBase de données modifiée: {}", db.is_modified);
    println!("Dernière modification: {}", db.metadata.updated_at.format("%d-%m-%Y %H:%M:%S"));
}