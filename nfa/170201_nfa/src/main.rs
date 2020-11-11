/*---------------------------------------------------------
 *  NFA
 *---------------------------------------------------------*/

/* *****************************************************************************
 *  Name:    Juan Humberto Herrera Martínez
 *  NetID:   JuanH13
 *  Precept: P00
 *
 *  Description:  Program a NFA which comply with the expression a*ba* 
 * 				  using regex library in Rust programming language
 *
 *  Written:       10/13/2020
 *  Last updated:  10/13/2020
 *
 *  % Rut NFA/src/main.rs
 *  % Direct NFA
 *  Regular expresions
 *  +-------------------------------------+
 *  |    Ingrese una cadena a evaluar:    |
 *  +-------------------------------------+
 *  aba
 *  +--------------+---------+-----------+---------------+
 *  |  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |
 *  +--------------+---------+-----------+---------------+
 *  |      0       |         |           |       1       |
 *  +--------------+---------+-----------+---------------+
 *  |      1       |         |           |       2       |
 *  +--------------+---------+-----------+---------------+
 *  |      2       |    a    |     a     |       3       |
 *  +--------------+---------+-----------+---------------+
 *  |      3       |         |           |       4       |
 *  +--------------+---------+-----------+---------------+
 *  |      4       |         |           |       5       |
 *  +--------------+---------+-----------+---------------+
 *  |      5       |    b    |     b     |       6       |
 *  +--------------+---------+-----------+---------------+
 *  |      6       |         |           |       7       |
 *  +--------------+---------+-----------+---------------+
 *  |      7       |         |           |       8       |
 *  +--------------+---------+-----------+---------------+
 *  |      8       |    a    |     a     |       9       |
 *  +--------------+---------+-----------+---------------+
 *  |      9       |         |           |       10      |
 *  +--------------+---------+-----------+---------------+
 *  |      10      |         |           |       11      |
 *  +--------------+---------+-----------+---------------+
 *  |      11      |         |Fin Cadena |               |
 *  +--------------+---------+-----------+---------------+
 *  |                Cadena Valida                       |
 *  +----------------------------------------------------+
 *  
 *
 **************************************************************************** */

//Llamada a la librería de expresiones rgulares
extern crate regex;

//Invocación de los librerías 
use std::io;        //Entradas por teclado
use regex::Regex;   //Expresiones regulares
use std::process;   //Procesos

