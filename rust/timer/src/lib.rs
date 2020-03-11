#![no_std]

#[allow(unused_extern_crates)]

use aux8::{rcc, tim6};

pub fn init(rcc: &rcc::RegisterBlock, tim6: &tim6::RegisterBlock, psc: u16, arr_val: u16) {
    // Power on TIM2 timer
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());

    // Power on the TIM6 timer
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    // OPM Select one pulse mode
    // CEN Keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    tim6.psc.write(|w| w.psc().bits(psc));
    tim6.arr.write(|w| w.arr().bits(arr_val));
    tim6.cr1.modify(|_, w| w.arpe().set_bit());
    tim6.cr1.modify(|_, w| w.cen().set_bit());
}

pub fn restart(tim6: &tim6::RegisterBlock) {
    tim6.sr.modify(|_, w| w.uif().clear_bit());
    tim6.cr1.modify(|_, w| w.cen().set_bit());
}

pub fn status_bit_is_set(tim6: &tim6::RegisterBlock) -> bool {
    tim6.sr.read().uif().bit_is_set()
}