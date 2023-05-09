
#[derive(Copy, Clone, Default, PartialEq)]
pub struct S_Entry{
	poss:[Option<usize>;9],
	pub options:usize,
	pub entry:usize,
}


impl S_Entry{
	pub fn new()-> Self{
		let poss = [Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8) ];
		let options=9;
		let entry=0;
		Self{
			poss, 
			options,
			entry,
		}
	}

	pub fn default() -> Self{
		S_Entry::new()
	}

	pub fn solved(&self ) -> bool{
		self.options ==1 
	}

	/// cooses one random entry of the still possible ones.
	pub fn possible( &self ) -> Vec<usize> {
		let mut ret =Vec::<usize>::with_capacity(self.options);
		for opt in &self.poss{
			match opt{
				Some(val) => {ret.push( *val);},
				None => ()
			}
		}
    	ret
	}

	pub fn equals( &self, val:usize ) -> bool{
		self.entry == val
	}

	pub fn solve(&mut self, val:usize ){
		//println!("Solving Entry with val {val}");
		for i in 0..9{
			if i == val {
				match self.poss[i]{
					Some(_p) => {
						//println!("Solve does KEEP {val}");
					},
					None => {
						let possible = self.possible();
						panic!("You tried to set an entry to an impossible value: {val} and I have only these available: {possible:?}");
					},
				};
			}else {
				match self.poss[i]{
					Some(_p) => {
						//println!("Solve removes the entry for {i} as we now 'own' {val}");
						self.poss[i] = std::option::Option::None
					},
					None => (),
				};
			}
			
		}
		//self.print_help();
		//println!("");
		self.options = 1;
		self.entry = val;
	}

	/// returns true if the entry has changed to a solved one due to the loss
	/// This requires to run a check on the souroundings, too
	pub fn loose_possibillity(&mut self, val:usize) -> bool {
		if self.solved(){
			return false
		} 
		let possible = self.possible();
		match self.poss[ val ] {
			Some( _d ) => {
				//println!("Removing {val} from my possibles? {possible:?}");
				self.poss[ val ] = std::option::Option::None;
				self.options -= 1;
			}
			None => (),
		}
		if self.options == 1{
			//now I need to finalize this here!
			for opt in self.poss{
				match opt{
					Some(val) => self.entry = val,
					None => (),
				};
			}
			//println!("This entry has reached state 'solved' while removing value {val} and {possible:?}");
			return true
		}
		return false
	}

	pub fn can_not_loose_possibillity( &self, val:usize) -> bool{
		if self.options == 1 && self.entry == val{
			return true;
		}
		let mut total = self.options;
		match self.poss[ val ] {
			Some( _d ) => {
				//println!("can I not loose {val} from my options? {}", self.options);
				total -= 1;
			}
			None => (),
		}
		total == 0
	}

	pub fn can_loose_possibillity( &self, val:usize) -> bool{
		let mut total = self.options;
		match self.poss[ val ] {
			Some( d ) => {
				total-= 1;
			}
			None => (),
		}
		total > 0
	}

	pub fn print_help(&self ){
		if self.options != 1{
			print!(" ");
			for opt in self.poss {
				match opt{
					Some(val) => print!("{}",val+1),
					None => print!("-"),
				};
			}
			print!(" ");
		}else{
			print!("     {}     ", self.entry+1 );
		}
	}

	pub fn print(&self ){
		if self.options != 1{
			print!(" 0 ");
		}else{
			print!(" {} ", self.entry+1 );
		}
	}


}


#[cfg(test)]
mod tests {

    use crate::s_entry::S_Entry;

     #[test]
    fn check_is_valid() {

    	let mut data = S_Entry::new();

    	assert_eq!( data.options, 9 );
    	assert_eq!( data.possible(), vec![0,1,2,3,4,5,6,7,8] );

    	let tmp = data.clone();

    	assert_eq!(data.can_loose_possibillity(5), true);
    	assert_eq!( data.possible(), vec![0,1,2,3,4,6,7,8] );
    	assert_eq!( tmp.possible(), vec![0,1,2,3,4,5,6,7,8] );

    	data.solve(4);
    	assert_eq!( data.possible(), vec![4] );
    	assert_eq!( data.options, 1 );

    }

}