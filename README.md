# Proyecto de Rust - DSL para Petri Nets

Deberá implementar una serie de **macros en Rust** que permitan modelar y reflejar modelos de **Red de Petri**. En particular el caso de la **cena de los filosofos**, como el que puede verse en:

 * https://www.wikiwand.com/es/Red_de_Petri
 * https://www.wikiwand.com/es/Problema_de_la_cena_de_los_fil%C3%B3sofos
 * https://www.wikiwand.com/fr/D%C3%AEner_des_philosophes
 * https://www.wikiwand.com/fr/R%C3%A9seau_de_Petri

Para ello debera implementar macros en Rust que permitan definir y ejecutar la red, de la siguiente manera:

define place piensa1,piensa2,,,piensan;    //esta es la instruccion de definicion de plazas

define trans empieza1,empieza2,,,,empiezan, comiendo1,,,comiendon;   // esta es la instruccion de definicion de transiciones

define arc piensa1 to empieza1, comiendo1 to piensa1;  // esta es la instruccion de definicion de arcos

define init piensa1{1},,,piensan{0}   // definicion del marcado inicial donde {n} indican la cantidad de tokens en la plaza respectiva

fire empiezan  // dispara la transicion empiezan

fire any  // dispara una transicion aleatoria

fire all   // dispara todas la transiciones que esten habilitadas en ese momento

list marc  // muestra el marcado actual

list enabled // muestra las transiciones habilitadas

Definidas las macros debera generar un programa que defina la Petri Net, ejecute disparos y muestre el estado de la red por medio de los comandos automaticamente o por medio de selecciones que un usuario elegira desde teclado.

## Links

#### Macros en Rust

 * http://danigm.net/rust-macros.html
 * https://doc.rust-lang.org/book/ch19-06-macros.html
 * https://doc.rust-lang.org/stable/rust-by-example/macros.html
 * https://doc.rust-lang.org/reference/macros.html
 * https://danielkeep.github.io/tlborm/book/index.html
 * https://doc.rust-lang.org/book/macros.html#scoping-and-macro-importexport
 * https://doc.rust-lang.org/reference/macros-by-example.html
 
 
