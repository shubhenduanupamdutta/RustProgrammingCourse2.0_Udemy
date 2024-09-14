//----------------------------------------------------------------
/*
Consider the code below. Write the expanded version of the code that can be viewed using the cargo expand utility.

macro_rules! sum_macro {

    ($($x:expr),*) => {

        {

            let mut sum = 0;

            $(sum += $x;)*

            sum

        }

    };

}



fn main() {

    let result = sum_macro!(1, 2, 3, 4, 5);

}
 */
//----------------------------------------------------------------

macro_rules! sum_macro {

    ($($x:expr),*) => {

        {

            let mut sum = 0;

            $(sum += $x;)*

            sum

        }

    };

}

pub fn main() {
    let result = sum_macro!(1, 2, 3, 4, 5);
    println!("Sum: {}", result);
}
