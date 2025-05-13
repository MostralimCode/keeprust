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
use utils::password_generator::PasswordGenerator;

fn main() {
	println!("KeepRust - Gestionnaire de mots de passe sécurisé");

	// Test du générateur de mots de passe
    println!("\nTest du générateur de mots de passe:");
    
    // Générer un mot de passe avec les options par défaut
    let generator = PasswordGenerator::default();
    match generator.generate() {
        Ok(password) => println!("Mot de passe par défaut: {}", password),
        Err(e) => println!("Erreur: {}", e),
    }
    
    // Générer un mot de passe complexe
    match generator.generate_complex() {
        Ok(password) => println!("Mot de passe complexe: {}", password),
        Err(e) => println!("Erreur: {}", e),
    }
    
    // Générer un mot de passe avec différentes options
    let custom_generator = PasswordGenerator::new()
        .length(20)
        .exclude_similar(true)
        .exclude_ambiguous(true);
    
    match custom_generator.generate() {
        Ok(password) => println!("Mot de passe personnalisé (20 caractères): {}", password),
        Err(e) => println!("Erreur: {}", e),
    }
    
    // Générer un mot de passe pour l'utilisation dans une URL (sans caractères spéciaux)
    let url_safe_generator = PasswordGenerator::new()
        .length(12)
        .symbols(false);
    
    match url_safe_generator.generate() {
        Ok(password) => println!("Mot de passe sûr pour URL: {}", password),
        Err(e) => println!("Erreur: {}", e),
    }
    
    // Générer un mot de passe facile à retenir (lettres uniquement)
    let memorable_generator = PasswordGenerator::new()
        .length(10)
        .digits(false)
        .symbols(false);
    
    match memorable_generator.generate() {
        Ok(password) => println!("Mot de passe mémorable: {}", password),
        Err(e) => println!("Erreur: {}", e),
    }
    
    // Générer un PIN (chiffres uniquement)
    let pin_generator = PasswordGenerator::new()
        .length(6)
        .uppercase(false)
        .lowercase(false)
        .symbols(false);
    
    match pin_generator.generate() {
        Ok(password) => println!("PIN: {}", password),
        Err(e) => println!("Erreur: {}", e),
    }
    
    println!("\nTest du générateur de mots de passe terminé!");
}