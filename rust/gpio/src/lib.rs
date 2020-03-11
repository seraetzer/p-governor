#![no_std]

#[allow(unused_extern_crates)]

use aux8::{gpioa, gpioc, rcc};

pub fn init(rcc: &rcc::RegisterBlock, gpioa: &gpioa::RegisterBlock, gpioe: &gpioc::RegisterBlock) {
    // initialize GPIOA
    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit()); // Comp Out an PA1 channel 2
    gpioa.moder.modify(|_, w| {
        w.moder0().input(); // change mode of PA pin0 to input
        w.moder1().alternate(); // change mode of PA pin1 to alternate function
        w.moder2().input() // change mode of PA pin2 to input
    });
    unsafe {
        gpioa.afrl.write(|w| w.afrl1().bits(1)); // alternate function on port A pin 1
    };
    // Initialize PE
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());

    // configure led pins as output
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });
}

pub fn read_pin0(gpioa: &gpioa::RegisterBlock) -> bool {
    gpioa.idr.read().idr0().bit_is_set()
}

pub fn read_pin2(gpioa: &gpioa::RegisterBlock) -> bool {
    gpioa.idr.read().idr2().bit_is_set()
}