//Función para verificar entradas mediante expresiones regulares [Colavoración o Apoyo con Alexis Aguirre]
fn caracter(character: char, estado:i32) -> i32
{
    let fin="\r";
    let a = Regex::new(r"a").unwrap();  //Expresión regular para a*
    let b = Regex::new(r"b").unwrap();  //Expresión regular para b
    
    //Pasar caracter a una cadena
    let mut s: String = "".to_string();
	s.push(character);
	let new_character: &str = &s[..];

    //Si se recibe cualquier estado correspondiente al vacío devielve 0 [Colavoración con David Rodríguez]
    if estado == 0 || estado == 1 || estado == 3 || estado == 4 || estado == 6 || estado == 7 || estado == 9 || estado == 10 
    {
        return 0;
    } 

     //Si recibe el estado final muestra que la cadena es valida automáticamente
    if estado == 11
    {
        println! ("|      {}      |         |Fin Cadena |               |",estado);
        body();
        println! ("|                Cadena Valida  :D                   |");
        println! ("+----------------------------------------------------+");
        process::exit(0x0100);
    }

    //comparamos si la entrada es a o b
    if a.is_match(new_character)
    {
        return 1;       //a
    }
    else 
    {
	    if b.is_match(new_character) 
	    {
	        return 2;       //b
	    }
	    else 
	    {
	    	if new_character==fin
		    {
		    	return 3;       //Fin de cadena
	    	}
	    }
	    //si no es ni una a ni una b entonces es un caracter no valido
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
    println! ("|      {}       |    {}    | {}|       {}       |",estadosig,character,simbolo,estado);
    body();
}
//solo muestra la linea que se repetira cada vez que la mandemos a llamar
fn body()
{
    println! ("+--------------+---------+-----------+---------------+");
}


fn main()
{
    //Este es la tabla de transiciones del automata NFA creado
    let tabla: Vec<Vec<char>>;
    tabla = vec![
                vec! ['1','E','E','E'],     // q0
                vec! ['2','E','E','E'],     // q1
                vec! ['E','3','E','E'],     // q2
                vec! ['4','E','E','E'],     // q3
                vec! ['5','E','E','E'],     // q4
                vec! ['E','E','6','E'],     // q5
                vec! ['7','E','E','E'],     // q6
                vec! ['8','E','E','E'],     // q7
                vec! ['E','9','E','E'],     // q8
                vec! ['a','E','E','E'],     // q9
                vec! ['b','E','E','E'],     // q10
                vec! ['E','E','E','A']      // q11
            ];
    
    //Define el estado inical
    let mut estado: i32 = 0;
    

    // Entrada de datos por teclado
    let mut simbolo: String = "".to_string();
    println! ("+-------------------------------------+");
    println! ("|    Ingrese una cadena a evaluar:    |");
    println! ("+-------------------------------------+");
    let mut cadena = String::new();		
    io::stdin().read_line(&mut cadena);

    //Comienza la impresión de la tabla
    body();
    encabezado();

    //ciclo para recorrer la cadena
    for  character in cadena.chars()
    {
        //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut charcaracter = caracter(character,estado);

        while charcaracter == 0
        {
            let  estadosig: i32 = estado;
            //guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
            estado =  (tabla[ estado as usize][charcaracter as usize]) as i32 - '0' as i32 ;

            /////////////Para la recurción en caso de que halla vacío ante o después de b/////////////
            
            if estadosig == 1  && character == 'b'  //Si está en q1 y sigue b
            {
                estado = 4;                         //Salta a q4
            }

            if estadosig == 7  && character == '\r' //Si está en q7 y sigue el fin de cadena
            {
                estado = 10;                         //Salta a q10
            }

            /////////////Para la recurción de a*/////////////
            if estadosig == 4  && character != 'b'  //Si está en q4 y no sigue b
            {
                estado = 1;                         //Regresa a q1
            }

            if estadosig == 10  && character != '\r' //Si está en q10 y no sigue el fin de cadena
            {
                estado = 7;                         //Regresa a q7
            }
            
            //Si llega a q9 y el estado siguiente corresponde a 10, denotado con el caracter 'a'
            if(estado == 49)
            {   
                estado=10;      //Marca el estado siguiente como 10
            }
            //Si llega a q10 y el estado siguiente corresponde a 11, denotado con el caracter 'b'
            if(estado == 50)
            {
                estado=11;      //Marca el estado siguiente como 10
            }

            //Muestra en la tabla lo correspondiente a vacío
            contenido(estadosig,' ',&"          ".to_string(),estado);

            //Vuelve a calcular charcaracter
            charcaracter = caracter(character,estado);
        }

        // Se recalcula el estado y el estado siguiente segun la tabla de tranición
        let  estadosig: i32 = estado;
        estado = (tabla[ estado as usize][charcaracter as usize]) as i32 - '0' as i32 ;

        //Identifica el smibolo a, b o F(Fin de cadena)
        if charcaracter == 1    
        {
            simbolo = "    a     ".to_string();
        }
        else if charcaracter == 2
        {
            simbolo = "    b     ".to_string();
        }
        else if charcaracter == 3
        {
            simbolo = "Fin Cadena".to_string();
        }

        //Si el valor obtenido es una E imprimimos cadena no valida
        if estado==21
        {
            let mut character_imp = character;
            if character_imp == '\r'
            {
                character_imp = ' ';
            }

            println! ("|      {}       |    {}    |{} |     Error     |",estadosig,character_imp,simbolo);
            body();
            println! ("|              Cadena No Valida :(                   |");
            println! ("+----------------------------------------------------+");
            process::exit(0x0100);
        }
        contenido(estadosig,character,&simbolo,estado); 
    }  
}
