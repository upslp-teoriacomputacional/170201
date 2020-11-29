/*---------------------------------------------------------
 *  Turing Machine
 *---------------------------------------------------------*/

/* *****************************************************************************
 *  Name:    Juan Humberto Herrera Martínez
 *  NetID:   JuanH13
 *  Precept: P00
 *
 *  Description:  Program a Turing machine that solves the division algorithm 
 *
 *  Written:       11/28/2020
 *  Last updated:  11/29/2020
 *
 *  % Rut 170201_TuringMachine/src/main.rs
 *  % Direct TuringMachine 
 *
 **************************************************************************** */
use std::process;
fn turing_m (state: char,				//estados de la maquina de turing
              blank: char, 				//simbolo blanco de el alfabeto dela cinta
              in_tape: Vec<char>,			//cinta
              fin: char,				//estado valido y/o final
              rules: Vec<Vec<char>>,   	//reglas de transicion
              position: usize)					//posicion siguiente de la maquina de turing
{
	
	//Convierte el estado, la cinta y las poiciones en valores mutables para así manipularlos
	let mut pos = position; 
    let mut st = state;
    let mut tape: Vec<char>;
    tape = in_tape;

    //Valida si es una entrada válida con respecto a la posición
    if tape.is_empty()
    {
    	tape.push(blank);
    } 
    if pos < 0
    {
    	pos += tape.len()
    } 
    if pos >= tape.len() || pos < 0
    {
    	println!("Se inicializa mal la posicion");
    	process::exit(0x0100);
    } 

    // Auxiliares para las transiciones
    let mut v1 = ' ';
    let mut dr = ' ';
    let mut s1 = ' ';
    
    //Mientras no se llege al estado final
    while st != fin
    {
    	//Imprime el estado actual
        print! ("{}\t|",st);
        for i in 0..tape.len()
        {
        	//Muestra el caracter actual a analizar
             if i==pos //Si es el que está señalado por el cursor
             {	//Lo muestra resaltado
             	print! (" [{}]", tape[i]);
             }
             else
             {	//Sino solo lo muestra
             	print! (" {}", tape[i]);
             }
        }
        println!("");
        
        //Recorre las reglas
        for rule in &rules
        {	//Si el caracter de la cinta correponde a la regla del estado actual
            if st == rule[0] && tape[pos] == rule[1]   
            {//Guarda
            	v1 = rule[2];	//Caracter por el que se reemplaza
	            dr = rule[3];	//Dirección de la cinta
	            s1 = rule[4];	//Estado siguiente
            }
        }

        //Realiza la sustitución de la cinta
        for x in 0..tape.len()
        {	//Si la posición de la cinta es X
	        if x == pos 
	        {	//Realiza la susutitución
	            tape[x] = v1;
	        }
        }

    //movimiento del cabezal
        if dr == 'L' //Si es a la izquierda
        {
            if pos > 0	//Si llega al extremo
            {
            	pos = pos - 1; //Reduce la posición
            }
            else
            {	//Si no agrega un caracter en blanco al inicio
            	tape.insert(0,blank);
            }
        }
        if dr == 'R' //Si es a la derecha
        {
        	//Aumenta la posición
            pos = pos + 1;
            //Si llega al extremo
            if pos >= tape.len()
            {	//Agrega un caracter en blanco al final
            	tape.push(blank);
            }
        }
        st = s1;//Actualiza estado actual
    }
}



