/*---------------------------------------------------------
 *  RE
 *---------------------------------------------------------*/

/* *****************************************************************************
 *  Name:    Juan Humberto Herrera Martínez
 *  NetID:   JuanH13
 *  Precept: P00
 *
 *  Description:  It exemplifies the way in which regex library
 * 				  is used in the Rust programming language
 *
 *  Written:       10/21/2020
 *  Last updated:  10/21/2020
 *
 *  % Rut regularexpression/src/main.rs
 *  % Direct RE
 *  Regular expresions
 *  
 *
 **************************************************************************** */

extern crate regex;		//For performing regex expressions
use regex::Regex;
                                 
fn main()
{
	let mut tokens: Vec<String> = Vec::new();                               // for string tokens
	let  source_code = "int result = 100;".split(" "); // turning source code into list of words
	let types: [&str; 3] = ["str", "int", "bool"];

	let digito = Regex::new(r"[0-9]").unwrap();
	let letras1 = Regex::new(r"[a-z]").unwrap();
	let letras2 = Regex::new(r"[A-Z]").unwrap();
    let operador = Regex::new(r"(\+|\-|\*/)").unwrap();



	// Loop through each source code word
	for word in source_code
	{
	    let mut aux_tokens: String = "".to_string();

	    // This will check if a token has datatype decleration
	    if types.contains(&word)
	    {
	    	aux_tokens = format!("{}{}","DATATYPE: ",word);
	    } // This will look for an identifier which would be just a word
	    else if letras1.is_match(word) || letras2.is_match(word)
	    {
			aux_tokens = format!("{}{}","IDENTIFIER: ",word);
	    }// This will look for an operator
	    else if operador.is_match(word) 
	    {
	    	aux_tokens = format!("{}{}","OPERATOR: ",word);
	    }
	    // This will look for integer items and cast them as a number
	    else if digito.is_match(word)
	    {
	    	//Teke the ; from the string
	    	let aux_w: u8 = word.as_bytes()[(word.len() - 1) as usize];
	    	let mut aux_w2: String = word.to_string();

	    	if (aux_w as char) == ';'
	    	{
	    		aux_w2.remove(word.len() - 1);
	            aux_tokens = format!("{}{}","INTEGER ", aux_w2);
	            tokens.push(aux_tokens);
	            aux_tokens = format!("{}{}","END_STATEMENT",";");
	        }
	        else
	        {
	            aux_tokens = format!("{}{}","INTEGER ",word);
	        }
	    }

	    if aux_tokens != ""
	    {
	    	tokens.push(aux_tokens);
	    }
	}

	for token in &tokens
	{
    	println!("{}", token);// Outputs the token array
	}

}

fn variable_prolog(w_string: String)->bool
{
	let mut w: Vec<char> = Vec::new();

	for w_chars in w_string.chars() {
		w.push(w_chars);
	}

	let aux_w: Vec<_> = w[0].to_uppercase().collect();

	//(str)-->bool. True si "w" es un nombre de variable correcto'''
    if w[0].is_alphabetic() && w[0] == (aux_w[0] as char) || w[0] == '_'
    {
    	//El primer caracter es una mayuscula o un subrayado
        //Se quita el primer caracter
        w.reverse();
		w.pop();
		w.reverse();
        while !w.is_empty() && (w[0].is_numeric() || w[0]  ==  '_')
        {
        	//Mientras queden caracteres en "w" y el primer caracter actual sea un alfanumerico o un subrayado, todo esta bien
            //Quitar el primer caracter
			w.reverse();
			w.pop();
			w.reverse(); 
            if w.is_empty()
            {
              	//Si ya no quedan elementos a revisar, es una variable PROLOG
                return true;
            }
        }                       
    }
    
    return false;
}
        