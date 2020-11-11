# Datos personales

•	**Institución:** Universidad politécnica de San Luis Potosí

•	**Matricula:** 170201

•	**Alumno:** Juan Humberto Herrera Martínez

•	**Carrera:** Ingeniería en Tecnologías de la Información

•	**Materia:** Teoría computacional

•	**Docente:** Juan Carlos González Ibarra 

# Notas:

   - El nombre del proyecto es 170201_afd, cuenta con el codigo fuente y lo necesario para ejecutar un proyecto Rust
   - Para exportar la librería Regex se necesita que el programa esté en formato de proyecto
   - El nombre del archivo fuente es main.rs para cumplir con esta condición. 

# Objetivos 
1.	En este programa se desarrolló un autómata finito determinita que valida cuatro operaciones matemáticas en el lenguaje de programación Rust.
2.	Representar los estados del autómata mediante expresiones regulares e implementar la libreria asociada al lenguaje de programación.

### Expresiones a utilizar:
   - (0-9) para numeros del 0 al 9.
   - (\+|\-|\*|\/) para los operadores aritméticos basicos.

### Tabla de estados:
	+-------------------------------------+
	|    Ingrese una cadena a evaluar:    |
	+-------------------------------------+
	6+6
	+--------------+---------+-----------+---------------+
	|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |
	+--------------+---------+-----------+---------------+
	|      0       |    6    |   Digito  |      1        |
	+--------------+---------+-----------+---------------+
	|      1       |    +    |  Operador |      2        |
	+--------------+---------+-----------+---------------+
	|      2       |    6    |   Digito  |      3        |
	+--------------+---------+-----------+---------------+
	|      3       |         |Fin Cadena |               |
	+--------------+---------+-----------+---------------+
	|                Cadena Valida                       |
	+----------------------------------------------------+


# Como solucionaste el problema
Dentro de Rust se puede instanciar a la librería Regex. Ésta proporciona una biblioteca para analizar, compilar y ejecutar expresiones regulares. Su sintaxis es similar a las expresiones regulares de estilo Perl, pero carece de algunas características como mirar alrededor y referencias inversas. A cambio, todas las búsquedas se ejecutan en tiempo lineal con respecto al tamaño de la expresión regular y el texto de búsqueda.


### AFD.

•	*Cargo.TOML*

	[package]
	name = "afd"
	version = "0.1.0"
	authors = ["JuanH13 <170201@upslp.edu.mx>"]
	edition = "2018"

	[dependencies]
	regex = "1"


•	*Librerías necearias*

```Rust
	extern crate regex;

	use std::io;		//Entradas por teclado
	use regex::Regex;	//Expresiones regulares
	use std::process;	//Procesos
```
 
•	*Funsión para validar caracteres*

```Rust
    	fn caracter(character: char) -> i32
	{
    		let mut Fin="";
    		let digito = Regex::new(r"[0-9]").unwrap();
    		let operador = Regex::new(r"(\+|\-|\*|\/)").unwrap();

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

```

•	*Funciones de impresión de datos*

```Rust
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
```

•	*Función principal*

```Rust
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
    		let mut cadena = String::new();
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
			        process::exit(0x0100);
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
```


# Problemas y soluciones al programar 
En Ptyhon se pueden evaluar las expresiones regulares entre cadenas y caracteres, pero en Rust solose puede entre cadenas de tipo String, por lo que se debe realizar la conversión antes de procesar los caracteres.

```Rust
	let mut s = String::from("");
	s.push(character);
	let newCharacter: &str = &s[..];
```

Un Vector no puede contener datos de multiples tipos por lo que para manejar el elrror se cambio la cadena "E" por -100

```Rust
	let tabla: Vec<Vec<i32>>;
    	tabla = vec![
                vec! [1,-100,-100],
                vec! [-100,2,-100],
                vec! [3,-100,-100],
                vec! [-100,-100,-200]
            ];
```

Para acceder al contenido de un arreglo se debe utilizar una variable de tipo usize por lo que debe realizarse un cast.

```Rust
	//guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
        estado = tabla[estado as usize][charcaracter as usize];
```


No hay variables globales en Rust, por lo que los operandos e asignaron en el metodo principal a partir de lo obtenido de la función caracter

```Rust
	if charcaracter == 0
      	{
      	    simbolo = " Digito ".to_string();
      	}
      	else if charcaracter == 1
      	{
      	    simbolo = "Operador".to_string();
      	}
```