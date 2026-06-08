extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    raw();
    noecho();

    addstr("Hello, world!").unwrap();
    refresh();

    // Infinite loop
    getch();

    endwin();
}


