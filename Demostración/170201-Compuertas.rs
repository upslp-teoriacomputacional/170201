/*---------------------------------------------------------
 *  Direct Proofs
 *---------------------------------------------------------*/

/* *****************************************************************************
 *  Name:    Juan Humberto Herrera MartÃ­nez
 *  NetID:   JuanH13
 *  Precept: P00
 *
 *  Description:  It exemplifies the way in which logical operations
 * 				  are used in the Rust programming language
 *
 *  Written:       9/18/2020
 *  Last updated:  9/18/2020
 *
 *  % Rut 170201-Compuertas.rs
 *  % Direct Proofs
 *  Disjunctive truth table
 *  Conjunction truth table
 *  Negation truth table
 *  Exclusion truth table
 *
 **************************************************************************** */

fn main()
{
	// We start by creating a list with the two possible Boolean values, False and True
    let mut booleanos: [bool; 2]= [false, true];
    
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
    println!("");

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
    println!("");
    
    
    //Negation truth table
    println!("x\tnot x");
    println!("-------------");
    for x in booleanos.iter()
    {
        println!("{}\t{}",x,!x);
    }
    
    println!("");
    
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
}