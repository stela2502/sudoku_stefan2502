use rand::Rng;
use rand::prelude::SliceRandom;
use crate::s_entry::S_Entry;
use rand::rngs::ThreadRng;

/// the data is stored as a [[S_Entry;9];9]
pub struct Sudoku{
	pub board: [[S_Entry;9];9],
	start_time : Option<std::time::SystemTime>,
	rand:ThreadRng,
	pub solved:usize,
}

impl Sudoku{
	pub fn new()->Self{
		let board = [[S_Entry::default(); 9]; 9];
		let start_time = std::option::Option::None;
		let solved = 0;
		let rand = rand::thread_rng();

		Self {
			board,
			start_time,
			rand,
			solved,
		}
	}

	pub fn clone(&self) -> Self{
		let mut ret = Sudoku::new();
		for r in 0..9{
			for c in 0..9{
				ret.board[r][c] = self.board[r][c].clone();
			}
		}
		ret
	}

	pub fn reset(&mut self ) {
		for r in 0..9{
			for c in 0..9{
				self.board[r][c] = S_Entry::default();
			}
		}
		self.solved = 0;
	}

	pub fn reset_to(&mut self, other:Sudoku ) {
		for r in 0..9{
			for c in 0..9{
				self.board[r][c] = other.board[r][c];
			}
		}
		self.solved = 0;
	}

	pub fn is_incomplete( &self ) -> bool{
		for r in 0..9{
			for c in 0..9{
				if ! self.board[r][c].solved() {
					return true
				}
			}
		}
		return false
	}

	pub fn unsolved( &self ) -> usize{
		let mut ret = 0;
		for r in 0..9{
			for c in 0..9{
				if ! self.board[r][c].solved() {
					ret +=1;
				}
			}
		}
		ret
	}

	pub fn fill_any( &mut self, row:usize, col:usize) -> bool{
    	let mut numbers = self.board[row][col].possible();
    	numbers.shuffle( &mut self.rand);
    	//println!("I try to fill the {row} {col} S_Entry with any of them: {numbers:?}");
		for val in numbers {
			match self.do_solvable_change( row, col, val ){
				Some(true) => return true,
				Some(false) => (),
				None => return false
			}
		};
		return false;
	}

	pub fn do_solvable_change( &mut self, row:usize, col:usize, val:usize ) -> Option<bool> {
		let tmp = self.clone();

		self.board[row][col].solve(val);
		/*print!("I have set the S_Entry to ");
		self.board[row][col].print_help();
		print!("\n");*/
		match self.check_for_position( row, col, val ){
			Some(true) => return Some(true),
			Some(false) => {
				self.reset_to( tmp );
				return Some(false)
			},
			None => return None,
		};
	}


