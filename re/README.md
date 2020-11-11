# Datos personales

•	**Institución:** Universidad politécnica de San Luis Potosí

•	**Matricula:** 170201

•	**Alumno:** Juan Humberto Herrera Martínez

•	**Carrera:** Ingeniería en Tecnologías de la Información

•	**Materia:** Teoría computacional

•	**Docente:** Juan Carlos González Ibarra 

# Notas:

   - El nombre del proyecto es 170201_re, cuenta con el codigo fuente y lo necesario para ejecutar un proyecto Rust
   - Para exportar la librería Regex se necesita que el programa esté en formato de proyecto
   - El nombre del archivo fuente es main.rs para cumplir con esta condición. 

# Objetivos 
1.	En este programa se identificó una linea de codigo y se analiza ,ediante expresiones regulares.
2.	Dependiendo de la linea de código se identitica sus partes escenciales: Tipo de dato, identificador, valor y fin de cadena,


# Como solucionaste el problema
Dentro de Rust se puede instanciar a la librería Regex. Ésta proporciona una biblioteca para analizar, compilar y ejecutar expresiones regulares. Su sintaxis es similar a las expresiones regulares de estilo Perl, pero carece de algunas características como mirar alrededor y referencias inversas. A cambio, todas las búsquedas se ejecutan en tiempo lineal con respecto al tamaño de la expresión regular y el texto de búsqueda.
Lo siguiente fue realizar las modificaciónes necesarias al programa AFD antes realizado.


### NFA.

•	*Cargo.TOML*

	[package]
	name = "nfa"
	version = "0.1.0"
	authors = ["JuanH13 <170201@upslp.edu.mx>"]
	edition = "2018"

	[dependencies]
	regex = "1"


•	*Código fuente*

```Rust
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
	

```


# Problemas y soluciones al programar 

Ya que una cadena no se puede iterarse como tal, e debe acceder a sus elementos utilizando otros métodos.

```Rust
	//Teke the ; from the string
	let aux_w: u8 = word.as_bytes()[(word.len() - 1) as usize];
	let mut aux_w2: String = word.to_string();

```

Ya que no se puede iterar una cadena, se opta por introducir sus elementos dentro de un vector para acceder a sus elementos individualmente.

```Rust
	let mut w: Vec<char> = Vec::new();

	for w_chars in w_string.chars() {
		w.push(w_chars);
	}
```

La función para convertir caracteres a mayuscula devuelve un vector, no un caracter, por lo que se debe obtener de esta forma.

```Rust
	let aux_w: Vec<_> = w[0].to_uppercase().collect();
```

Para quitar el primer caracter no se puede tealizar una peración como en python, se debe invertir el vector y quitar el último elemento, para luego volverla a invertir..

```Rust
	w.reverse();
	w.pop();
	w.reverse(); 
```

	