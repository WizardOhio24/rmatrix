# rmatrix
A terminal program which shows random letters flowing down it, just like in the screens in the matrix.  The default is green letters flowing down the screen displaying a-z,A-Z, but can be customised.

Arguments:

  - -c \<COLOR\> [green, blue, red, magenta, white, black, yellow] - letter color.

  - -s \<SPEED\> [1.0] - multiple of speed.

  - -l \<LENGTH\> [16] - the length of the letter chains.

  - \<LETTERS\> [abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ] - the letters to use in the letter chains.

For example `rmatrix -c white -s 1.5 -l 5 ,.` gives a rain-like effect.

Press 'q' to exit.

For similar projects, see [cmatrix](https://github.com/abishekvashok/cmatrix) and [unimatrix](https://github.com/will8211/unimatrix).

##### Technologies:
Rust & Termion

##### Screenshot:
![rmatrix_screenshot](./demo_screenshot.png "RMatrix")
