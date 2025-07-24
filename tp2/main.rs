use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use chrono::Local;

struct FileManager {
    path: String,
}

impl FileManager {
    fn new(path: &str) -> Self {
        FileManager {
            path: path.to_string(),
        }
    }

    fn create_file(&self) {
    if Path::new(&self.path).exists() {
        println!("Le fichier '{}' existe déjà.", self.path);
    } else {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path)
            .expect("Erreur lors de la création du fichier");
        println!("Fichier '{}' créé avec succès.", self.path);
    }
}

    fn write_file(&self, content: &str) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .expect("Erreur d'ouverture du fichier");

        let now = Local::now();
        writeln!(file, "{} - {}", now.format("%Y-%m-%d %H:%M:%S"), content)
            .expect("Erreur d'écriture dans le fichier");
    }

    fn read_file(&self) {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .expect("Erreur de lecture du fichier");

        let mut content = String::new();
        file.read_to_string(&mut content).expect("Erreur de lecture");
        println!("Contenu:\n{}", content);
    }

    fn modify_file(&self, new_content: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .expect("Erreur d'ouverture pour modification");

        writeln!(file, "{} - {}", Local::now().format("%Y-%m-%d %H:%M:%S"), new_content)
            .expect("Erreur de modification");
    }

    fn delete_file(&self) {
        if Path::new(&self.path).exists() {
            fs::remove_file(&self.path).expect("Erreur de suppression");
            println!("Fichier supprimé !");
        } else {
            println!("Fichier inexistant.");
        }
    }
}

fn menu() {
    println!("\n== MENU ==");
    println!("1. Créer un fichier vide");
    println!("2. Lire un fichier");
    println!("3. Écrire dans un fichier");
    println!("4. Modifier un fichier");
    println!("5. Supprimer un fichier");
    println!("6. Quitter");
}
fn get_filename() -> String {
    println!("Entrez le nom du fichier (sans extension) :");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Erreur de lecture");
    filename.trim().to_string()
}
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

fn main() {
    let file_manager = FileManager::new("data.txt");

    loop {
        menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        let choice = input.trim();

        match choice {
            "1" => {
                let filename = get_filename();
                let fm = FileManager::new(&filename);
                fm.create_file();
            },
            "2" => {
                let filename = get_filename();
                let fm = FileManager::new(&filename);
                fm.read_file();
            },
            "3" => {
                let filename = get_filename();
                let content = get_input("Entrez le texte à écrire :");
                let fm = FileManager::new(&filename);
                fm.write_file(&content);
            },
            "4" => {
                let filename = get_filename();
                let content = get_input("Entrez le nouveau contenu :");
                let fm = FileManager::new(&filename);
                fm.modify_file(&content);
            },
            "5" => {
                let filename = get_filename();
                let fm = FileManager::new(&filename);
                fm.delete_file();
            },
            "6" => {
                println!("Fermeture du programme. À bientôt !");
                break;
            },
            _ => println!("Choix invalide."),
        }

        // Utilisation de `while` pour pause
        let mut again = String::new();
        println!("Appuyez sur Entrée pour continuer...");
        while io::stdin().read_line(&mut again).is_err() {}
    }
}