fn main() {
    let rules: Vec<Vec<char>>;
    let tape: Vec<char>;
    
    //Reglas de transición
    rules = vec![
    			//q0
                vec! ['0','B','B','R','0'],
                vec! ['0','1','1','R','0'],
                vec! ['0','/','/','R','0'],
                vec! ['0','=','=','L','1'],
                //q1
                vec! ['1','1','x','L','2'],
                //q2
                vec! ['2','1','1','L','2'],
                vec! ['2','/','/','L','3'], 
                //q3
                vec! ['3','B','B','L','3'],
                vec! ['3','■','■','R','9'],
                vec! ['3','1','B','R','4'],
                //q4
                vec! ['4','x','x','R','4'],
                vec! ['4','B','B','R','4'],
                vec! ['4','/','/','R','B'],
                vec! ['4','=','=','R','5'],
                //q5
                vec! ['5','1','1','R','5'],
                vec! ['5','■','1','L','6'],
                //q6
                vec! ['6','=','=','L','6'], 
                vec! ['6','1','1','L','6'],
                vec! ['6','x','1','L','6'],
                vec! ['6','■','■','R','7'],
                vec! ['6','/','/','L','A'],
				//q7
                vec! ['7','=','=','R','7'], 
                vec! ['7','1','1','R','7'],
                vec! ['7','/','/','R','7'],
                vec! ['7','B','1','R','7'],
                vec! ['7','■','■','R','8'],
				//q8 (Estado final)

				//q9
                vec! ['9','/','/','R','9'],
                vec! ['9','1','1','R','9'],
                vec! ['9','x','1','R','9'], 
                vec! ['9','B','1','R','G'],
                vec! ['9','=','=','R','7'],
				//q10
                vec! ['A','B','B','L','A'],
                vec! ['A','1','1','R','0'],
                vec! ['A','■','■','R','7'],
				//q11
                vec! ['B','1','1','R','B'],
                vec! ['B','x','x','L','C'],
				//q12
                vec! ['C','B','B','R','C'],
                vec! ['C','/','/','L','D'],
                vec! ['C','1','x','L','2'],
				//q13
                vec! ['D','B','B','L','D'],
                vec! ['D','1','1','R','E'],
                vec! ['D','■','■','R','F'],
				//q14
                vec! ['E','B','B','R','E'],
                vec! ['E','x','x','R','E'],
                vec! ['E','/','/','R','E'],
                vec! ['E','=','=','R','5'],
				//q15
                vec! ['F','/','/','R','F'],
                vec! ['F','x','x','R','F'],
                vec! ['F','B','B','R','F'],
                vec! ['F','=','=','R','5'],
				//q16
                vec! ['G','=','=','R','G'], 
                vec! ['G','x','x','R','G'],
                vec! ['G','/','/','R','G'],
                vec! ['G','B','B','R','G'],
                vec! ['G','1','1','R','G'],
                vec! ['G','■','■','R','H'],
				//q17
                vec! ['H','1','1','R','H'],
                vec! ['H','■','1','L','I'],
				//q18
                vec! ['I','■','■','L','I'], 
                vec! ['I','x','x','L','I'],
                vec! ['I','1','1','L','I'],
                vec! ['I','=','=','L','I'],
                vec! ['I','/','/','L','J'],
				//q19
                vec! ['J','B','B','L','J'],
                vec! ['J','1','1','R','N'],
				//q20
                vec! ['K','/','/','R','K'],
                vec! ['K','x','x','R','K'],
                vec! ['K','=','=','R','K'],
                vec! ['K','B','B','R','K'], 
                vec! ['K','1','1','R','K'],
                vec! ['K','■','■','R','L'],
				//q21
                vec! ['L','1','1','R','L'],
                vec! ['L','■','1','L','I'],
				//q22
                vec! ['M','x','1','R','O'],
                vec! ['M','=','=','R','8'],
				//q23
                vec! ['N','/','/','R','M'],
                vec! ['N','B','1','R','K'],
				//q24
                vec! ['O','1','1','R','O'],
                vec! ['O','x','x','R','O'],
                vec! ['O','=','=','R','O'],
                vec! ['O','■','■','R','P'],
				//q25
                vec! ['P','1','1','R','P'],
                vec! ['P','■','■','L','Q'],
				//q26
                vec! ['Q','1','■','L','R'],
				//q27
                vec! ['R','■','■','L','R'],
                vec! ['R','1','1','L','R'],
                vec! ['R','=','=','L','S'],
				//q28
                vec! ['S','x','x','L','S'],
                vec! ['S','1','1','R','M']
            ];

    //Entrada a la cinta "111/11="
    tape = vec! ['1','1','1','/','1','1','='];

    println!("Turing Machine");

    //Inicia la máquina
    turing_m('0','■', tape,'8',rules,0);
}
