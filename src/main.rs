use std::io;

fn main() {
    // Just type de function you want to use for this problems
}

// Ingresaremos una cantidad de GB para almazenar en discos CD, estos discos tienen capacidad maxima de 
// 700mb cada uno
fn many_cd(){
    println!("Ingrese el tamaño de GB a almazenar:");
    let mut number_gb = String::new();
    io::stdin()
        .read_line(&mut number_gb)
        .expect("Error al leer");
    let mut number: i32 =  number_gb.trim()
                        .parse()
                        .expect("Fail to convert");
    
    number *= 1024;

    let mut result = 0;
    while number > 0 {
        number -= 700;
        result += 1;
    }
    println!("Cantidad de cd a usarse: {result}");
}

// funcion que nos ayuda a saber la cantidad de paga que se le dara al trabajador, para esto sabemos que si el 
// trabajador estuvo de 40 horas o menos se le pagara 120 pesos la hora, pero si este llega a trabajar mas de 40 horas,
// se le pagaran esas horas extras pero siendo 200 pesos cada hora extra.
fn hours_work(){
    println!("Ingrese el numero de horas trabajadas: ");
    let mut horas = String::new();
    io::stdin()
        .read_line(&mut horas)
        .expect("Error al leer");

    let horas_number: i32 = horas.trim()
                                .parse()
                                .expect("Error al convertir");
    if horas_number <= 40{
        let resultado_1 = horas_number * 120;
        println!("Usted recibe un total de: {resultado_1}");
    } else if horas_number > 40{
        let horas_extra = (horas_number - 40) * 200;
        let horas_normales = (40 * 120) + horas_extra;
        println!("Por sus horas extra recibe: {horas_normales}");
        
    }
}

// se le pide un numero al usuario, cada numero que se ingrese se ira almazenando y al momento que se ingrese el valor 0
// se mostrara cuantos numeros pares y impares ingreso en el proceso
fn odd_even(){
    let mut numero = 1;
    let mut odd = 0;
    let mut even = 0;
    while numero != 0{
        println!("Ingrese su numero:");
        let mut valor_inicial = String::new();
        io::stdin()
            .read_line(&mut valor_inicial)
            .expect("Error al leer");
        let numero: i32 = valor_inicial.trim()
                                    .parse()
                                    .expect("Error al convertir");
        if numero % 2 == 0 {
            even += 1;
        } else{
            odd += 1;
        }
        if numero == 0{
            break;
        }
    }
    println!("Numeros impares: {odd}");
    println!("Numeros pares: {even}");

}
