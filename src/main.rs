mod cli;
mod crypto;
mod models;
mod utils;

use models::entry::Entry;
use models::group::Group;
use models::database::Database;
use crypto::{cipher, key_derivation};
use serde_json;

fn main() {
	println!("KeepRust - Gestionnaire de mots de passe sécurisé");

	 // Test du module de cryptographie
     println!("\nTest du module de cryptographie:");
    
     // 1. Dériver une clé à partir d'un mot de passe
     let password = "mot_de_passe_maitre_123!";
     let iterations = 10000; // Nombre d'itérations pour PBKDF2
     
     let master_key = match key_derivation::derive_key_from_password(password, iterations) {
         Ok(key) => {
             println!("Clé dérivée avec succès");
             println!("   Sel: {}", key.salt);
             println!("   Itérations: {}", key.iterations);
             key
         },
         Err(e) => {
             println!("Erreur lors de la dérivation de la clé: {}", e);
             return;
         }
     };
     
     // 2. Créer une base de données et la sérialiser en JSON
     let mut db = Database::new(
         "Base de test".to_string(),
         "Base de données pour tester le chiffrement".to_string()
     );
     
     // Ajouter un groupe et une entrée
     let email_group = Group::new("Email".to_string(), Some(db.root_group.id.clone()));
     let email_group_id = email_group.id.clone();
     db.root_group.add_subgroup(email_group);
     
     if let Some(group) = db.find_group_mut(&email_group_id) {
         group.add_entry(Entry::new(
             "Gmail".to_string(),
             "test@gmail.com".to_string(),
             "password123".to_string(),
             "https://gmail.com".to_string(),
             "Compte de test".to_string()
         ));
     }
     
     // Sérialiser la base de données en JSON
     let db_json = match serde_json::to_string(&db) {
         Ok(json) => {
             println!("Base de données sérialisée avec succès");
             json
         },
         Err(e) => {
             println!("Erreur lors de la sérialisation: {}", e);
             return;
         }
     };
     
     // 3. Chiffrer les données
     let encrypted_data = match cipher::encrypt(db_json.as_bytes(), &master_key.key) {
         Ok(data) => {
             println!("Données chiffrées avec succès");
             println!("   Taille du texte chiffré: {} caractères", data.ciphertext.len());
             data
         },
         Err(e) => {
             println!("Erreur lors du chiffrement: {}", e);
             return;
         }
     };
     
     // 4. Déchiffrer les données
     let decrypted_data = match cipher::decrypt(&encrypted_data, &master_key.key) {
         Ok(data) => {
             println!("Données déchiffrées avec succès");
             data
         },
         Err(e) => {
             println!("Erreur lors du déchiffrement: {}", e);
             return;
         }
     };
     
     // 5. Désérialiser la base de données
     let decrypted_json = String::from_utf8_lossy(&decrypted_data);
     let decrypted_db: Database = match serde_json::from_str(&decrypted_json) {
         Ok(db) => {
             println!("Base de données désérialisée avec succès");
             db
         },
         Err(e) => {
             println!("Erreur lors de la désérialisation: {}", e);
             return;
         }
     };
     
     // 6. Vérifier que les données sont identiques
     println!("\nVérification des données après chiffrement/déchiffrement:");
     println!("Nom de la base de données: {}", decrypted_db.metadata.name);
     println!("Description: {}", decrypted_db.metadata.description);
     
     // Recherche du groupe Email
     if let Some(email_group) = decrypted_db.find_group(&email_group_id) {
         println!("Groupe trouvé: {}", email_group.name);
         
         // Afficher toutes les entrées
         for (_, entry) in &email_group.entries {
             println!("  Entrée: {}", entry.title);
             println!("    Utilisateur: {}", entry.username);
             println!("    Mot de passe: {}", entry.password);
         }
     } else {
         println!("Groupe non trouvé après déchiffrement!");
     }
     
     println!("\nTest de cryptographie terminé avec succès!");
 }