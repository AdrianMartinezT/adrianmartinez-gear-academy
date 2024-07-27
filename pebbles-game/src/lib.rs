#![no_std]

use gstd::{};
use pebbles_game_io::GameState;
static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init(){
    //my code
}

#[no_mangle]
extern "C" fn handle(){
    //my code
}


#[no_mangle]
extern "C" fn state(){
    //my code
}

