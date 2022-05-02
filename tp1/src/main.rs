use chrono::prelude::*;

fn mad(a: i32, b: i32, c: i32) -> i32 {
    a * b + c
}

fn sum_from_to_while(a: i32, b: i32) -> i32 {
    let mut sum = 0;
    let mut i = a;
    while i <= b {
        sum += i;
        i += 1;
    }
    sum
}

fn sum_from_to_for(a: i32, b: i32) -> i32 {
    let mut sum = 0;
    for i in a..(b + 1) {
        sum += i;
    }
    sum
}

fn sum_from_to_recursion(a: i32, b: i32) -> i32 {
    if a == b {
        a
    } else {
        a + sum_from_to_recursion(a + 1, b)
    }
}

#[derive(Debug, Clone)]
struct Livre {
    titre: String,
    annee_publication: i32,
    genre: Genre,
}

fn age_livre(livre: Livre) -> i32 {
    let now = Local::now().year();
    now - livre.annee_publication
}

#[derive(Debug, Clone)]
enum Genre {
    Fiction(),
    Histoire(),
    Fantasy(),
    Informatique(),
}

fn note_livre(livre: Livre) -> f32 {
    let mult = match livre.genre {
        Genre::Fiction() => 12.0,
        Genre::Histoire() => 2.0,
        Genre::Fantasy() => 36.0,
        Genre::Informatique() => 41.0,
    };
    let somme = livre.titre.len() as i32 + livre.annee_publication;
    somme as f32 / mult
}

fn main() {
    println!("mad(1, 2, 3) : {}", mad(1, 2, 3));
    println!("sum_from_to_while(1, 5) : {}", sum_from_to_while(1, 5));
    println!("sum_from_to_for(1, 5) : {}", sum_from_to_for(1, 5));
    println!(
        "sum_from_to_recursion(1, 5) : {}",
        sum_from_to_recursion(1, 5)
    );

    let l1 = Livre {
        titre: String::from("Le Petit Prince"),
        annee_publication: 1943,
        genre: Genre::Fantasy(),
    };

    let l2 = Livre {
        titre: String::from("cool raoul"),
        annee_publication: 2020,
        genre: Genre::Fiction(),
    };

    let l3 = Livre {
        titre: String::from("raoul"),
        annee_publication: 2020,
        genre: Genre::Histoire(),
    };

    let l4 = Livre {
        titre: String::from("cool"),
        annee_publication: 2020,
        genre: Genre::Histoire(),
    };

    let l5 = Livre {
        titre: String::from("arbre moche"),
        annee_publication: 2020,
        genre: Genre::Informatique(),
    };

    println!(
        "Mon livre est : {:?} genre : {:?} publié en : {:?}",
        l1.titre, l1.genre, l1.annee_publication
    );

    println!(
        "Mon livre est : {:?} genre : {:?} publié en : {:?}",
        l2.titre, l2.genre, l2.annee_publication
    );

    println!(
        "Mon livre est : {:?} genre : {:?} publié en : {:?}",
        l3.titre, l3.genre, l3.annee_publication
    );

    println!(
        "Mon livre est : {:?} genre : {:?} publié en : {:?}",
        l4.titre, l4.genre, l4.annee_publication
    );

    println!(
        "Mon livre est : {:?} genre : {:?} publié en : {:?}",
        l5.titre, l5.genre, l5.annee_publication
    );

    println!(
        "Age du livre : {:?} : {:?} ans",
        l1.titre,
        age_livre(l1.clone())
    );

    println!(
        "Age du livre : {:?} : {:?} ans",
        l2.titre,
        age_livre(l2.clone())
    );

    println!(
        "Age du livre : {:?} : {:?} ans",
        l3.titre,
        age_livre(l3.clone())
    );

    println!(
        "Age du livre : {:?} : {:?} ans",
        l4.titre,
        age_livre(l4.clone())
    );

    println!(
        "Age du livre : {:?} : {:?} ans",
        l5.titre,
        age_livre(l5.clone())
    );

    println!(
        "Note du livre : {:?} : {:?}",
        l1.titre,
        note_livre(l1.clone())
    );

    println!(
        "Note du livre : {:?} : {:?}",
        l2.titre,
        note_livre(l2.clone())
    );

    println!(
        "Note du livre : {:?} : {:?}",
        l3.titre,
        note_livre(l3.clone())
    );

    println!(
        "Note du livre : {:?} : {:?}",
        l3.titre,
        note_livre(l3.clone())
    );

    println!(
        "Note du livre : {:?} : {:?}",
        l4.titre,
        note_livre(l4.clone())
    );

    println!(
        "Note du livre : {:?} : {:?}",
        l5.titre,
        note_livre(l5.clone())
    );
}
