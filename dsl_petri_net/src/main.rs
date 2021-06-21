//use std::io;
use std::io::{self, Write};

#[derive(Debug)]
struct Plaza {
    nombre : String,
    num_tokens : i32
}

/*
impl Plaza {
    fn set_num_tokens() {

    }
}*/

#[derive(Debug)]
struct Transicion {
    nombre : String,
    is_sensibilizada : bool
}

/*
/// Arco que van de una plaza P a transicion T
#[derive(Debug)]
struct ArcoPre {
    plaza : Plaza,
    transicion : Transicion
}

/// Arco que van de una transicion T a una plaza P
#[derive(Debug)]
struct ArcoPost {
    transicion : Transicion,
    plaza : Plaza
}*/

/// Arco que va de una plaza P a una transicion T o viceversa
#[derive(Debug)]
struct Arco {
    plaza : Plaza,
    transicion : Transicion
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
#[macro_export]
macro_rules! arc_pre {
    ($($p:ident to $t:ident),+) => {
        {
            let mut arcos_pre : Vec<Arco> = Vec::new();
            $(
                let arc = Arco {plaza : $p, transicion : $t};
                arcos_pre.push(arc);
            )+
            arcos_pre
        }
    };

}

//define arc piensa1 to empieza1, comiendo1 to piensa1;  // esta es la instruccion de definicion de arcos
#[macro_export]
macro_rules! arc_post {
    ($($t:ident to $p:ident),+) => {
        {
            let mut arcos_post : Vec<Arco> = Vec::new();
            $(
                let arc = Arco {plaza : $p, transicion : $t};
                arcos_post.push(arc);
            )+
            arcos_post
        }
    };
}

#[macro_export]
macro_rules! list_enabled {
    ($transiciones: ident) => {
        for transicion in &$transiciones{
            println!("Transicion {} {}",transicion.nombre, if(transicion.is_sensibilizada){"-> sensibilizada"}else{"-> no sensibilizada"})
        }
    }

}

#[macro_export]
macro_rules! update_enabled {
    ($transiciones: ident) => {
        for i in (0..$transiciones.len()){
            $transiciones[i].is_sensibilizada = true;
        }
    }
}

//define init piensa1{1},,,piensan{0}   // definicion del marcado inicial donde {n} indican la cantidad de tokens en la plaza respectiva
#[macro_export]
macro_rules! init {

    ($($plaza: ident{$num: expr}),+) => {
       {
           $(
               $plaza.num_tokens = $num;
           )+
       }
    };

}

fn main() {
    //creacion e inicializacion de la RdP
    let a = place!("pensando", "comiendo");
    println!("{:?}", a);
    let mut b = transition!("t1", "t2");
    println!("{:?}", b);
    //let arc1 = arc_pre!(p2 to t2);

    let mut p1 = Plaza {nombre : "juan".to_string(), num_tokens : 0};
    let mut p2 = Plaza {nombre : "pepe".to_string(), num_tokens : 0};
    init!(p1{1}, p2{3});
    init!(p1{4});
    println!("{:?}", p1);
    println!("{:?}", p2);
    let mut t2 = Transicion {nombre : "t2".to_string(), is_sensibilizada : false};
    list_enabled!(b);
    update_enabled!(b);
    list_enabled!(b);
    //let arc1 = arc!(p2 to t2);
     let mut arc1 = arc_pre!(p2 to t2);
     println!("ARCOS {:?}", arc1);


    //Interaccion con el usuario - recepcion de comandos
    let finalizo : bool = false;
   // let mut input_comando = String::new();
    println!("\x1b[1;36m >> Bienvenidos a Petri Net Simulator << \x1b[0;37m");
    /*while !finalizo {
           println!("Seleccione una acción a realizar");
           println!(" 1) Disparar transicion empiezan");
           println!(" 2) Disparar una transicion al azar");
           println!(" 3) Disparar todas las transiciones habilitadas");
           println!(" 4) Mostrar el marcado actual");
           println!(" 5) Mostrar transiciones habilitadas");
           println!(" 6) Finalizar");
           print!("{} >> ", '\u{1F980}');
           io::stdout().flush().ok();

           let mut input_comando = String::new();
           io::stdin().read_line(&mut input_comando).ok().expect("Error al leer de teclado");
          // println!("El comando es {}", input_comando.trim()); //trim quita el \n final 
           match input_comando.trim() {
               "1" => println!("uno"),
               "2" => println!("dos"),
               "3" => println!("tres"),
               "4" => println!("cuatro"),
               "5" => println!("cinco"),
               "6" => break,
               _ => println!("\x1b[1;31m Opcion incorrecta \x1b[0;37m")
           }
    }*/


}
