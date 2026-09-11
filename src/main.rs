use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    //dbg!(&args);

    let (dict, numero) = match args.as_slice() {
        [numero] => ("numbers.dict" ,numero.as_str()),
        [dict, numero] => (dict.as_str() ,numero.as_str()),
        _ => return eprintln!("Error")
    };

    let f = match fs::read_to_string(dict) {
        Ok(fichier) => fichier,
        Err(_) => return eprintln!("Dict Error"),
    };

    let lignes = f.lines().count();

    println!("Dictionnaire : {} ({} lignes)", dict, lignes);
    println!("Nombre : {}", numero);
}

#[cfg(test)]

mod tests {

    use super::*; 

}

/*
Avec 1 argument, c'est le nombre, et le dictionnaire utilisé est numbers.dict.
Avec 2 arguments, le premier est le chemin du dictionnaire et le second est le nombre.
Avec un autre nombre d'arguments, le programme affiche Error.
Si la lecture du fichier échoue, il affiche Dict Error.
Sinon, il affiche le chemin du dictionnaire, son nombre de lignes et le nombre reçu.
*/
