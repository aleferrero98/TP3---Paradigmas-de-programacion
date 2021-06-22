use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Plaza {
    nombre : String,
    num_tokens : i32,
    orden : usize
}

/*
impl Plaza {
    fn set_num_tokens() {

    }
}*/

#[derive(Debug, Clone)]
struct Transicion {
    nombre : String,
    is_sensibilizada : bool,
    orden : usize
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
#[derive(Debug, Clone)]
struct Arco {
    plaza : Plaza,
    transicion : Transicion
}

//define place piensa1,piensa2,,,piensan;    //esta es la instruccion de definicion de plazas
/// Crea todas las plazas de la RdP a partir de los nombres de cada una.
#[macro_export]
macro_rules! placeee {
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


//define place piensa1,piensa2,,,piensan;    //esta es la instruccion de definicion de plazas
/// Crea todas las plazas de la RdP a partir de los nombres de cada una.
#[macro_export]
macro_rules! place {
    ($($plaza: ident),*) => {
        {
            let mut a = 0;
            $( 
                $plaza = Plaza {nombre : "".to_string(), num_tokens : 0, orden : a.clone()};
                a = a + 1;
            )+
        }
    };
}

//define trans empieza1,empieza2,,,,empiezan, comiendo1,,,comiendon;   // esta es la instruccion de definicion de transiciones
/// Crea todas las transiciones de la RdP a partir de los nombres de cada una.
#[macro_export]
macro_rules! transitionnn {
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

//define trans empieza1,empieza2,,,,empiezan, comiendo1,,,comiendon;   // esta es la instruccion de definicion de transiciones
/// Crea todas las transiciones de la RdP a partir de los nombres de cada una.
#[macro_export]
macro_rules! transition {
    ($($transicion: ident),*) => {
        {
            let mut a = 0;
            $( 
                $transicion = Transicion {nombre : "".to_string(), is_sensibilizada : false, orden : a.clone()};
                a = a + 1;
            )+
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
                let arc = Arco {plaza : $p.clone(), transicion : $t.clone()};
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
                let arc = Arco {plaza : $p.clone(), transicion : $t.clone()};
                arcos_post.push(arc);
            )+
            arcos_post
        }
    };
}

#[macro_export]
macro_rules! update_enabled {
    ($plazas: ident $transiciones: ident $arco : ident) => {
        for i in (0..$transiciones.len()){
            $transiciones[i].is_sensibilizada = true;
        }

        // for i in (0..$arco.len()){
        //     if($arco[i].plaza.num_tokens < 1){
        //         arco[i].transicion.is_sensibilizada = false;
        //     }
        // }
        for i in (0..$arco.len()){
            if($plazas[$arco[i].plaza.orden].num_tokens < 1){
                $transiciones[$arco[i].transicion.orden].is_sensibilizada = false;
            }
        }
    }
}

#[macro_export]
macro_rules! list_marc {
    ($plazas: ident) => {
        for plaza in &$plazas{
            println!("Plaza {} {}",plaza.nombre, plaza.num_tokens)
        }
    }

}

#[macro_export]
macro_rules! list_enabled {
    ($transiciones: ident) => {
        for transicion in &$transiciones{
            println!("Transicion {} {}",transicion.nombre, if(transicion.is_sensibilizada){"-> sensibilizada"}else{"-> no sensibilizada"})
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
    // PLAZAS
    let mut pensando1 : Plaza; 
    let mut pensando2 : Plaza; 
    let mut pensando3 : Plaza; 
    let mut pensando4 : Plaza; 
    let mut pensando5 : Plaza; 

    let mut comiendo1 : Plaza; 
    let mut comiendo2 : Plaza; 
    let mut comiendo3 : Plaza; 
    let mut comiendo4 : Plaza; 
    let mut comiendo5 : Plaza; 

    let mut tenedor1 : Plaza; 
    let mut tenedor2 : Plaza; 
    let mut tenedor3 : Plaza; 
    let mut tenedor4 : Plaza; 
    let mut tenedor5 : Plaza; 

    place!(pensando1, pensando2, pensando3, pensando4, pensando5, 
           comiendo1, comiendo2, comiendo3, comiendo4, comiendo5, 
           tenedor1, tenedor2, tenedor3, tenedor4, tenedor5); 

    // TRANSICIONES
    let mut empezar_comer1 : Transicion;
    let mut empezar_comer2 : Transicion;
    let mut empezar_comer3 : Transicion;
    let mut empezar_comer4 : Transicion;
    let mut empezar_comer5 : Transicion;
    
    let mut terminar_comer1 : Transicion;
    let mut terminar_comer2 : Transicion;
    let mut terminar_comer3 : Transicion;
    let mut terminar_comer4 : Transicion;
    let mut terminar_comer5 : Transicion;

    transition!(empezar_comer1, empezar_comer2, empezar_comer3, empezar_comer4, empezar_comer5,
                terminar_comer1, terminar_comer2, terminar_comer3, terminar_comer4, terminar_comer5);
                                 
    // ARCOS
    let mut arcos_pre = arc_pre!(tenedor1 to empezar_comer1, tenedor2 to empezar_comer1, pensando1 to empezar_comer1, comiendo1 to terminar_comer1,
                                tenedor2 to empezar_comer2, tenedor3 to empezar_comer2, pensando2 to empezar_comer2, comiendo2 to terminar_comer2,
                                tenedor3 to empezar_comer3, tenedor4 to empezar_comer3, pensando3 to empezar_comer3, comiendo3 to terminar_comer3,
                                tenedor4 to empezar_comer4, tenedor5 to empezar_comer4, pensando4 to empezar_comer4, comiendo4 to terminar_comer4,
                                tenedor5 to empezar_comer5, tenedor1 to empezar_comer5, pensando5 to empezar_comer5, comiendo5 to terminar_comer5);
    println!("{:?}", arcos_pre);

    let mut arcos_post = arc_post!(terminar_comer1 to tenedor1, terminar_comer1 to tenedor2, terminar_comer1 to pensando1, empezar_comer1 to comiendo1,
                                terminar_comer2 to tenedor2, terminar_comer2 to tenedor3, terminar_comer2 to pensando2, empezar_comer2 to comiendo2,
                                terminar_comer3 to tenedor3, terminar_comer3 to tenedor4, terminar_comer3 to pensando3, empezar_comer3 to comiendo3,
                                terminar_comer4 to tenedor4, terminar_comer4 to tenedor5, terminar_comer4 to pensando4, empezar_comer4 to comiendo4,
                                terminar_comer5 to tenedor5, terminar_comer5 to tenedor1, terminar_comer5 to pensando5, empezar_comer5 to comiendo5);
    println!("{:?}", arcos_post);

    init!(pensando1{1}, pensando2{1}, pensando3{1}, pensando4{1}, pensando5{1}, 
          comiendo1{0}, comiendo2{0}, comiendo3{0}, comiendo4{0}, comiendo5{0}, 
          tenedor1{1}, tenedor2{1}, tenedor3{1}, tenedor4{1}, tenedor5{1}); 

    
          
    let mut vec_plazas = vec![pensando1, pensando2, pensando3, pensando4, pensando5, 
                              comiendo1, comiendo2, comiendo3, comiendo4, comiendo5, 
                              tenedor1, tenedor2, tenedor3, tenedor4, tenedor5];

    for item in &vec_plazas {
        println!("{:?}", item);
    }

    let mut vec_transiciones = vec![empezar_comer1, empezar_comer2, empezar_comer3, empezar_comer4, empezar_comer5,
                                    terminar_comer1, terminar_comer2, terminar_comer3, terminar_comer4, terminar_comer5];

    for item in &vec_transiciones {
        println!("{:?}", item);
    }
    
    list_enabled!(vec_transiciones);
    update_enabled!(vec_plazas vec_transiciones arcos_pre);
    list_enabled!(vec_transiciones);
    update_enabled!(vec_plazas vec_transiciones arcos_pre);
    list_enabled!(vec_transiciones);
    list_marc!(vec_plazas);



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
