use crate::models::{database::Database, entry::Entry, group::Group, repository::Repository};
use crate::utils::password_generator::PasswordGenerator;
use clap::Parser;
use rpassword::read_password;
use std::io::{self, Write};
use std::path::Path;

use super::app::{Cli, Commands};

/// Exécute la commande CLI spécifiée
pub fn run() -> Result<(), String> {
    let cli = Cli::parse();
    
    // Chemin de la base de données
    let db_path = Path::new(&cli.database);
    
    // Si aucune commande n'est spécifiée, ouvrir la base de données (si elle existe)
    // ou afficher l'aide
    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            if db_path.exists() {
                Commands::Open
            } else {
                print_help();
                return Ok(());
            }
        }
    };
    
    // Exécuter la commande appropriée
    match command {
        Commands::New { name, description } => cmd_new(db_path, name, description),
        Commands::Open => cmd_open(db_path),
        Commands::Generate { length, no_uppercase, no_lowercase, no_digits, no_symbols, exclude_similar, exclude_ambiguous } => {
            cmd_generate(length, !no_uppercase, !no_lowercase, !no_digits, !no_symbols, exclude_similar, exclude_ambiguous)
        },
        _ => {
            println!("Cette commande n'est pas encore implémentée.");
            Ok(())
        }
    }
}

/// Affiche l'aide
fn print_help() {
    println!("KeepRust - Gestionnaire de mots de passe sécurisé");
    println!("Utilisez --help pour afficher l'aide complète.");
}

/// Crée une nouvelle base de données
fn cmd_new(path: &Path, name: Option<String>, description: Option<String>) -> Result<(), String> {
    // Vérifier si le fichier existe déjà
    if path.exists() {
        println!("Le fichier {} existe déjà.", path.display());
        print!("Voulez-vous l'écraser? (o/N): ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        
        if !input.trim().eq_ignore_ascii_case("o") && !input.trim().eq_ignore_ascii_case("oui") {
            println!("Opération annulée.");
            return Ok(());
        }
    }
    
    // Demander un nom si non spécifié
    let db_name = match name {
        Some(n) => n,
        None => {
            print!("Nom de la base de données: ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
            input.trim().to_string()
        }
    };
    
    // Demander une description si non spécifiée
    let db_description = match description {
        Some(d) => d,
        None => {
            print!("Description (optionnelle): ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
            input.trim().to_string()
        }
    };
    
    // Demander un mot de passe maître
    print!("Mot de passe maître: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let password = read_password().map_err(|e| e.to_string())?;
    
    // Demander une confirmation du mot de passe
    print!("Confirmez le mot de passe: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let confirm_password = read_password().map_err(|e| e.to_string())?;
    
    // Vérifier que les mots de passe correspondent
    if password != confirm_password {
        return Err("Les mots de passe ne correspondent pas.".to_string());
    }
    
    // Créer la base de données
    let db = Database::new(db_name, db_description);
    
    // Sauvegarder la base de données
    let repo = Repository::new(path);
    repo.save(&db, &password).map_err(|e| e.to_string())?;
    
    println!("Base de données créée avec succès: {}", path.display());
    Ok(())
}

/// Ouvre une base de données existante
fn cmd_open(path: &Path) -> Result<(), String> {
    // Vérifier si le fichier existe
    if !path.exists() {
        return Err(format!("Le fichier {} n'existe pas.", path.display()));
    }
    
    // Demander le mot de passe
    print!("Mot de passe: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let password = read_password().map_err(|e| e.to_string())?;
    
    // Ouvrir la base de données
    let repo = Repository::new(path);
    let db = match repo.load(&password) {
        Ok(db) => db,
        Err(e) => return Err(format!("Erreur lors de l'ouverture de la base de données: {}", e)),
    };
    
    // Afficher les informations de la base de données
    println!("\nBase de données: {}", db.metadata.name);
    println!("Description: {}", db.metadata.description);
    println!("Version: {}", db.metadata.version);
    println!("Créée le: {}", db.metadata.created_at.format("%d-%m-%Y %H:%M:%S"));
    println!("Modifiée le: {}", db.metadata.updated_at.format("%d-%m-%Y %H:%M:%S"));
    
    // Afficher la structure
    println!("\nStructure:");
    println!("- {}", db.root_group.name);
    
    for (_, subgroup) in &db.root_group.subgroups {
        println!("  - {} ({} entrées)", subgroup.name, subgroup.entries.len());
    }
    
    // Compter le nombre total d'entrées
    let mut total_entries = db.root_group.entries.len();
    for (_, subgroup) in &db.root_group.subgroups {
        total_entries += subgroup.entries.len();
    }
    
    println!("\nNombre total d'entrées: {}", total_entries);
    Ok(())
}

/// Génère un mot de passe aléatoire
fn cmd_generate(length: usize, use_uppercase: bool, use_lowercase: bool, use_digits: bool, 
                use_symbols: bool, exclude_similar: bool, exclude_ambiguous: bool) -> Result<(), String> {
    // Configurer le générateur de mots de passe
    let generator = PasswordGenerator::new()
        .length(length)
        .uppercase(use_uppercase)
        .lowercase(use_lowercase)
        .digits(use_digits)
        .symbols(use_symbols)
        .exclude_similar(exclude_similar)
        .exclude_ambiguous(exclude_ambiguous);
    
    // Générer le mot de passe
    let password = generator.generate_complex()
        .map_err(|e| e.to_string())?;
    
    // Afficher le mot de passe
    println!("{}", password);
    Ok(())
}