use std::io::{self, Write};
use rand::Rng; // 0.8.0

#[derive(Debug, Clone)]
struct Plaza {
    nombre : String,
    num_tokens : i32,
    orden : usize
}

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
macro_rules! place {
    ($($plaza: ident{$name: expr}),+) => {
        {
            let mut a = 0;
            $( 
                $plaza = Plaza {nombre : $name.to_string(), num_tokens : 0, orden : a.clone()};
                a = a + 1;
            )+
        }
    };
}

//define trans empieza1,empieza2,,,,empiezan, comiendo1,,,comiendon;   // esta es la instruccion de definicion de transiciones
/// Crea todas las transiciones de la RdP a partir de los nombres de cada una.
#[macro_export]
macro_rules! transition {
    ($($transicion: ident{$name: expr}),+) => {
        {
            let mut a = 0;
            $( 
                $transicion = Transicion {nombre : $name.to_string(), is_sensibilizada : false, orden : a.clone()};
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
        {
            println!("\x1b[1;34mMarcado actual:\x1b[0;37m");
            for plaza in &$plazas{
                println!(" > Plaza {}: {} tokens",plaza.nombre, plaza.num_tokens)
            }
        }
    }

}

#[macro_export]
macro_rules! print_transitions_enabled {
    ($transiciones: ident) => {
        {
            let transiciones = &$transiciones;
            let vec_sensibilizadas = list_enabled!(transiciones);
            println!("\x1b[1;34mTransiciones Sensibilizadas:\x1b[0;37m");
            for transicion in &vec_sensibilizadas{
                println!(" > Transicion {}", transicion.nombre);
            }
        }
    }

}

#[macro_export]
macro_rules! list_enabled {
    ($transiciones: ident) => {
        {
            let mut vec_sensibilizadas : Vec<Transicion> = Vec::new();
            for transicion in $transiciones{
                if(transicion.is_sensibilizada){
                    vec_sensibilizadas.push(transicion.clone());
                }
            }
            vec_sensibilizadas
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

//ver transiciones sensibilizadas
//elegir una al azar
//quitar 1 token en plaza de arcos pre
//sumar 1 token en plaza de arcos post

//fire any  // dispara una transicion aleatoria
#[macro_export]
macro_rules! fire_any {
    ($plazas: ident, $transiciones: ident, $arc_pre : ident, $arc_post : ident) => {
        {
            let vec_trans = &$transiciones;
            let sensibilizadas = list_enabled!(vec_trans);
            // Generate random number in the range [0, n)
            let num_trans = rand::thread_rng().gen_range(0..sensibilizadas.len());
            //println!("NUM ALEATORIO: {:?}", num_trans);
            let plazas = &mut $plazas;
            let arc_pre = &$arc_pre;
            let arc_post = &$arc_post;
            let trans_a_disparar = sensibilizadas[num_trans].clone();
            fire_transition!(plazas, trans_a_disparar, arc_pre, arc_post); 
            update_enabled!($plazas $transiciones $arc_pre);
        }

    };
}

/// Dispara una transicion. Saca un token de cada plaza de entrada a la transicion y
/// agrega un token en cada plaza de salida.
/// El matcheo es viendo el nombre de las plazas y transicion.
#[macro_export]
macro_rules! fire_transition {
    ($plazas: ident, $transicion: ident, $arc_pres : ident, $arc_post : ident) => {
        {
            // for i in (0..$arc_pres.len()){
            //     if $arc_pres[i].transicion.nombre == $transicion.nombre {
            //         for j in (0..$plazas.len()){
            //             if $arc_pres[i].plaza.nombre == $plazas[j].nombre {
            //                 $plazas[j].num_tokens -= 1;
            //                 println!("saco en plaza {}", $plazas[j].nombre);
            //                 break;
            //             }
            //         }
            //     }
            // }

            //saca un token de todas las plazas de entrada a la transicion
            for i in (0..$arc_pres.len()){
                if ($arc_pres[i].transicion.nombre == $transicion.nombre) {
                    let iplaza = $arc_pres[i].plaza.orden;
                    $plazas[iplaza].num_tokens -= 1;
                    //println!("saco en plaza {}", $plazas[iplaza].nombre);
                }
            }

            // for k in (0..$arc_post.len()){
            //     if $arc_post[k].transicion.nombre == $transicion.nombre {
            //         for l in (0..$plazas.len()){
            //             if $arc_post[k].plaza.nombre == $plazas[l].nombre {
            //                 $plazas[l].num_tokens += 1;
            //                 println!("pongo en plaza {}", $plazas[l].nombre);
            //                 break;
            //             }
            //         }
            //     }
            // }
            
            //agrega un token en c/u de las plazas de salida de la transicion
            for k in (0..$arc_post.len()){
                if $arc_post[k].transicion.nombre == $transicion.nombre {
                    let iplaza = $arc_pres[k].plaza.orden;
                    $plazas[iplaza].num_tokens += 1;
                    //println!("pongo en plaza {}", $plazas[iplaza].nombre);
                }
            }
            println!("\x1b[3;32m Se disparo la transicion {}\x1b[0;37m", $transicion.nombre);
        }
    };
}

//fire all   // dispara todas la transiciones que esten habilitadas en ese momento
#[macro_export]
macro_rules! fire_all {
    ($plazas: ident, $transiciones: ident, $arc_pre : ident, $arc_post : ident) => {
        {
            let vec_trans = &$transiciones;
            let sensibilizadas = list_enabled!(vec_trans);
            for i in (0..sensibilizadas.len()){ 
                let t = sensibilizadas[i].clone();
                if($transiciones[t.orden].is_sensibilizada == true){
                    fire_transition!($plazas, t, $arc_pre, $arc_post);
                }
                update_enabled!($plazas $transiciones $arc_pre);
            }

        }

    };
}

#[macro_export]
macro_rules! fire_empieza {
    ($plazas: ident, $transiciones: ident, $arc_pre : ident, $arc_post : ident, $num : expr) => {
        {
            if($num > 0 && $num < 6){
                let n : usize = ($num - 1);
                if($transiciones[n].is_sensibilizada){
                    let t = $transiciones[n].clone();
                    fire_transition!($plazas, t, $arc_pre, $arc_post);
                    update_enabled!($plazas $transiciones $arc_pre);
                } else{
                    println!("\x1b[1;33m No está sensibilizada la transicion {}\x1b[0;37m", $transiciones[n].nombre);
                }
            } else {
                println!("\x1b[1;31m El número de Filosofo incorrecto. Debe ser entre 1 y 5.\x1b[0;37m");
            }
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

    place!(pensando1{"pensando1"}, pensando2{"pensando2"}, pensando3{"pensando3"}, pensando4{"pensando4"}, 
           pensando5{"pensando5"}, comiendo1{"comiendo1"}, comiendo2{"comiendo2"}, comiendo3{"comiendo3"}, 
           comiendo4{"comiendo4"}, comiendo5{"comiendo5"}, tenedor1{"tenedor1"}, tenedor2{"tenedor2"}, 
           tenedor3{"tenedor3"}, tenedor4{"tenedor4"}, tenedor5{"tenedor5"}); 

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

    transition!(empezar_comer1{"empezar_comer1"}, empezar_comer2{"empezar_comer2"}, empezar_comer3{"empezar_comer3"}, 
                empezar_comer4{"empezar_comer4"}, empezar_comer5{"empezar_comer5"}, terminar_comer1{"terminar_comer1"}, 
                terminar_comer2{"terminar_comer2"}, terminar_comer3{"terminar_comer3"}, terminar_comer4{"terminar_comer4"}, 
                terminar_comer5{"terminar_comer5"});
                                 
    // ARCOS
    let mut arcos_pre = arc_pre!(tenedor1 to empezar_comer1, tenedor2 to empezar_comer1, pensando1 to empezar_comer1, comiendo1 to terminar_comer1,
                                tenedor2 to empezar_comer2, tenedor3 to empezar_comer2, pensando2 to empezar_comer2, comiendo2 to terminar_comer2,
                                tenedor3 to empezar_comer3, tenedor4 to empezar_comer3, pensando3 to empezar_comer3, comiendo3 to terminar_comer3,
                                tenedor4 to empezar_comer4, tenedor5 to empezar_comer4, pensando4 to empezar_comer4, comiendo4 to terminar_comer4,
                                tenedor5 to empezar_comer5, tenedor1 to empezar_comer5, pensando5 to empezar_comer5, comiendo5 to terminar_comer5);
   // println!("{:?}", arcos_pre);

    let mut arcos_post = arc_post!(terminar_comer1 to tenedor1, terminar_comer1 to tenedor2, terminar_comer1 to pensando1, empezar_comer1 to comiendo1,
                                terminar_comer2 to tenedor2, terminar_comer2 to tenedor3, terminar_comer2 to pensando2, empezar_comer2 to comiendo2,
                                terminar_comer3 to tenedor3, terminar_comer3 to tenedor4, terminar_comer3 to pensando3, empezar_comer3 to comiendo3,
                                terminar_comer4 to tenedor4, terminar_comer4 to tenedor5, terminar_comer4 to pensando4, empezar_comer4 to comiendo4,
                                terminar_comer5 to tenedor5, terminar_comer5 to tenedor1, terminar_comer5 to pensando5, empezar_comer5 to comiendo5);
  // println!("{:?}", arcos_post);

    // MARCADO INICIAL
    init!(pensando1{1}, pensando2{1}, pensando3{1}, pensando4{1}, pensando5{1}, 
          comiendo1{0}, comiendo2{0}, comiendo3{0}, comiendo4{0}, comiendo5{0}, 
          tenedor1{1}, tenedor2{1}, tenedor3{1}, tenedor4{1}, tenedor5{1}); 

    
          
    let mut vec_plazas = vec![pensando1, pensando2, pensando3, pensando4, pensando5, 
                              comiendo1, comiendo2, comiendo3, comiendo4, comiendo5, 
                              tenedor1, tenedor2, tenedor3, tenedor4, tenedor5];

   /* for item in &vec_plazas {
        println!("{:?}", item);
    }*/

    let mut vec_transiciones = vec![empezar_comer1, empezar_comer2, empezar_comer3, empezar_comer4, empezar_comer5,
                                    terminar_comer1, terminar_comer2, terminar_comer3, terminar_comer4, terminar_comer5];

    /*for item in &vec_transiciones {
        println!("{:?}", item);
    }*/
    
    //print_transitions_enabled!(vec_transiciones);
    update_enabled!(vec_plazas vec_transiciones arcos_pre);
    



    //Interaccion con el usuario - recepcion de comandos
    println!("\x1b[1;36m\n >> Bienvenidos a Petri Net Simulator << \x1b[0;37m");
    println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", 
            "\x1b[3;37m'Cinco filósofos se sientan alrededor de una mesa y pasan su vida cenando y pensando.",
            "Cada filósofo tiene un plato de fideos y un tenedor a la izquierda de su plato.",
            "Para comer los fideos son necesarios dos tenedores y cada filósofo sólo puede tomar ",
            "los que están a su izquierda y derecha. Si cualquier filósofo toma un tenedor ",
            "y el otro está ocupado, se quedará esperando, con el tenedor en la mano, ",
            "hasta que pueda tomar el otro tenedor, para luego empezar a comer.",
            "Si dos filósofos adyacentes intentan tomar el mismo tenedor al mismo tiempo, ",
            "ambos compiten por tomar el mismo tenedor, y uno de ellos se queda sin comer.'\x1b[0;37m");
    loop {
           println!("\n\x1b[1;37m\x1b[4;37mSeleccione una acción a realizar\x1b[0;37m");
           println!("\x1b[1;37m 1)\x1b[0;37m Disparar transicion empiezan");
           println!("\x1b[1;37m 2)\x1b[0;37m Disparar una transicion al azar");
           println!("\x1b[1;37m 3)\x1b[0;37m Disparar todas las transiciones habilitadas");
           println!("\x1b[1;37m 4)\x1b[0;37m Mostrar el marcado actual");
           println!("\x1b[1;37m 5)\x1b[0;37m Mostrar transiciones habilitadas");
           println!("\x1b[1;37m 6)\x1b[0;37m Finalizar");
           print!("{} >> ", '\u{1F980}');
           io::stdout().flush().ok();

           let mut input_comando = String::new();
           let mut num_filosofo = String::new();
           io::stdin().read_line(&mut input_comando).ok().expect("Error al leer de teclado");
          // println!("El comando es {}", input_comando.trim()); //trim quita el \n final 
           match input_comando.trim() {
               "1" => {
                    print!("Ingrese el número del filosofo: ");
                    io::stdout().flush().ok();
                    io::stdin().read_line(&mut num_filosofo).ok().expect("Error al leer de teclado");
                    let nro = num_filosofo.trim().parse::<usize>().unwrap();
                    fire_empieza!(vec_plazas, vec_transiciones, arcos_pre, arcos_post, nro);
                },
               "2" => {fire_any!(vec_plazas, vec_transiciones, arcos_pre, arcos_post);},
               "3" => {fire_all!(vec_plazas, vec_transiciones, arcos_pre, arcos_post);},
               "4" => {list_marc!(vec_plazas);},
               "5" => {print_transitions_enabled!(vec_transiciones);},
               "6" => break,
               _ => println!("\x1b[1;31m Opcion incorrecta \x1b[0;37m")
           }
    }


}
