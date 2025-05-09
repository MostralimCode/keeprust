use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;

/// Représente une entrée dans le gestionnaire de mots de passe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
	/// Identifiant unique pour cette entrée
	pub id: String,

	/// Titre descriptif de l'entrée (ex: "Gmail", "Facebook", etc.)
	pub title: String,

	/// Nom d'utilisateur ou email associé
	pub username: String,

	// Mot de passe
	pub password: String,
	
	/// URL de site ou du service
	pub url: String,

	/// Notes supplémentaires
	pub notes: String,

	/// Date et heure de création de l'entrée
	pub created_at: DateTime<Utc>,

	/// Date et heure de dernière modification
	pub updated_at: DateTime<Utc>,
}

impl Entry {
	/// Crée une nouvelle entrée avec les informations fournies
	pub fn new(title: String, username: String, password: String, url: String, notes: String) -> Self {
		let now = Utc::now();
		Self {
			id: Uuid::new_v4().to_string(),
			title,
			username,
			password,
			url,
			notes,
			created_at: now,
			updated_at: now,
		}
	}

	/// Met à jour l'entrée et actualise l'horodatage de modification
	pub fn update(&mut self, title: Option<String>, username: Option<String>,
			password: Option<String>, url: Option<String>, notes: Option<String>) {
		if let Some(title) = title {
			self.title = title;
		}

		if let Some(username) = username {
			self.username = username;
		}

		if let Some(password) = password {
			self.password = password;
		}

		if let Some(url) = url {
			self.url = url;
		}

		// Mise à jour de l'horodatage
		self.updated_at = Utc::now();
	}
}

impl fmt::Display for Entry {
	/// Formate l'entrée pour l'affichage (sans révéler le mot de passe)
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Titre: {}\nUtilisateur: {}\nURL: {}\nCréé le: {}\nModifié le: {}",
			self.title,
			self.username,
			self.url,
			self.created_at.format("%d-%m-%Y %H:%M:%S"),
			self.updated_at.format("%d-%m-%Y %H:%M:%S"),
		)
	}
}







