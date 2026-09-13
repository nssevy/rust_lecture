use std::env;
//use std::fs;

fn choose_dict_and_num(arguments: &[String]) -> Result<(&str, &str), String> {
    
    let (dict, numero) = match arguments {
        [numero] => ("numbers.dict" ,numero.as_str()),
        [dict, numero] => (dict.as_str() ,numero.as_str()),
        _ => return Err("Dict error".to_string())
    };

    /*let read = match fs::read_to_string(dict) {
        Ok(fichier) => fichier,
        Err(_) => return Err(format!("Dict Error")),
    };*/

    Ok((dict, numero))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match choose_dict_and_num(&args){
        Ok((dict, numero)) => println!("Dictionnaire : {} (--) lignes \nNombre : {}", dict, numero),
        Err(e)=> eprintln!("erreur : {}", e),
    }
    //dbg!(&args);

    /*let (dict, numero) = match args.as_slice() {
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
    println!("Nombre : {}", numero);*/
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn check_que_le_dictionnaire_est_bien_lu() {
        asse
    }

    #[test]
    fn check_que_1_argument_renvoi_bien_le_dictionnaire_et_nombre(){
        let tab: Vec<String> = vec!["42".into()];
        assert_eq!( Ok( ("numbers.dict", "42" ) ), choose_dict_and_num(&tab));
    }

}

/*
Avec 1 argument, c'est le nombre, et le dictionnaire utilisé est numbers.dict.
Avec 2 arguments, le premier est le chemin du dictionnaire et le second est le nombre.
Avec un autre nombre d'arguments, le programme affiche Error.
Si la lecture du fichier échoue, il affiche Dict Error.
Sinon, il affiche le chemin du dictionnaire, son nombre de lignes et le nombre reçu.
*/
