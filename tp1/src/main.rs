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
}

fn age_livre(livre: Livre) -> i32 {
    let now = Local::now().year();
    now - livre.annee_publication
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
    };

    let l2 = Livre {
        titre: String::from("cool raoul"),
        annee_publication: 2020,
    };

    println!(
        "Mon livre est : {:?} publié en : {:?}",
        l1.titre, l1.annee_publication
    );

    println!(
        "Mon livre est : {:?} publié en : {:?}",
        l2.titre, l2.annee_publication
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
}
