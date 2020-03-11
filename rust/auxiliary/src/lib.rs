//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::DWT};
pub use cortex_m_rt::entry;
pub use f3::hal::stm32f30x::{gpioc, rcc, tim2, gpioa, tim6};

use f3::hal::stm32f30x::{self, GPIOE, RCC, TIM2, GPIOA, TIM6};

pub fn init() -> (&'static gpioc::RegisterBlock,
                &'static rcc::RegisterBlock,
                &'static tim2::RegisterBlock,
                &'static gpioa::RegisterBlock,
                &'static tim6::RegisterBlock,
                DWT) {
    // restrict access to the other peripherals
    stm32f30x::Peripherals::take().unwrap();
    let p = cortex_m::Peripherals::take().unwrap();
    unsafe { (&*GPIOE::ptr(), &*RCC::ptr(), &*TIM2::ptr(), &*GPIOA::ptr(), &*TIM6::ptr(), p.DWT) }
}
