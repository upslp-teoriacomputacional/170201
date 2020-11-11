/*---------------------------------------------------------
 *  AP - a^nb^n
 *---------------------------------------------------------*/

/* *****************************************************************************
 *  Name:    Juan Humberto Herrera Martínez
 *  NetID:   JuanH13
 *  Precept: P00
 *
 *  Description:  Program a AP which comply with the expression a^nb^n 
 * 				  using a Stack 
 *
 *  Written:       11/11/2020
 *  Last updated:  11/11/2020
 *
 *  % Rut anbn/src/main.rs
 *  % Direct AP
 *      PushDown Automaton
 *  
 *  +-------------------------------------+
 *  |    Ingrese una cadena a evaluar:    |
 *  +-------------------------------------+
 *  aaabbb
 *  +--------------+---------+-----------+---------------+
 *  |  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente | PILA
 *  +--------------+---------+-----------+---------------+
 *  |      0       |    Ɛ    |           |       0       |[]
 *  +--------------+---------+-----------+---------------+
 *  |      0       |    a    |     a     |       1       |['x']
 *  +--------------+---------+-----------+---------------+
 *  |      1       |    a    |     a     |       1       |['x', 'x']
 *  +--------------+---------+-----------+---------------+
 *  |      1       |    a    |     a     |       1       |['x', 'x', 'x']
 *  +--------------+---------+-----------+---------------+
 *  |      1       |    b    |     b     |       2       |['x', 'x']
 *  +--------------+---------+-----------+---------------+
 *  |      2       |    b    |     b     |       2       |['x']
 *  +--------------+---------+-----------+---------------+
 *  |      2       |    b    |     b     |       2       |[]
 *  +--------------+---------+-----------+---------------+
 *  |      2       |         |Fin Cadena |               |
 *  +--------------+---------+-----------+---------------+
 *  |                Cadena Valida  :D                   |
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
fn caracter(character: char) -> i32
{
    let a = Regex::new(r"a").unwrap();  //Expresión regular para a
    let b = Regex::new(r"b").unwrap();  //Expresión regular para b
    
    //Pasar caracter a una cadena
    let mut s: String = "".to_string();
	s.push(character);
	let new_character: &str = &s[..];

    //comparamos si la entrada es a o b
    if a.is_match(new_character)
    {
        return 0;       //a
    }
    else 
    {
	    if b.is_match(new_character) 
	    {
	        return 1;       //b
	    }
	    else 
	    {
	    	if character=='\n' || character=='\r'
		    {
		    	return 2;       //Fin de cadena o salto de linea
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
    println! ("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente | PILA ");
    body();
}

//definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
fn contenido(estadosig: i32,character: char,simbolo: &str,estado: i32,pila: &Vec<char>)
{
    println! ("|      {}       |    {}    | {}|       {}       |{:?}",estadosig,character,simbolo,estado,pila);
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
    tabla = vec![  //  a   b   Fin
                vec! ['1','E','A'],     // q0
                vec! ['1','2','E'],     // q1
                vec! ['E','2','A']      // q2
            ];
    
    //Define el estado inical
    let mut estado: i32 = 0;
    
    //Define la Pila
    let mut pila: Vec<char> = Vec::new();

    //Se define una variable que indica que e trata de hacer pop a una pila vacía
    let mut poped: bool = false;

    // Simbolo que lee el autómata
    let mut simbolo: String = "".to_string();

    // Entrada de datos por teclado
    println! ("+-------------------------------------+");
    println! ("|    Ingrese una cadena a evaluar:    |");
    println! ("+-------------------------------------+");
    let mut cadena = String::new();		
    io::stdin().read_line(&mut cadena);

    //Comienza la impresión de la tabla
    body();
    encabezado();

    //Imprime la primera transición a Vacío
    contenido(0,'Ɛ',"          ",0,&pila); 

    //ciclo para recorrer la cadena
    for  character in cadena.chars()
    {
        //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let charcaracter = caracter(character);
        
        // Se recalcula el estado y el estado siguiente segun la tabla de tranición
        let  estadosig: i32 = estado;
        estado = (tabla[ estado as usize][charcaracter as usize]) as i32 - '0' as i32 ;

        //Identifica el smibolo a, b o F(Fin de cadena)
        if charcaracter == 0    
        {
            simbolo = "    a     ".to_string();
            pila.push('x');
        }
        else if charcaracter == 1
        {
            simbolo = "    b     ".to_string();
            if pila.is_empty()// Si el imbolo es b y la pila ya etá vacía antes del Pop
            {   //Se indica a la variable
                poped = true;
            }
            pila.pop();
        }
        else if charcaracter == 2
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

        //Si recibe el estado final
        if estado == 17 
        {
            //Imprime la salida
            println! ("|      {}       |         |Fin Cadena |               |",estadosig);
            body();

            // Si se llegó al final de cadena con la pila vacía, y no se intentó hacer Pop a una pila vacía la cadena es valida
            if pila.is_empty() && !poped
            {
                println! ("|                Cadena Valida  :D                   |");
                println! ("+----------------------------------------------------+");
            }
            else
            {
                println! ("|              Cadena No Valida :(                   |");
                println! ("+----------------------------------------------------+");
            }
            process::exit(0x0100);
        }

        contenido(estadosig,character,&simbolo,estado,&pila); 
    }  
}
