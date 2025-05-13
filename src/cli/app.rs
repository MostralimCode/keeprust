use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "keeprust")]
#[command(author = "Mostralim <mostralimcode@pm.me>")]
#[command(version = "0.1.0")]
#[command(about = "Un gestionnaire de mots de passe sécurisé similaire à KeePass", long_about = None)]
pub struct Cli {
    /// Chemin vers le fichier de base de données (par défaut: keeprust.krs)
    #[arg(short, long, default_value = "keeprust.krs")]
    pub database: String,
    
    /// Commande à exécuter
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Crée une nouvelle base de données
    New {
        /// Nom de la base de données
        #[arg(short, long)]
        name: Option<String>,
        
        /// Description de la base de données
        #[arg(short, long)]
        description: Option<String>,
    },
    
    /// Ouvre une base de données existante
    Open,
    
    /// Affiche une entrée spécifique
    Show {
        /// ID de l'entrée à afficher
        id: String,
        
        /// Affiche également le mot de passe
        #[arg(short, long)]
        show_password: bool,
    },
    
    /// Liste toutes les entrées ou toutes les entrées d'un groupe
    List {
        /// ID du groupe (liste toutes les entrées si non spécifié)
        #[arg(short, long)]
        group: Option<String>,
    },
    
    /// Ajoute une nouvelle entrée
    Add {
        /// Titre de l'entrée
        #[arg(short, long)]
        title: String,
        
        /// Nom d'utilisateur ou email
        #[arg(short, long)]
        username: String,
        
        /// Mot de passe (sera demandé si non spécifié)
        #[arg(short, long)]
        password: Option<String>,
        
        /// URL associée
        #[arg(short = 'r', long)]
        url: Option<String>,
        
        /// Notes supplémentaires
        #[arg(short, long)]
        notes: Option<String>,
        
        /// ID du groupe parent (utilise le groupe racine si non spécifié)
        #[arg(short, long)]
        group: Option<String>,
        
        /// Génère un mot de passe aléatoire
        #[arg(short = 'G', long)]  // Changer de -g à -G
        generate: bool,
        
        /// Longueur du mot de passe généré
        #[arg(short, long, default_value = "16")]
        length: usize,
    },
    
    /// Génère un mot de passe aléatoire (sans le sauvegarder)
    /// Génère un mot de passe aléatoire (sans le sauvegarder)
    Generate {
        /// Longueur du mot de passe
        #[arg(short, long, default_value = "16")]
        length: usize,
        
        /// Exclut les lettres majuscules
        #[arg(long)]
        no_uppercase: bool,
        
        /// Exclut les lettres minuscules
        #[arg(long)]
        no_lowercase: bool,
        
        /// Exclut les chiffres
        #[arg(long)]
        no_digits: bool,
        
        /// Exclut les symboles
        #[arg(long)]
        no_symbols: bool,
        
        /// Exclut les caractères similaires (I, l, 1, O, 0, etc.)
        #[arg(short = 's', long)]  // Changé de 'e' à 's'
        exclude_similar: bool,
        
        /// Exclut les caractères ambigus ({}[]()/\'"`~,;:.<>)
        #[arg(short = 'a', long)]  // Changé de 'e' à 'a'
        exclude_ambiguous: bool,
    },

     /// Crée un nouveau groupe
     AddGroup {
        /// Nom du groupe
        #[arg(short, long)]
        name: String,
        
        /// ID du groupe parent (utilise le groupe racine si non spécifié)
        #[arg(short, long)]
        parent: Option<String>,
    },
}