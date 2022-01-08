#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "18 - UART0"]
    UART0 = 18,
    #[doc = "19 - UART1"]
    UART1 = 19,
    #[doc = "20 - UART2"]
    UART2 = 20,
    #[doc = "21 - UART3"]
    UART3 = 21,
    #[doc = "22 - UART4"]
    UART4 = 22,
    #[doc = "23 - UART5"]
    UART5 = 23,
    #[doc = "25 - TWI0"]
    TWI0 = 25,
    #[doc = "26 - TWI1"]
    TWI1 = 26,
    #[doc = "27 - TWI2"]
    TWI2 = 27,
    #[doc = "28 - TWI3"]
    TWI3 = 28,
    #[doc = "31 - SPI0"]
    SPI0 = 31,
    #[doc = "32 - SPI1"]
    SPI1 = 32,
    #[doc = "34 - PWM"]
    PWM = 34,
    #[doc = "71 - HSTIMER0"]
    HSTIMER0 = 71,
    #[doc = "72 - HSTIMER1"]
    HSTIMER1 = 72,
    #[doc = "73 - GPADC"]
    GPADC = 73,
    #[doc = "75 - TIMER0"]
    TIMER0 = 75,
    #[doc = "76 - TIMER1"]
    TIMER1 = 76,
    #[doc = "77 - LRADC"]
    LRADC = 77,
    #[doc = "78 - TPADC"]
    TPADC = 78,
    #[doc = "79 - WATCHDOG"]
    WATCHDOG = 79,
    #[doc = "85 - GPIOB_NS"]
    GPIOB_NS = 85,
    #[doc = "87 - GPIOC_NS"]
    GPIOC_NS = 87,
    #[doc = "89 - GPIOD_NS"]
    GPIOD_NS = 89,
    #[doc = "91 - GPIOE_NS"]
    GPIOE_NS = 91,
    #[doc = "93 - GPIOF_NS"]
    GPIOF_NS = 93,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            18 => Ok(Interrupt::UART0),
            19 => Ok(Interrupt::UART1),
            20 => Ok(Interrupt::UART2),
            21 => Ok(Interrupt::UART3),
            22 => Ok(Interrupt::UART4),
            23 => Ok(Interrupt::UART5),
            25 => Ok(Interrupt::TWI0),
            26 => Ok(Interrupt::TWI1),
            27 => Ok(Interrupt::TWI2),
            28 => Ok(Interrupt::TWI3),
            31 => Ok(Interrupt::SPI0),
            32 => Ok(Interrupt::SPI1),
            34 => Ok(Interrupt::PWM),
            71 => Ok(Interrupt::HSTIMER0),
            72 => Ok(Interrupt::HSTIMER1),
            73 => Ok(Interrupt::GPADC),
            75 => Ok(Interrupt::TIMER0),
            76 => Ok(Interrupt::TIMER1),
            77 => Ok(Interrupt::LRADC),
            78 => Ok(Interrupt::TPADC),
            79 => Ok(Interrupt::WATCHDOG),
            85 => Ok(Interrupt::GPIOB_NS),
            87 => Ok(Interrupt::GPIOC_NS),
            89 => Ok(Interrupt::GPIOD_NS),
            91 => Ok(Interrupt::GPIOE_NS),
            93 => Ok(Interrupt::GPIOF_NS),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
