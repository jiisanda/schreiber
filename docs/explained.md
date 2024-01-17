### Explaining the code flow of the application

I. Commit: __raw terminal__
```
use std::io::{self, Read};
```

Here, self, is including `io`, and `Read` enables us to use `bytes()`, and this is called traits. 
The `for` statement, says "For every byte you can read from the keyboard, bind it to `b` and execute the following block.

When we run the code, we enter into a canonical mode(or cooked mode). To terminate the session we will use `Ctrl+C` or `Ctrl+D`, or use `q`.

In here, what we need is the raw mode of the terminal, in raw mode, characters are directly read from and written to the device without any translation or interpretation by the operating system. On input characters are available as soon as they are typed.
And for entering this raw mode we have added a crate dependency to `Cargo.toml`, `termion = "2"`.

To run the program from here, we will need a Linux Subsystem in Windows. 

So, here we are initializing the variable `_stdout`, but not using it, as `into_raw_mode` is going to modify the terminal and return a value which, once removed, will reset the terminal into canonical mode, this concept in rust is called **Ownership System**.


In the updated for loop...
```
    for b in io::stdin().bytes() {

        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
```

Here, we are re-initializing `b` one with `for b ...` and other on `let  b = ...`, this is called **Variable Shadowing**. 
`is_control()`, checks whether the character entered is a control character or not, control characters are the characters which are printable. In the `println!` statement, we use `{:?}`, which means printing the *debug string representation*. And the `\r` is a *Carriage Return*, this makes sure the output is neatly printed line by line without indentation. It's different from `\n`.



II. Commit: 

In function `refresh_screen()`, we have `\x1b[1J` here:
- `\x1b`: is the escape character, or `27` in decimal
The next 3 are called *escape sequence*, 
- `[`, this instructs the terminal to do various text formatting tasks, such as coloring text, moving the cursor around, and clearing part of the screen.
- `J` is the command the *Erase In Display* i.e., to clear the screen.
- `\x1b[1J` - would clear the screen up to where the cursor is
- `\x1b[0J` - would clear the screen from the cursor up to the end of the screen [Default]
- `\x1b[2J` - would clear the entire screen

```
fn refresh_screen(&self) -> Result<(), std::io::Error> {
    // print!("\x1b[2J");
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    io::stdout().flush()
}
```

Here, `termion::clear::All`, clears the terminal, and `termion::cursor::Goto(1, 1)`, brings the cursor to the head of the terminal. `termion::cursor::Goto(12, 40)` will bring the cursor to the center of the terminal.