	/// here you need to save the Sudoku using the clone() function and reset_to the 
	/// cloned if this fails!
	/// This function resurns Some(true) if no problems were encountered.
	/// false if any entry would fail if this value would be taken away.
	/// And None is an error has occured in a dowstream event
	pub fn check_for_position( &mut self, row:usize, col:usize, val:usize ) -> Option<bool>{

		for i in 0..9{
			if i != col{
				if self.board[row][i].can_not_loose_possibillity(val){
					//println!("the value row {} and col {} can not be set to {} because of {};{}?!",row+1, col+1, val+1, row+1, i+1);
					//self.print();
					//self.print_help();
					return Some(false)
				}
				else {
					if self.board[row][i].loose_possibillity( val ){
						//println!("I have a self-resolved value on position for row {}{}", row +1, i +1);
						//self.print();
						//self.print_help();
						match self.check_for_position( row , i , self.board[row][i].entry ){
							Some(true) => (),
							Some(false) => return None,
							None => return None,
						}
						//self.print();
						//self.print_help();
					}
				}
			}
			if i != row{
				if self.board[i][col].can_not_loose_possibillity(val){
					//println!("the value row {} and col {} can not be set to {} beause of {};{}?!",row+1, col+1, val+1, i+1, col+1);
					//self.print();
					//self.print_help();
					return Some(false)
				}
				else {
					if self.board[i][col].loose_possibillity( val ){
						//println!("I have a self-resolved value on position for column {}{}", i +1,  col+1);
						//self.print();
						//self.print_help();
						match self.check_for_position( i , col , self.board[i][col].entry ){
							Some(true) => (),
							Some(false) => return None,
							None => return None,
						}
					}
				}
			}
		}
		let st_c = col / 3 * 3;
		let st_r = row / 3 * 3;
		//println!("For this entry we check the box {st_r}{st_r}");
		for i in 0..3{
			'inner: for j in 0..3{
				if (i + st_r  == row) || (j + st_c == col){
					//println!("but not for {} and {}", i+1, j+1);
					continue 'inner; // already processed
				}
				if self.board[i + st_r][j + st_c ].can_not_loose_possibillity( val ){
					//let possible = self.board[i + st_r][j + st_c ].possible();
					//println!("the value row {} and col {} can not be set to {} because of {},{} only has these options {possible:?} + 1",row+1,col+1, val+1, i + st_r +1, i + st_c +1);
					//self.print();
					//self.print_help();
					return Some(false)
				}else {
					//println!("loose possibilty at {},{} for {}", i + st_r+1, j + st_c+ 1, val +1);
					if self.board[i + st_r][j + st_c ].loose_possibillity( val ){
						//println!("I have a self-resolved value on position for the box {} {}", i + st_r+1, j + st_c+1);
						//self.print();
						//self.print_help();
						// shit - a check for this solved value is necessary
						match self.check_for_position( i + st_r, j + st_c, self.board[i + st_r][j + st_c ].entry ){
							Some(true) => (),
							Some(false) => return None,
							None => return None,
						};
					}
				}
			}
		}
		return Some(true)
	}

	pub fn rand_init(&mut self, n:usize ) -> bool {
		let mut r_r = self.rand.gen_range(0..9);
		let mut r_c = self.rand.gen_range(0..9);
		for _i in 0..n {
			while self.board[r_r][r_c].solved() {
				r_r = self.rand.gen_range(0..9);
				r_c = self.rand.gen_range(0..9);
			}
			if ! self.fill_any( r_r, r_c ){
				println!("I can not fill any value in - failure?!");
				return false
			}else {
				//println!("Changed row {} and col {} to val {}", r_r +1, r_c +1 , self.board[r_r][r_c].entry+1);
			}
		}
		return true;
	}

	pub fn print( &self ){
		// print a row of --- (3*9 dashes)
		let dashes = std::iter::repeat("---").take(9).collect::<String>();
		for i in 0..9{
			if i %3 == 0{
				print!("----{dashes}\n");
			}
			for j in 0..9{
				if j %3 ==0 {
					print!("|");
				}
				 self.board[i][j].print();
			}
			print!("|\n");
		}
		print!("----{dashes}\n");
	}

	pub fn print_help(&self){
		let dashes = std::iter::repeat("-----------").take(9).collect::<String>();
		for i in 0..9{
			if i %3 == 0{
				print!("----{dashes}\n");
			}
			for j in 0..9{
				if j %3 ==0 {
					print!("|");
				}
				self.board[i][j].print_help();
			}
			print!("|\n");
		}
		print!("----{dashes}\n");
	}

	fn is_valid( &self, row:usize, col:usize, val:usize ) -> bool{
		for i in 0..9{
			if self.board[row][i].equals(val) || self.board[i][col].equals(val){
				return false
			}
		}
		let st_c = col / 3 * 3;
		let st_r = row / 3 * 3;
		for i in 0..3{
			for j in 0..3{
				if self.board[i + st_r][j + st_c ].equals( val ){
					return false
				}
			}
		}
		return true
	}

	pub fn solve_solutions( &mut self, solved:&mut usize ){
		match self.start_time{
			Some(time) => {
				if time.elapsed().unwrap().as_millis() > 1000 * 60 * 10 { // 10 min
					return
				}
			},
			None => self.start_time = Some(std::time::SystemTime::now()),
		};

		let mut min = usize::MAX;
		let mut min_r=10;
		let mut min_c=10;

		for r in 0..9{
			for c in 0..9{
				if !self.board[r][c].solved(){
					if ! self.fill_any( r, c ){
						if self.is_incomplete() {
							*solved +=1;
						}
					}
				}
			}
		}
		// if min_r == 10 || min_c ==10 {
		// 	panic!("There is no minimal value in the table?!")
		// }

		// let possible = self.board[min_r][min_c].possible();
		// let mut tmp:Sudoku;
		// for opt in possible{
		// 	tmp = self.clone();
		// 	if self.do_solvable_change(min_r, min_c, opt ){
		// 		tmp.solve_solutions( solved );
		// 	}
		// }

		if self.is_incomplete(){
			*solved +=1;
		}
	}

	pub fn solve( &mut self ) -> bool{
		match self.start_time{
			Some(time) => {
				if time.elapsed().unwrap().as_millis() > 1000 * 60 * 10 { // 10 min
					return false;
				}
			},
			None => self.start_time = Some(std::time::SystemTime::now()),
		};

		let mut min = usize::MAX;
		let mut min_r = 10;
		let mut min_c = 10;

		let mut tmp = self.clone();

		for r in 0..9{
			for c in 0..9{
				if ! self.board[r][c].solved() {
					tmp = self.clone();
					//if self.fill_any(r, c){
					// 	println!("Solve changed a value! {},{} to {}", r+1, c+1, self.board[r][c].entry+1);
					// 	self.print_help()
					// }
					// else{
					// 	println!("failed a change! {},{} to {}", r+1, c+1, self.board[r][c].entry+1);
					if ! self.fill_any(r, c){
						self.reset_to(tmp);
					}
				}
			}
		}

		// if min_r == 10 || min_c ==10 {
		// 	panic!("There is no minimal value in the table?!")
		// }

		// let possible = self.board[min_r][min_c].possible();
		// let tmp = self.clone();

		// println!("I try to solve the value {min_r} and {min_c} which has the options {possible:?}", );

		// for opt in possible{
		// 	if self.do_solvable_change(min_r, min_c, opt ){
		// 		if ! self.solve() {
		// 			self.reset_to( tmp.clone() );
		// 		}else {
		// 			return true
		// 		}
		// 	}
		// }
		return ! self.is_incomplete()
	}

	fn sum( &self ) -> usize {
		let mut ret = 0;
		for r in 0..9{
			for c in 0..9{
				ret += self.board[r][c].entry;
			}
		}
		ret
	}
	/// remove n values from a solved sudoku
	/// making sure that only one solution is valid at a time.
	pub fn purge(&mut self, n:usize ) -> bool{
		let mut rng = rand::thread_rng();
		let mut numbers = (1..(9*9)).collect::<Vec<usize>>();
    	numbers.shuffle(&mut rng);
    	let mut row:usize;
    	let mut col:usize;
    	let mut total = 0;
    	let mut tries = 0;
    	let mut tmp:S_Entry;
    	let mut temporary:Sudoku;
    	let mut restarts = 0;
    	'outer: while total < n {
    		'inner_loop: loop {

    			if total+tries == numbers.len(){
    				if self.unsolved() >= n {
    					return true
    				}
    				restarts += 1;
    				if restarts == 3{
    					println!("Total of {} unsolved entries of {} wanted", self.unsolved(), n);
    					return false;
    				}
    				total = 0;
    				tries = 0;
    			}

    			row = numbers[total+tries] / 9;
    			col = numbers[total+tries] % 9;
    			tmp = self.board[row][col];
    			self.board[row][col] = S_Entry::new();
    			let mut solved = 0;
    			temporary = self.clone(); 
    			temporary.solve_solutions( &mut solved);
    			//println!("removal of the {total}+1th entry I have {} solutions", temporary.solved );
    			if solved == 0 {
    				// println!( "I managed to remove enry {row},{col}:");
    				self.solved = solved;
    				//self.print();
    				total +=1;
    				break 'inner_loop;
    			}
    			//println!("I failed to purge entry {row} {col} {} ending at {total} of {n} iterations", solved );
    			self.board[row][col] = tmp;
    			tries += 1;
    		}
    	}
    	println!("I ran through {total} purge iterations {}", self.solved);
    	return true
	}

	pub fn new_entry_at(&mut self, row:usize, col:usize ){
		let mut ret = S_Entry::new();
		for i in 0..9{
			if self.board[row][i].solved() {
				if i == col{
					continue;
				}
				if ret.can_not_loose_possibillity( self.board[row][i].entry ){
					panic!("I can not find a possible entry at {row} {col} [{i}] - no possibilites left");
				}else{
					if ret.loose_possibillity( self.board[row][i].entry ){
						// this does not matter here as we call this function from within the purge
						// everything should be correct in there!
					}
				}
			}
			if self.board[i][col].solved() {
				if i == row{
					continue;
				}
				if ret.can_not_loose_possibillity( self.board[i][col].entry ){
					panic!("I can not find a possible entry at {row} [{i}] {col} - no possibilites left");
				}else{
					if ret.loose_possibillity( self.board[row][i].entry ){
						// this does not matter here as we call this function from within the purge
						// everything should be correct in there!
					}
				}
			}
		}
		let st_c = col / 3 * 3;
		let st_r = row / 3 * 3;
		for i in 0..3{
			'inner: for j in 0..3{
				if row == st_r + i || col == st_c +j {
					continue 'inner;
				}
				if self.board[i + st_r][j + st_c ].solved(){
					continue;
				}
				if ret.can_not_loose_possibillity( self.board[i + st_r][j + st_c ].entry ){
					panic!("I can not find a possible entry at {row} [{}] {col}[{}] - no possibilites left", i + st_r,j + st_c );
				}else{
					if ret.loose_possibillity( self.board[row][i].entry ){
						// this does not matter here as we call this function from within the purge
						// everything should be correct in there!
					}
				}
			}
		}
		self.board[row][col] = ret;
	}

	pub fn reset_help( &mut self ){
		for row in 0..9{
			for col in 0..9{
				if ! self.board[row][col].solved() {
					for i in 0..9{
						if i != row {
							if self.board[i][col].solved(){
								if self.board[col][col].loose_possibillity( self.board[i][col].entry ){
								}
							}
							
						}
						if i != col {
							if self.board[row][i].solved(){
								if self.board[row][col].loose_possibillity( self.board[row][i].entry ){
								}
							}
						}
					}
					let st_c = col / 3 * 3;
					let st_r = row / 3 * 3;
					for i in 0..3{
						'inner: for j in 0..3{
							if (i + st_r  == row) || (j + st_c == col){
								//println!("but not for {} and {}", i+1, j+1);
								continue 'inner; // already processed
							}
							if self.board[row][col].loose_possibillity( self.board[i + st_r][j + st_c ].entry  ){
							}
						}
					}
				}
			}
		}
	}
}


#[cfg(test)]
mod tests {

    use crate::sudoku::Sudoku;

     #[test]
    fn check_is_valid() {

    	let mut data = Sudoku::new();

    	assert_eq!( data.sum(), 0 );

    	assert_eq!( data.board[0][0].options, 9 );

    	data.board[0][0].solve(4);

    	assert_eq!( data.sum(), 4 );

    	let tmp = data.clone();

    	assert_eq!( data.is_valid( 1,0, 4), false );

    	data.reset();

    	assert_eq!( data.sum() , 0 );

    	assert_eq!( tmp.sum() , 4 );
    }

    fn check_rand_init() {

    	let mut data = Sudoku::new();

    	assert_eq!( data.sum(), 0 );

    	data.rand_init(81);

    	assert_eq!( data.sum(), 1+2+3+4+5+6+7+8+9 *9 );
    	//panic!("This has not run -or?");
    }

}