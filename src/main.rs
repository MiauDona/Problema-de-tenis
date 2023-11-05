fn main() {
    // Puntuaciones de los jugadores
    let mut puntuacion_1 = 0;
    let mut puntuacion_2 = 0;

    // Definir nombre de los jugadores
    let p1 = "Jugador 1";
    let p2 = "Jugador 2";

    // Definir la secuencia de jugadas y el maximo de puntos que se pueden obtener por un unico jugador
    let mut secuencia: Vec<String> = vec![];
    let max_value: u8 = 5;

    // Inicializar contador
    let mut i = 0;
    
    // Iniciar el juego. Mientras que ningun jugador supere el maximo de puntos, el juego continua
    while (puntuacion_1 <= max_value && puntuacion_2 <= max_value) || (puntuacion_1 == puntuacion_2) || (puntuacion_1 == puntuacion_2 + 1) || (puntuacion_1 + 1 == puntuacion_2) {

        // Preguntar por la jugada de la ronda
        println!("Quien ganó la ronda?");
        let mut ganador = String::new();
        std::io::stdin().read_line(&mut ganador).expect("Error al leer el nombre");

        // Guardar la jugada en el vector
        ganador = String::from(ganador.trim());
        if ganador == p1 || ganador == p2 {
            secuencia.push(ganador);
            println!("Secuencia: {:?}", secuencia);
        }
        else {
            println!("Jugada no válida");
            continue;
        }
        

        // Si gana el jugador 1, se le suma un punto
        if secuencia[i] == p1 {
            puntuacion_1 += 1;

        // Si gana el jugador 2, se le suma un punto
        } else if secuencia[i] == p2 {
            puntuacion_2 += 1;
        }

        // Suma 1 al contador
        i += 1;

        // Imprimir i
        println!("i: {}", i);

        // Imprimir el puntaje
        match puntuacion_1 {
            0 => println!("{}: Love", p1),
            1 => println!("{}: 15", p1),
            2 => println!("{}: 30", p1),
            3 => if puntuacion_1 == puntuacion_2 {
                println!("{}: Deuce", p1);
            } else { 
                println!("{}: 40", p1);
            },
            _ => if puntuacion_1 > puntuacion_2 {
                println!("{}: Ventaja", p1);
            } else if puntuacion_1 < puntuacion_2 {
                println!("{}: Desventaja", p2);
            } else {
                println!("{}: Deuce", p1);
            }
        }

        match puntuacion_2 {
            0 => println!("{}: Love", p2),
            1 => println!("{}: 15", p2),
            2 => println!("{}: 30", p2),
            3 =>if puntuacion_1 == puntuacion_2 {
                println!("{}: Deuce", p2);
            } else { 
                println!("{}: 40", p2);
            },

            _ => if puntuacion_1 > puntuacion_2 {
                println!("{}: Desventaja ", p1);
            } else if puntuacion_1 < puntuacion_2 {
                println!("{}: Ventaja ", p2);
            } else {
                println!("{}: Deuce", p2);
            }
        }


    }
    






























    // Imprimir el nombre del ganador
    // .trim() es para quitar los espacios en blanco al inicio y al final de la cadena de texto.

     
    // Un buf: &mut String es un puntero a un String mutable, es decir, un puntero a una cadena de texto que puede ser modificada. 
    // Esto quiere decir que el valor de la cadena de texto puede cambiar y el puntero seguirá apuntando a la misma dirección de memoria que 
    // contiene el valor de la cadena de texto de forma dinámica.
    // Un puntero es una dirección de memoria que apunta a un valor en memoria que puede ser modificado por el programa.
    
    
    // std::io::stdin().read_line(&mut nombre).expect("Error al leer el nombre"); significa que se va a leer una linea de texto desde el teclado y se va a guardar en la variable nombre. 
    //El .expect() es para manejar errores como por ejemplo que el usuario no escriba nada y presione enter.
    
    // Asi se comprueba que en un vector hay un elemento y cuente cuantos de ese elemento hay, tneiendo en cuenta que 
    // let mut contador = 0;
    // for i in 0..secuencia.len() {
    //     if secuencia[i] == "piedra" {
    //         contador += 1;
    //     }
    // }


}

