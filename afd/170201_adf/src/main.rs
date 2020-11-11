/*---------------------------------------------------------
 *  AFD
 *---------------------------------------------------------*/

/* *****************************************************************************
 *  Name:    Juan Humberto Herrera Martínez
 *  NetID:   JuanH13
 *  Precept: P00
 *
 *  Description:  It exemplifies the way in which regex library
 * 				  is used in the Rust programming language
 *
 *  Written:       10/05/2020
 *  Last updated:  10/05/2020
 *
 *  % Rut afd/src/main.rs
 *  % Direct AFD
 *  Regular expresions
 *  +-------------------------------------+
 *  |    Ingrese una cadena a evaluar:    |
 *  +-------------------------------------+
 *  6+6
 *  +--------------+---------+-----------+---------------+
 *  |  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |
 *  +--------------+---------+-----------+---------------+
 *  |      0       |    6    |   Digito  |      1        |
 *  +--------------+---------+-----------+---------------+
 *  |      1       |    +    |  Operador |      2        |
 *  +--------------+---------+-----------+---------------+
 *  |      2       |    6    |   Digito  |      3        |
 *  +--------------+---------+-----------+---------------+
 *  |      3       |         |Fin Cadena |               |
 *  +--------------+---------+-----------+---------------+
 *  |                Cadena Valida                       |
 *  +----------------------------------------------------+
 *  
 *
 **************************************************************************** */


extern crate regex;

use std::io;
use regex::Regex;
use std::process;

fn caracter(character: char) -> i32
{
    let mut Fin="";
    let digito = Regex::new(r"[0-9]").unwrap();
    let operador = Regex::new(r"(\+|\-|\*|/)").unwrap();
    
    //Pasar caracter a una cadena
    let mut s = String::from("");
	s.push(character);
	let newCharacter: &str = &s[..];

    //comparamos si es digito o operador
    if digito.is_match(newCharacter)
    {
        return 0;
    }
    else 
    {
	    if operador.is_match(newCharacter) 
	    {
	        return 1;
	    }
	    else 
	    {
	    	if newCharacter==Fin
	    	{
	       		return 2;
	    	}
	    }
	    //si no es ni un digito ni un operador entonces es un caracter no validp
		println!("Error el caracter: {} no es valido",character);
		process::exit(0x0100);
    }    
}

//definimos al la funcion  encabezado
fn encabezado()
{
    println! ("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
    body();
}

//definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
fn contenido(estadosig: i32,character: char,simbolo: &str,estado: i32)
{
    println! ("|      {}       |    {}    |  {} |      {}        |",estadosig,character,simbolo,estado);
    body();
}
//solo muestra la linea que se repetira cada vez que la mandemos a llamar
fn body()
{
    println! ("+--------------+---------+-----------+---------------+");
}


fn main()
{
    //Este es la tabla de transiciones del automata AFD creado
    let tabla: Vec<Vec<i32>>;
    
    
    tabla = vec![
                vec! [1,-100,-100],
                vec! [-100,2,-100],
                vec! [3,-100,-100],
                vec! [-100,-100,-200]
            ];
    
    let mut estado: i32 = 0;
    let mut simbolo: String = "".to_string();
    println! ("+-------------------------------------+");
    println! ("|    Ingrese una cadena a evaluar:    |");
    println! ("+-------------------------------------+");

    let mut cadena = String::new();		// Entrada de datos por tectlado
    io::stdin().read_line(&mut cadena);
    body();
    encabezado();

    //ciclo para recorrer la cadena
    for  character in cadena.chars()
    {
        let mut estadosig: i32 = estado;
        
        //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut charcaracter= caracter(character);
        
        //guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
        estado = tabla[estado as usize][charcaracter as usize];
    
      	if charcaracter == 0
      	{
      	    simbolo = " Digito ".to_string();
      	}
      	else if charcaracter == 1
      	{
      	    simbolo = "Operador".to_string();
      	}


        //Si el valor obtenido es una E imprimimos cadena no valida
        if estado==-100
        {
            println! ("|    {}    |  {}    | Error |     {}       |",estadosig,character,estado);
            body();
            println! ("|              Cadena No Valida :(                   |");
            println! ("+----------------------------------------------------+");
//            process::exit(0x0100);
        }
        contenido(estadosig,character,&simbolo,estado);

	    //al concluir si el estado no es 3 que es el de aceptacion imprimimos cadena no valida    
		if estado>3
		{
			println! ("|              Cadena No Valida :(                   |");
			println! ("+----------------------------------------------------+");
		}
		//si el estado es 3 es una cadena de aceptacion
		if estado==3
		{
			println! ("|      {}       |         |Fin Cadena |               |",estado);
			body();
			println! ("|                Cadena Valida                       |");
			println! ("+----------------------------------------------------+");
		}  
    }  
}