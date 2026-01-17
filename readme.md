### Guessing Game in Rust – Explanation and Usage

A simple number-guessing game implemented in Rust. The program generates a random secret number between 1 and 100 and repeatedly prompts the user to guess it until the correct number is entered. The code demonstrates core Rust concepts: importing libraries, generating random numbers, looping, reading input, error handling, parsing, and matching. Below, each part of the code is broken down with detailed explanations and references to the Rust documentation and book.

**Imports and Dependencies**

The program starts by importing necessary items:
```
use std::io;
use std::cmp::Ordering;
use rand::Rng;
```

`std::io:` Brings in the standard input/output library. It provides types and methods (like `io::stdin().read_line(...)`) to read user input and write output.

`std::cmp::Ordering`: Imports the Ordering enum (`Less, Equal, Greater`) from the standard library. This enum represents the result of comparing two values. It’s used to handle the comparison of the guess and the secret number.

`rand::Rng`: Imports the `Rng` trait from the rand crate. The Rng trait defines random-number generation methods. Bringing it into scope is required to use methods like .gen_range() on a random number generator. (We assume the rand crate is included in `Cargo.toml` as a dependency.)


**Key points:**

- You need rand = "x.y" in Cargo.toml to use rand::thread_rng() and related methods.
- The Rng trait gives access to methods like gen_range.
- The Ordering enum (from std::cmp) has variants Less, Equal, and Greater used for comparisons.

**Main Function and Variable Initialization**

The entry point is the main function:
```
fn main() {
    // Generate secret number
    let random = rand::rng().random_range(1..=100);
    // Initialize guess counter
    let mut count: usize = 0;
    // ...
}
```

**Random Number Generation:** Calling `rand::rng().random_range(1..=100)` produces a random integer in the inclusive range 1 to 100. Here, `rng()` returns a thread-local random generator, and `random_range(1..=100)` uses an inclusive range (start..=end) as documented. This means all values from 1 up to 100 (inclusive) can be generated.

**Inclusive Range Syntax:** The syntax 1..=100 is inclusive of both ends. In Rust, start..=end means include both bounds, so 1..=100 yields any integer 1 through 100.

**Counter (count):** We declare `let mut count: usize = 0;` to keep track of how many guesses the user has made. It is a mutable unsigned integer (usize). This counter is incremented on each loop iteration (each guess). Using usize is a common choice for counters or indexing in Rust.

**Key points:**
- A secret number is generated once before the loop.
- The random range is inclusive thanks to ..= notation.
- count is mutable (mut) and starts at 0 to count guesses.

**The Guess Loop**

The code then enters an infinite loop to allow multiple guesses:
```
loop {
    // Prompt, read, and process each guess
}
```

The `loop { ... }` construct creates an infinite loop in Rust. It will keep running until a break statement is encountered. This loop allows the program to repeatedly prompt the user for guesses.
Inside the loop, the program will read input, parse it, compare it to the secret number, and give feedback. On a correct guess, break will exit the loop. On invalid input, continue will skip to the next iteration.

**Key points:**

- Use `loop { ... }` for an endless loop in Rust.
- `break` is used to exit the loop entirely (when the user guesses correctly).
- `continue` skips the rest of the current iteration and goes back to the start of the loop (used if input parsing fails).

**Reading the User’s Guess**

Inside the loop, the program reads a line of input from the user:
```
let mut guess = String::new();
count += 1;
println!("[Guess {}] Enter your guess:", count);

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read the line");
```

**Creating an empty String:** `String::new()` creates a new, empty heap-allocated string. The variable guess is declared as mutable (mut) so we can fill it with the user’s input.

**Increment counter:** `count += 1;` increases the guess count each time we prompt the user.

**Prompting the user:** `println!("[Guess {}] Enter your guess:", count);` displays the current guess number and asks for input.

**Reading input:** `io::stdin().read_line(&mut guess)` reads a line from standard input into the guess string buffer. The `&mut guess` argument is a *mutable reference : borrowing* to the string. This method returns a `Result`, and calling `.expect(...)` unwraps it, panicking if there is an error (unlikely under normal circumstances). In effect, the program waits for the user to type a line and press Enter. After this call, guess contains the user’s input plus the newline character.

**Key points:**

- `read_line(&mut guess)` fills guess with the input. It blocks until Enter is pressed.
- We handle any I/O error with `.expect(...)`, which will terminate the program if reading fails.
- The input in guess will include a trailing newline (e.g. "5\n"), which is handled next.

**Trimming and Parsing the Input**

After reading the line, the code attempts to convert the input string to a number:
```
let guess: i32 = match guess.trim().parse() {
    Ok(number) => number,
    Err(_) => continue,
};
```

**guess.trim():** Removes whitespace at both ends of the string, including the newline. If the user input was *"5\n", trim() produces "5"*. We must trim the newline before parsing, or else parsing will fail. This is explained in the Rust book: trimming removes "\n" or "\r\n", leaving just the numeric characters.

