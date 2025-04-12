use std::io;
use rand::Rng;

fn main() {

    let mut num: i64 = -1;
    let mut health:u8 = 10;
    let max:i64 = 1000;

    let mut rng = rand::thread_rng();
    let random_num: i64 = rng.gen_range(0..=max);
   // println!("random num: {} ", random_num);

   println!("\n********************************************************************");
   println!("Trouve le code secret du coffre, tu n'as que 10 tentatives possibles");
   println!("********************************************************************\n");
   
    while (num < 0 || num > 500000) || num != random_num{
        if (health == 0){
            println!("tu as perdu, le code secret était : {}", random_num);
            break;
        }
        let mut input = String::new();
        println!("Choisissez un nombre entre 0 et {}: ", max);
        io::stdin().read_line(&mut input);
      
        if let Ok(n) = input.trim().parse::<i64>() {

            if (n >= 0 && n <= 500000){
                num = n;
                if num == random_num{
                    println!("\n*********************");
                    println!("Bien joué, tu repars avec tout le magot du coffre.");
                    println!("*********************\n");
                }
                else{
                    if random_num > num {
                        println!("\nIncorrect: le code secret est + grand");
                    }
                    else{
                        println!("\nIncorrect: le code secret est + petit");
                    }
                    
                    health -= 1;
                    println!("il te reste {} vies \n", health);
                }
               // num = n;
            }
            else{
                println!("--------------------- \n");
                println!("Nombre hors valeurs autorisée ! \n");
            }
           
        } else {
            println!("--------------------- \n");
            println!("Ce n'est pas un nombre ! \n");
            continue;
        }
        
    }

}


/*
use std::io;

fn main() {
    let mut nom = String::new();
    let mut prenom = String::new();
    let mut age = String::new();

    println!("Entrez votre prénom :");
    io::stdin().read_line(&mut prenom).expect("Erreur de lecture");

    println!("Entrez votre nom :");
    io::stdin().read_line(&mut nom).expect("Erreur de lecture");

    println!("Entrez votre âge :");
    io::stdin().read_line(&mut age).expect("Erreur de lecture");

    // Nettoyage des sauts de ligne
    let prenom = prenom.trim();
    let nom = nom.trim();
    let age = age.trim();

    println!("Bonjour {} {}, {} ans", prenom, nom, age);
}
*/