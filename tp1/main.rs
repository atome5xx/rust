use std::collections::HashMap;
use std::io;

struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    fn afficher_solde(&self) {
        println!("Le solde de {} est de {:.2} €", self.nom, self.solde);
    }

    fn retirer(&mut self, montant: f64) {
        if montant < 0 {
            println!("Impossible de retirer une somme négative");
            return;
        }
        if montant <= self.solde {
            self.solde -= montant;
            println!("Retrait de {:.2} € effectué. Nouveau solde : {:.2} €", montant, self.solde);
        } else {
            println!("Fonds insuffisants !");
        }
    }
}

fn main() {
    let mut comptes: HashMap<String, CompteBancaire> = HashMap::new();
    loop {
        println!("\n=== MENU ===");
        println!("1. Créer un compte");
        println!("2. Afficher la liste des comptes");
        println!("3. Afficher le solde d’un compte");
        println!("4. Retirer de l’argent");
        println!("5. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        match choix.trim() {
            "1" => {
                println!("Nom du compte :");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim().to_string();

                println!("Solde initial :");
                let mut solde_str = String::new();
                io::stdin().read_line(&mut solde_str).unwrap();
                let solde: f64 = solde_str.trim().parse().unwrap_or(0.0);

                let compte = CompteBancaire { nom: nom.clone(), solde };
                comptes.insert(nom.clone(), compte);
                println!("Compte '{}' créé.", nom);
            }

            "2" => {
                println!("\nListe des comptes :");
                for (nom, compte) in &comptes {
                    println!("- {} (solde: {:.2} €)", nom, compte.solde);
                }
            }

            "3" => {
                println!("Nom du compte :");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim();
                if let Some(compte) = comptes.get(nom) {
                    compte.afficher_solde();
                } else {
                    println!("Compte non trouvé.");
                }
            }

            "4" => {
                println!("Nom du compte :");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim().to_string();

                if let Some(compte) = comptes.get_mut(&nom) {
                    println!("Montant à retirer :");
                    let mut montant_str = String::new();
                    io::stdin().read_line(&mut montant_str).unwrap();
                    let montant: f64 = montant_str.trim().parse().unwrap_or(0.0);
                    compte.retirer(montant);
                } else {
                    println!("Compte non trouvé.");
                }
            }

            "5" => {
                println!("Au revoir !");
                break;
            }

            _ => println!("Choix invalide."),
        }
    }
}
