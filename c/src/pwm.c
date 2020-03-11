#include "main.h"

void init_pwm(void)
{
  TIM2->ARR = 1000;
  TIM2->PSC = 7;
  TIM2->CCR2 = 500;
  TIM2->CCMR1 = TIM_CCMR1_OC2PE | TIM_CCMR1_OC2M_1 | TIM_CCMR1_OC2M_2;
  TIM2->CCER = TIM_CCER_CC2E;
  TIM2->EGR = TIM_EGR_UG;
  TIM2->CR1 = 0;
  TIM2->CR2 = 0;
  TIM2->CR1 = TIM_CR1_CEN;
}

void set_duty_cycle(unsigned short duty)
{
    if (duty >= 1000) 
    {
        TIM2->CCR2 = 1001;
        
    }
    else
    {
        TIM2->CCR2 = duty;
    }
    
}

