
//define place piensa1,piensa2,,,piensan;    //esta es la instruccion de definicion de plazas

//define trans empieza1,empieza2,,,,empiezan, comiendo1,,,comiendon;   // esta es la instruccion de definicion de transiciones

//define arc piensa1 to empieza1, comiendo1 to piensa1;  // esta es la instruccion de definicion de arcos

//define init piensa1{1},,,piensan{0}   // definicion del marcado inicial donde {n} indican la cantidad de tokens en la plaza respectiva

//fire empiezan  // dispara la transicion empiezan

//fire any  // dispara una transicion aleatoria

//fire all   // dispara todas la transiciones que esten habilitadas en ese momento

//list marc  // muestra el marcado actual

//list enabled // muestra las transiciones habilitadas

