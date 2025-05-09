mod cli;
mod crypto;
mod models;
mod utils;

use models::entry::Entry;

fn main() {
	println!("KeepRust - Gestionnaire de mots de passe sécurisé");

	// Test de création d'une entrée
	let entry = Entry::new(
		"Gmail".to_string(),
		"utilisateur@gmail.com".to_string(),
		"mot_de_passe_secure_123".to_string(),
		"https://gmail.com".to_string(),
		"Mon compte email principal".to_string()
	);

	// Afficher l'entrée (sans le mot de passe)
	println!("\nExemple d'entrée créée:");
	println!("{}", entry);

	// Accès direct aux champs pour du debug
	println!("\nInformations complètes (debug):");
	println!("ID : {}", entry.id);
	println!("Titre: {}", entry.title);
	println!("Utilisateur: {}", entry.username);
	println!("Mot de passe: {}", entry.password);

}
