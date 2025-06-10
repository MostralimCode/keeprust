mod cli;
mod crypto;
mod models;
mod utils;

use crate::cli::commands;
use utils::password_generator::PasswordGenerator;

fn main() {
    // Si des arguments sont passés, exécuter l'interface CLI
    if std::env::args().len() > 1 {
        match commands::run() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Erreur: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }
    
    // Sinon, afficher un message d'aide
    println!("KeepRust - Gestionnaire de mots de passe sécurisé");
    println!("Utilisez --help pour voir les commandes disponibles.");
    
    // Montrer un exemple rapide de génération de mot de passe
    println!("\nExemple de génération de mot de passe:");
    match PasswordGenerator::default().generate_complex() {
        Ok(password) => println!("{}", password),
        Err(e) => println!("Erreur: {}", e),
    }
}