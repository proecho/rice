use rand::Rng;
use std::collections::HashMap;

fn main() {
	let mut a = 0;
	for n in 1..100001{
		let value = iteration();
		//println!("{}", value);
		if value == true{
			a += 1;
		}
	}
	println!("{}", (a as f32)/100000.0);

}

fn a_list_generator() -> Vec<(u32,u32)> {
	let mut rice_list: Vec<(u32,u32)> = Vec::new();
    for n in 1..101 {
        let mut rng = rand::thread_rng();
		let a = n;
		let b = rng.gen_range(1, 401);
	    rice_list.push((a,b));
	}
	//println!("{:?}",rice_list);
	rice_list   
}

fn iteration() -> bool {
	let rice_list = a_list_generator();
	let mut rice_counter: HashMap<u32,u32> = HashMap::new();
	for (a,b) in rice_list {
	    match rice_counter.get_mut(&b){
			Some(n) => {
				*n = (*n) +1;
			},
			None =>{
				 rice_counter.insert(b,1);
			},
		}
	}
	
	for (k,v) in rice_counter.iter(){
		if *v == 5 {
			//println!("true");
			return true;
	     }
	}
	false
}
