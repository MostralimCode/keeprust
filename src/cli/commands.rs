use crate::models::{database::Database, entry::Entry, group::Group, repository::Repository};
use crate::utils::password_generator::PasswordGenerator;
use clap::Parser;
use rpassword::read_password;
use std::io::{self, Write};
use std::path::Path;
use crate::utils::clipboard::{copy_password_securely, copy_to_clipboard};
use crate::utils::password_analyzer::{PasswordAnalyzer, PasswordStrength};

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
        Commands::Show { id, show_password } => cmd_show(db_path, &id, show_password),
        Commands::List { group } => cmd_list(db_path, group.as_deref()),
        Commands::Add { title, username, password, url, notes, group, generate, length } => 
            cmd_add(db_path, title, username, password, url, notes, group, generate, length),
        Commands::Generate { length, no_uppercase, no_lowercase, no_digits, no_symbols, exclude_similar, exclude_ambiguous } => {
            cmd_generate(length, !no_uppercase, !no_lowercase, !no_digits, !no_symbols, exclude_similar, exclude_ambiguous)
        },
        Commands::AddGroup { name, parent } => cmd_add_group(db_path, name, parent),
        Commands::Copy { id, timeout } => cmd_copy_password(db_path, &id, timeout),
        Commands::CopyUser { id } => cmd_copy_username(db_path, &id),
        Commands::Analyze { password } => cmd_analyze_password(password),
        Commands::History { id } => cmd_show_history(db_path, &id),
        Commands::Audit => cmd_audit_passwords(db_path),
    }
}

