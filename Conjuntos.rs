use std::collections::HashSet;

fn main()
{
	let a: HashSet<_> = [1, 2, 3,3].iter().collect();
	let b: HashSet<_> = [5,9,7,1].iter().collect();

	print!("A: [");
	for x in a.iter() {
	    print!("{},", x);
	}
	println!("]");
	print!("B: [");
	for x in b.iter() {
	    print!("{},", x);
	}
	println!("]");
	print!("AuB: [");
	for x in a.union(&b) {
	    print!("{},", x);
	}
	println!("]");
	print!("AnB: [");
	for x in a.intersection(&b) {
	    print!("{},", x);
	}
	println!("]");

}
