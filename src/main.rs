// This function takes no arguments and returns two u32 numbers
// u32 is a 32-bit unsigned integer
fn find_numbers_with_3_digits_brute_force()->(u32,u32){
	// Keep track of how many numbers are found
	let mut found=0;
	// Keep track of how many numbers are tested
	let mut tested=0;

	// Loop from 100 to 999 (the .. range is non-inclusive)
	for test_number in 100_u32..1000{

		// Count this as one number tested
		tested+=1;

		let mut sum=0;
		// Convert number to decimal string
		let digits=test_number.to_string();

		// Loop over digit characters
		for digit_ascii in digits.chars(){

			// Parse each ascii character back into u32 from a base 10 number and ignore any errors
			let digit:u32=digit_ascii.to_digit(10).unwrap();

			// Square it and add it to the sum
			sum+=digit.pow(2);
		}

		// Check if the number is divisible by 11
		if test_number%11==0{

			// Check if the sum is equal to the number divided by 11
			if test_number/11==sum{
				println!("FOUND {test_number}");
				// Count this as one number found
				found+=1;
			}
		}
	}

	// Return the counts
	(found,tested)
}

fn find_numbers_with_n_digits(n:u32)->(u32,u32){
	// Count how many total numbers we tested
	let mut found=0;
	let mut tested=0;

	// Let's loop over all multiples of 11 with N digits and test them.
	// When N=3, loop from 100/11 up to 1000/11
	// and then multiply each number by 11
	// giving only multiples of 11 and skipping all other test numbers

	// lowest n digit number divided by 11
	let lowest=10_u32.pow(n-1)/11;

	// highest n digit number divided by 11
	let highest=10_u32.pow(n)/11;

	// Loop from lowest to highest
	for number_11 in lowest..highest{

		// This is the test number
		let test_number=number_11*11;

		// Add one to the test_number count
		tested+=1;

		// Add up the square of each digit of the N digit number
		let mut sum=0;
		for digit in 0..n{

			// This is the value of `digit` of test_number
			// For example when digit = 2, the calculation is
			// value=(num/100)%10
			// For example:
			// num = 6512
			// num/100 = 65
			// 65 % 10 = 5
			// Successfully extracting the digit 5
			let value=(test_number/10_u32.pow(digit))%10;

			// Square it and add to sum
			sum+=value.pow(2);
		}

		// Print out the numbers that satisfy the condition
		// test_number/11 == sum
		if number_11==sum{
			println!("FOUND {test_number}");
			found+=1;
		}
	}

	// Return the counts
	(found,tested)
}

// This is where the program starts execution
fn main() {

	// Run the brute force algorithm
	println!("==Running algorithm 1 (brute force)==");

	// write down the starting time
	let timer1=std::time::Instant::now();

	// brute force all 3 digit numbers with strings
	let (found,tested)=find_numbers_with_3_digits_brute_force();

	// write down the elapsed time
	let time1=timer1.elapsed();

	// How many total numbers were tested?
	println!("{found} found / {tested} tested");

	// How long did it take?
	println!("time={time1:?}");


	// Run the more efficient algorithm
	println!("==Running algorithm 2 (efficient)==");

	// write down the starting time
	let timer2=std::time::Instant::now();

	// when digits = 5
	// the maximum sum is 5*9^2 = 405, but
	// the minimum value of N/11 is 10000/11 = 909
	// So no more numbers are possible

	let mut total_found=0;
	let mut total_tested=0;
	for n in 1..5{
		let (found,tested)=find_numbers_with_n_digits(n);
		total_found+=found;
		total_tested+=tested;
	}

	// write down the elapsed time
	let time2=timer2.elapsed();

	// How many total numbers were tested?
	println!("{total_found} found / {total_tested} tested");

	// How long did it take?
	println!("time={time2:?}");
}
