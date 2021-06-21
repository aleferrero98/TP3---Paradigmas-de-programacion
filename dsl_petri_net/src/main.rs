//use std::io;
use std::io::{self, Write};

#[derive(Debug)]
struct Plaza {
    nombre : String,
    num_tokens : i32
}

#[derive(Debug)]
struct Transicion {
    nombre : String,
    is_sensibilizada : bool
}

//define place piensa1,piensa2,,,piensan;    //esta es la instruccion de definicion de plazas
/// Crea todas las plazas de la RdP a partir de los nombres de cada una.
#[macro_export]
macro_rules! place {
    ($($name: expr),*) => {
        {
            let mut plazas : Vec<Plaza> = Vec::new();
            $( 
                let p = Plaza {nombre : $name.to_string(), num_tokens : 0};
                plazas.push(p);
            )+
            plazas
        }
    };
}

//define trans empieza1,empieza2,,,,empiezan, comiendo1,,,comiendon;   // esta es la instruccion de definicion de transiciones
/// Crea todas las transiciones de la RdP a partir de los nombres de cada una.
#[macro_export]
macro_rules! transition {
    ($($name: expr),*) => {
        {
            let mut transiciones : Vec<Transicion> = Vec::new();
            $( 
                let t = Transicion {nombre : $name.to_string(), is_sensibilizada : false};
                transiciones.push(t);
            )+
            transiciones
        }
    };
}

//define arc piensa1 to empieza1, comiendo1 to piensa1;  // esta es la instruccion de definicion de arcos


//define init piensa1{1},,,piensan{0}   // definicion del marcado inicial donde {n} indican la cantidad de tokens en la plaza respectiva
#[macro_export]
macro_rules! init {
    //() => { //marcado por defecto};

    ($($plaza: ident{$num: expr}),*) => {
       {
           $(
                println!("{:?}", $plaza);
                println!("{:?}", $num);
           )+
       }
    };

}

fn main() {
    //creacion e inicializacion de la RdP
    let a = place!("pensando", "comiendo");
    println!("{:?}", a);
    let b = transition!("t1", "t2");
    println!("{:?}", b);

    let p1 = &a[0];
    let p2 = &a[1];
    init!(p1{1}, p2{3});

    //Interaccion con el usuario - recepcion de comandos
    println!("\x1b[1;36m >> Bienvenidos a Petri Net Simulator << \x1b[0;37m");
    print!("Ingrese un comando: ");
    io::stdout().flush().ok();

    

    /*
    let mut input_comando = String::new();
    io::stdin().read_line(&mut input_comando).ok().expect("Error al leer de teclado");
    println!("El comando es {}", input_comando.trim()); //trim quita el \n final 
    */
}
