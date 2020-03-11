// Code for PWM Initialization and configuration

#![no_std]

#[allow(unused_extern_crates)]

use aux8::tim2;

pub fn init(tim2: &tim2::RegisterBlock, arr_val: u16, psc: u16) {
    unsafe {
        tim2.arr.write(|w| w.arrl().bits(arr_val)); // frequency of pwm signal every 1000 us inc cnt by 1 -> 1kHz
        tim2.psc.write(|w| w.psc().bits(psc)); // prescaler for TIM2 (freq of timer = freq_of_sysclk / (n + 1)) here 1MHz
        tim2.ccmr1_output.modify(|_, w| w.oc2m().bits(6)); // select pwm mode 1 (edge aligned mode)
    }
    tim2.ccmr1_output.modify(|_, w| w.oc2pe().set_bit()); // enable preload
    tim2.cr1.modify(|_, w| w.arpe().set_bit()); // enable autoreload preload
    tim2.ccer.modify(|_, w| w.cc2e().set_bit()); // enable channel 2 output
    tim2.egr.write(|w| w.ug().set_bit()); // initialize all registers with update generation
    
    tim2.cr1.reset(); // reset cr1
    tim2.cr2.reset(); // reset cr2
    tim2.cr1.modify(|_, w| w.cen().set_bit()); // enable counter
}

pub fn set_duty_cycle(tim2: &tim2::RegisterBlock, duty_cycle: u16) {
    if duty_cycle >= 1000 {
        tim2.ccr2.write(|w| unsafe {w.ccr2l().bits(1001)})
    }
    tim2.ccr2.write(|w| unsafe {w.ccr2l().bits(duty_cycle)}) // pwm signal is high till TIM2_CNT reaches ccr2

}
