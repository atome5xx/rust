TP1 – Application de gestion de comptes bancaires en Rust
Objectif

Ce projet consiste à développer une application en ligne de commande permettant de gérer plusieurs comptes bancaires. L’utilisateur peut :

Créer un compte bancaire avec un nom et un solde initial,

Afficher la liste des comptes,

Sélectionner un compte par son index pour consulter le solde ou effectuer un retrait,

Quitter le programme proprement.

Ce TP est une introduction pratique au langage Rust et couvre ses concepts fondamentaux.

Concepts et fonctionnalités utilisés

1. Variables mutables
Rust impose l'immuabilité par défaut. Pour modifier une variable, elle doit être déclarée avec le mot-clé mut.

rust
Copier
Modifier
let mut solde = 100.0;
solde -= 20.0;

2. Affichage formaté
Utilisation de println! pour afficher des messages formatés :

rust
Copier
Modifier
println!("Retrait de {:.2} € effectué.", montant);

3. Fonctions
Définition et appel de fonctions, avec ou sans paramètres :

rust
Copier
Modifier
fn afficher_solde(solde: f64) {
    println!("Solde actuel : {:.2} €", solde);
}

4. Boucles et contrôle de flux
Utilisation de loop pour gérer le menu principal, combiné à match pour gérer les choix de l'utilisateur.

5. Références et emprunts (&, &mut)
Utilisation des références pour éviter les copies inutiles et garantir la sécurité mémoire.

rust
Copier
Modifier
fn afficher(compte: &CompteBancaire) { ... }
fn retirer(compte: &mut CompteBancaire, montant: f64) { ... }

6. Structures (struct)
Définition de types personnalisés pour structurer les données proprement :

rust
Copier
Modifier
struct CompteBancaire {
    nom: String,
    solde: f64,
}

Bonnes pratiques de Rust mises en œuvre

1. Immuabilité par défaut
Les variables sont immuables sauf indication explicite. Cela renforce la sécurité du code.

rust
Copier
Modifier
let x = 5; // immuable
let mut y = 10;

2. Nommage
Les variables sont nommées en snake case
