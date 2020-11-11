# Datos personales

•	**Institución:** Universidad politécnica de San Luis Potosí

•	**Matricula:** 170201

•	**Alumno:** Juan Humberto Herrera Martínez

•	**Carrera:** Ingeniería en Tecnologías de la Información

•	**Materia:** Teoría computacional

•	**Docente:** Juan Carlos González Ibarra 


# Objetivos 
1.	En este programa se aborda lo esencial dentro de las operaciones a nivel de bit en el lenguaje de programación Rust.
2.	Entre las funciones básicas y operaciones a utilizar se encuentran

### Operaciones a bit:
   AND
   OR
   NOT
   XOR


# Como solucionaste el problema
Dentro de Rust existen operadores a nivel de bit, donde los más comunes son:
  - AND (&) donde S(a,b) = {x | a = True y b = True}
  - OR (|) donde S(a,b) = {x | a = True o b = True}
  - NOT (!) donde S(a) = {x | x = complemento de a}
  - XOR (^) donde S(a) = {x | a = True y/o b = True}



### Tablas de verdad.
•	*Definir Arreglo booleano*

```Rust
	// We start by creating a list with the two possible Boolean values, False and True
	let mut booleanos: [bool; 2]= [false, true];
```
 
•	*Tabla de verdad OR*

```Rust
    //Disjunctive truth table
    println!("x\ty\tx or y");
    println!("----------------------");
    
    for x in booleanos.iter()
    {
        for y in booleanos.iter()
        {
            println!("{}\t{}\t{}",x,y,(*x | *y));
        }
    }
```

•	*Tabla de verdad AND*

```Rust
    //Conjunction truth table
    println!("x\ty\tx and y");
    println!("----------------------");
    
    for x in booleanos.iter()
    {
        for y in booleanos.iter()
        {
            println!("{}\t{}\t{}",x,y,(*x & *y));
        }
    }       
```

•	*Tabla de verdad NOT*

```Rust
    //Negation truth table
    println!("x\tnot x");
    println!("-------------");
    for x in booleanos.iter()
    {
        println!("{}\t{}",x,!x);
    }
```

•	*Tabla de verdad de XOR*


```Rust
    //Exclusion truth table
    println!("x\ty\tx^y");
    println!("----------------------");
    for x in booleanos.iter()
    {
        for y in booleanos.iter()
        {
            println!("{}\t{}\t{}",x,y,( x^y));
        }
    }
```


# Problemas y soluciones al programar 
En Ptyhon existe e pude iterar en conjuntos de booleanos para realizar operaciones simples.
Pero en Rust se debe de acceder por referencia a la localidad de memoria para operar con ellos

```Rust
    println!("{}\t{}\t{}",x,y,(*x & *y));
```