/// Affiche l'aide
fn print_help() {
    println!("KeeRust - Gestionnaire de mots de passe sécurisé");
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

/// Affiche les informations d'une entrée spécifique
fn cmd_show(path: &Path, id: &str, show_password: bool) -> Result<(), String> {
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
    
    // Rechercher l'entrée
    let entry = match db.find_entry(id) {
        Some(entry) => entry,
        None => return Err(format!("Entrée avec ID '{}' non trouvée.", id)),
    };
    
    // Afficher les informations de l'entrée
    println!("\nEntrée: {}", entry.title);
    println!("ID: {}", entry.id);
    println!("Utilisateur: {}", entry.username);
    if show_password {
        println!("Mot de passe: {}", entry.password);
    } else {
        println!("Mot de passe: ********");
    }
    println!("URL: {}", entry.url);
    println!("Notes: {}", entry.notes);
    println!("Créé le: {}", entry.created_at.format("%d-%m-%Y %H:%M:%S"));
    println!("Modifié le: {}", entry.updated_at.format("%d-%m-%Y %H:%M:%S"));
    
    Ok(())
}

/// Liste toutes les entrées ou les entrées d'un groupe spécifique
fn cmd_list(path: &Path, group_id: Option<&str>) -> Result<(), String> {
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
    
    // Si un ID de groupe est spécifié, afficher uniquement les entrées de ce groupe
    if let Some(gid) = group_id {
        let group = match db.find_group(gid) {
            Some(g) => g,
            None => return Err(format!("Groupe avec ID '{}' non trouvé.", gid)),
        };
        
        println!("\nEntrées du groupe '{}':", group.name);
        if group.entries.is_empty() {
            println!("  (Aucune entrée)");
        } else {
            for (_, entry) in &group.entries {
                println!("  - {} (ID: {})", entry.title, entry.id);
                println!("    Utilisateur: {}", entry.username);
            }
        }
        
        return Ok(());
    }
    
    // Sinon, afficher toutes les entrées organisées par groupe
    println!("\nToutes les entrées:");
    
    // Entrées du groupe racine
    if !db.root_group.entries.is_empty() {
        println!("\nGroupe: {}", db.root_group.name);
        for (_, entry) in &db.root_group.entries {
            println!("  - {} (ID: {})", entry.title, entry.id);
            println!("    Utilisateur: {}", entry.username);
        }
    }
    
    // Entrées des sous-groupes
    for (_, group) in &db.root_group.subgroups {
        if !group.entries.is_empty() {
            println!("\nGroupe: {}", group.name);
            for (_, entry) in &group.entries {
                println!("  - {} (ID: {})", entry.title, entry.id);
                println!("    Utilisateur: {}", entry.username);
            }
        }
    }
    
    Ok(())
}

/// Ajoute une nouvelle entrée
fn cmd_add(path: &Path, title: String, username: String, password: Option<String>, 
           url: Option<String>, notes: Option<String>, group_id: Option<String>,
           generate: bool, length: usize) -> Result<(), String> {
    // Vérifier si le fichier existe
    if !path.exists() {
        return Err(format!("Le fichier {} n'existe pas.", path.display()));
    }
    
    // Demander le mot de passe de la base de données
    print!("Mot de passe de la base de données: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let db_password = read_password().map_err(|e| e.to_string())?;
    
    // Ouvrir la base de données
    let repo = Repository::new(path);
    let mut db = match repo.load(&db_password) {
        Ok(db) => db,
        Err(e) => return Err(format!("Erreur lors de l'ouverture de la base de données: {}", e)),
    };
    
    // Déterminer le mot de passe de l'entrée
    let entry_password = if generate {
        // Générer un mot de passe
        let generator = PasswordGenerator::new().length(length);
        let pwd = generator.generate_complex()
            .map_err(|e| e.to_string())?;
        println!("Mot de passe généré: {}", pwd);
        pwd
    } else if let Some(pwd) = password {
        // Utiliser le mot de passe fourni
        pwd
    } else {
        // Demander un mot de passe
        print!("Mot de passe pour l'entrée: ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        read_password().map_err(|e| e.to_string())?
    };
    
    // Créer l'entrée
    let entry = Entry::new(
        title,
        username,
        entry_password,
        url.unwrap_or_default(),
        notes.unwrap_or_default()
    );
    
    // Déterminer le groupe cible
    let target_group = if let Some(gid) = group_id {
        match db.find_group_mut(&gid) {
            Some(g) => g,
            None => return Err(format!("Groupe avec ID '{}' non trouvé.", gid)),
        }
    } else {
        &mut db.root_group
    };
    
    // Ajouter l'entrée au groupe
    let entry_id = entry.id.clone();
    target_group.add_entry(entry);
    
    // Marquer la base de données comme modifiée
    db.mark_as_modified();
    
    // Sauvegarder la base de données
    repo.save(&db, &db_password).map_err(|e| e.to_string())?;
    
    println!("Entrée ajoutée avec succès (ID: {}).", entry_id);
    Ok(())
}

/// Ajoute un nouveau groupe
fn cmd_add_group(path: &Path, name: String, parent_id: Option<String>) -> Result<(), String> {
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
    let mut db = match repo.load(&password) {
        Ok(db) => db,
        Err(e) => return Err(format!("Erreur lors de l'ouverture de la base de données: {}", e)),
    };
    
    // Déterminer le groupe parent
    let parent_id = match parent_id {
        Some(id) => {
            // Vérifier que le groupe parent existe
            if db.find_group(&id).is_none() {
                return Err(format!("Groupe parent avec ID '{}' non trouvé.", id));
            }
            Some(id)
        },
        None => Some(db.root_group.id.clone()),
    };
    
    // Créer le nouveau groupe
    let new_group = Group::new(name.clone(), parent_id.clone());
    let group_id = new_group.id.clone();
    
    // Ajouter le groupe au parent
    if let Some(pid) = parent_id {
        if let Some(parent) = db.find_group_mut(&pid) {
            parent.add_subgroup(new_group);
        } else {
            return Err("Erreur interne: groupe parent non trouvé.".to_string());
        }
    } else {
        // Cas improbable, mais gérons-le quand même
        db.root_group.add_subgroup(new_group);
    }
    
    // Marquer la base de données comme modifiée
    db.mark_as_modified();
    
    // Sauvegarder la base de données
    repo.save(&db, &password).map_err(|e| e.to_string())?;
    
    println!("Groupe '{}' ajouté avec succès (ID: {}).", name, group_id);
    Ok(())
}

/// Copie le mot de passe d'une entrée dans le presse-papiers
fn cmd_copy_password(path: &Path, id: &str, timeout: u64) -> Result<(), String> {
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
    
    // Rechercher l'entrée
    let entry = match db.find_entry(id) {
        Some(entry) => entry,
        None => return Err(format!("Entrée avec ID '{}' non trouvée.", id)),
    };
    
    // Copier le mot de passe dans le presse-papiers
    copy_password_securely(&entry.password, timeout)
        .map_err(|e| e.to_string())?;
    
    println!("Mot de passe de '{}' copié dans le presse-papiers.", entry.title);
    Ok(())
}

/// Copie le nom d'utilisateur d'une entrée dans le presse-papiers
fn cmd_copy_username(path: &Path, id: &str) -> Result<(), String> {
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
    
    // Rechercher l'entrée
    let entry = match db.find_entry(id) {
        Some(entry) => entry,
        None => return Err(format!("Entrée avec ID '{}' non trouvée.", id)),
    };
    
    // Copier le nom d'utilisateur dans le presse-papiers
    copy_to_clipboard(&entry.username)
        .map_err(|e| e.to_string())?;
    
    println!("Nom d'utilisateur de '{}' copié dans le presse-papiers.", entry.title);
    Ok(())
}

/// Analyse la force d'un mot de passe
fn cmd_analyze_password(password: Option<String>) -> Result<(), String> {
    let pwd = match password {
        Some(p) => p,
        None => {
            print!("Mot de passe à analyser: ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            read_password().map_err(|e| e.to_string())?
        }
    };
    
    let analyzer = PasswordAnalyzer::new();
    let analysis = analyzer.analyze(&pwd);
    
    println!("\n🔍 Analyse du mot de passe:");
    println!("{}", analysis);
    
    Ok(())
}

/// Affiche l'historique des mots de passe d'une entrée
fn cmd_show_history(path: &Path, id: &str) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Le fichier {} n'existe pas.", path.display()));
    }
    
    print!("Mot de passe: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let password = read_password().map_err(|e| e.to_string())?;
    
    let repo = Repository::new(path);
    let db = match repo.load(&password) {
        Ok(db) => db,
        Err(e) => return Err(format!("Erreur lors de l'ouverture de la base de données: {}", e)),
    };
    
    let entry = match db.find_entry(id) {
        Some(entry) => entry,
        None => return Err(format!("Entrée avec ID '{}' non trouvée.", id)),
    };
    
    println!("\n📜 Historique des mots de passe pour '{}':", entry.title);
    
    if entry.password_history.is_empty() {
        println!("  (Aucun historique disponible)");
    } else {
        for (i, history) in entry.password_history.iter().enumerate() {
            println!("  {}. Modifié le: {}", 
                i + 1, 
                history.changed_at.format("%d-%m-%Y %H:%M:%S")
            );
        }
    }
    
    Ok(())
}

/// Audit de sécurité de toutes les entrées
fn cmd_audit_passwords(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Le fichier {} n'existe pas.", path.display()));
    }
    
    print!("Mot de passe: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let password = read_password().map_err(|e| e.to_string())?;
    
    let repo = Repository::new(path);
    let db = match repo.load(&password) {
        Ok(db) => db,
        Err(e) => return Err(format!("Erreur lors de l'ouverture de la base de données: {}", e)),
    };
    
    let analyzer = PasswordAnalyzer::new();
    let mut weak_passwords = Vec::new();
    let mut total_entries = 0;
    
    // Fonction pour auditer un groupe
    fn audit_group(group: &Group, analyzer: &PasswordAnalyzer, weak_passwords: &mut Vec<(String, String, PasswordStrength)>, total_entries: &mut usize) {
        for (_, entry) in &group.entries {
            *total_entries += 1;
            let analysis = analyzer.analyze(&entry.password);
            
            if matches!(analysis.strength, PasswordStrength::VeryWeak | PasswordStrength::Weak) {
                weak_passwords.push((entry.id.clone(), entry.title.clone(), analysis.strength));
            }
        }
        
        // Auditer récursivement les sous-groupes
        for (_, subgroup) in &group.subgroups {
            audit_group(subgroup, analyzer, weak_passwords, total_entries);
        }
    }
    
    audit_group(&db.root_group, &analyzer, &mut weak_passwords, &mut total_entries);
    
    println!("\n🔐 Audit de sécurité terminé:");
    println!("Total d'entrées analysées: {}", total_entries);
    
    if weak_passwords.is_empty() {
        println!("✅ Aucun mot de passe faible détecté!");
    } else {
        println!("⚠️  {} mot(s) de passe faible(s) détecté(s):", weak_passwords.len());
        
        for (id, title, strength) in weak_passwords {
            println!("  - {} (ID: {}) - Force: {}", title, id, strength);
        }
        
        println!("\n💡 Recommandation: Utilisez la commande 'generate' pour créer des mots de passe plus forts.");
    }
    
    Ok(())
}