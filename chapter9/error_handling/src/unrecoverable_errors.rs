// recoverable errors and unrecoverable errors
// Result<T,E> for recoverable
// panic! for unrecoverable

// unrecoverable errors : actions that causes the code to panic or by using the panic! macro
// we can use environment variables to track down the cause of panic
fn main() {
    // panic!("Crash and Burn");

    let v = vec![1,2,3];
    v[99];
}

// a backtrace is a list of functions that have been called to get to this point
// start reading from the top untill we see the files we wrote -> this line is the point from where the code started
// the lines above are the functions that our code called and below are the ones that called our code
// debug symbols have to be enabled to get the Backtrace