**Parsing to i32:** Calling `.parse()` attempts to convert the trimmed string into a number. We declare the type explicitly (let guess: i32) so Rust knows we want an i32. If the string is something like "5", parse() returns Ok(5).

**Handling the Result:** `parse()` returns a `Result<i32, _>`, because parsing can fail if the input isn’t a valid number. We use a match expression:

`Ok(number) => number`: If parsing succeeds, extract the number.

`Err(_) => continue`: If parsing fails (e.g., the user entered "foo"), we ignore the error and use continue to start the next loop iteration. This means invalid input is simply skipped without crashing the program.

**This code also uses variable shadowing:** the original guess variable (a String) is shadowed by a new guess variable of type i32. Rust allows this, so after parsing, guess refers to the integer value, not the string. Shadowing is a common technique when converting values.

**Key points:**

- `trim().parse()` removes whitespace then parses. The `.parse()` method on a string yields a `Result`, because conversion can fail.
- Using match on the `Result`: on `Err`, we continue the loop, effectively re-prompting the user. On `Ok`, we get the integer.
- Shadowing allows reusing the name guess for the parsed number, avoiding a second variable name.

**Comparing the Guess to the Secret Number**

With the user’s guess as an integer, the code compares it to the secret:
```
match guess.cmp(&random) {
    Ordering::Less => println!("Your guess is too low."),
    Ordering::Greater => println!("Your guess is too high."),
    Ordering::Equal => {
        println!("You guessed it right in {} guesses! Congratulations!", count);
        break;
    }
}
```

**guess.cmp(&secret_number):** The `cmp` method (from the Ord trait) compares two integers and returns an `Ordering`. It yields Ordering::Less if guess < secret_number, Greater if guess > secret_number, or Equal if they are the same. For example, 1.cmp(&2) is Less, and 2.cmp(&1) is Greater.

**Handling each case:** We use a match on the Ordering result:

`Ordering::Less` means the guess is smaller than the secret; we print a hint that it’s too low.
`Ordering::Greater` means the guess is too high; we hint it’s too big.
`Ordering::Equal` means the guess is correct. In that case, we congratulate the user (showing the guess count) and execute break;.

**Breaking the loop:** The break statement exits the loop. Once we break out, the main function will end, terminating the program.

**Key points:**

- Use `cmp` to compare numbers, which returns `Ordering`.
- The match expression handles all cases (`Less, Greater, Equal`). On Equal, the program announces success and breaks the loop.

**Final Output and Ending**

After each guess (whether too low or too high), the code often prints a separator line for clarity:
```
println!("____________________________________________");
```

This line is just for formatting in the console; it doesn’t affect logic. The loop then repeats, prompting for the next guess.

When the correct guess is entered and break is called inside the `Ordering::Equal` arm, the loop ends. The program then reaches the end of main and exits.

**Summary:** Each loop iteration does the following: increment the guess counter, prompt and read input into a String, trim/parse it into an i32 (retrying on error), compare it with the secret number, and give feedback. The code demonstrates Rust concepts like standard I/O, random numbers, loops, pattern matching on Result and Ordering, shadowing variables, and error handling. References for these concepts include the Rust book and official docs.

**Usage**

To use this guessing game code:

1. Install Rust: Make sure you have the Rust toolchain installed (via rustup).
2. Create a new project or file: You can create a new Cargo project:
```
cargo new guessing_game
cd guessing_game
```
Then place the game code in src/main.rs.
3. Add the rand dependency: In Cargo.toml, include:
```
[dependencies]
rand = "0.9.2"
```
(Adjust the version as needed.) Run cargo build to fetch dependencies.
4. Compile and run: Use Cargo to build and run:
```
cargo run
```
You should see a prompt like:
```
Guess the number!
[Guess 1] Enter your guess:
```
Enter a number and press Enter. The program will tell you whether your guess is too low or too high, and continue until you guess correctly. It will then print a congratulations message with the number of guesses and exit.
5. Exiting: Besides guessing correctly, you can interrupt the game with Ctrl-C. Invalid input (like typing a non-number) will be ignored (the game will simply ask again).

Enjoy experimenting with the code and learning how each part works! The above explanations and examples should clarify the Rust syntax and concepts used in this simple guessing game.

**References:** Concepts and examples are based on the Rust standard library and The Rust Programming Language book (official Rust tutorial).

### Using this code
1) For using my code, you can open your terminal
2) Clone the repository
```
git clone https://github.com/rdbhvr-001/rust-guessing-game.git; cd rust-guessing-game
```
3) Build the game and run
```
cargo build --release; ./target/release/guessing_game
```
4) Or alternatively run the code
```
cargo run
```
5) You can also make a copy of the binary to your ~/.local/bin
```
cp -r ./target/release/guessing_game ~/.local/bin/
```
6) You are ready to play the game. Congratulations! Try minimising the number of guesses
```
guessing_game
```

