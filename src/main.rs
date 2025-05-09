mod cli;
mod crypto;
mod models;
mod utils;

use models::entry::Entry;
use models::group::Group;

fn main() {
	println!("KeepRust - Gestionnaire de mots de passe sécurisé");

	// Créer un groupe racine
    let mut root_group = Group::new("Root".to_string(), None);
    
    // Créer un sous-groupe pour les emails
    let email_group = Group::new("Email".to_string(), Some(root_group.id.clone()));
    let email_group_id = email_group.id.clone();
    
    // Ajouter le sous-groupe au groupe racine
    root_group.add_subgroup(email_group);
    
    // Créer une entrée
    let gmail_entry = Entry::new(
        "Gmail".to_string(),
        "utilisateur@gmail.com".to_string(),
        "mot_de_passe_secure_123".to_string(),
        "https://gmail.com".to_string(),
        "Mon compte email principal".to_string()
    );
    let gmail_id = gmail_entry.id.clone();
    
    // Ajouter l'entrée au sous-groupe des emails
    if let Some(email_group) = root_group.get_subgroup_mut(&email_group_id) {
        email_group.add_entry(gmail_entry);
    }
    
    // Démonstration de navigation dans la hiérarchie
    println!("\nStructure du gestionnaire de mots de passe:");
    println!("Groupe racine: {}", root_group.name);
    
    for (id, subgroup) in &root_group.subgroups {
        println!("  Sous-groupe: {} (ID: {})", subgroup.name, id);
        
        for (entry_id, entry) in &subgroup.entries {
            println!("    Entrée: {} (ID: {})", entry.title, entry_id);
            println!("      Utilisateur: {}", entry.username);
        }
    }
    
    // Démonstration d'accès direct
    if let Some(email_group) = root_group.get_subgroup(&email_group_id) {
        if let Some(entry) = email_group.get_entry(&gmail_id) {
            println!("\nAccès direct à l'entrée Gmail:");
            println!("{}", entry);
        }
    }
}
