#![no_main]
#![no_std]

// set the panic handler
use panic_halt as _;

use core::convert::Infallible;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use generic_array::typenum::{U14, U5};
use keyberon::debounce::Debouncer;
use keyberon::impl_heterogenous_array;
use keyberon::key_code::KbHidReport;
use keyberon::key_code::KeyCode::{self};
use keyberon::layout::Layout;
use keyberon::matrix::{Matrix, PressedKeys};
use rtic::app;
use stm32f4xx_hal::gpio::{gpioa, gpiob, Input, Output, PullUp, PushPull};
use stm32f4xx_hal::otg_fs::{UsbBusType, USB};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::{stm32, timer};
use usb_device::bus::UsbBusAllocator;
use usb_device::class::UsbClass as _;

type UsbClass = keyberon::Class<'static, UsbBusType, Leds>;
type UsbDevice = usb_device::device::UsbDevice<'static, UsbBusType>;

pub struct Cols(
    gpiob::PB10<Input<PullUp>>,
    gpiob::PB2<Input<PullUp>>,
    gpiob::PB1<Input<PullUp>>,
    gpiob::PB0<Input<PullUp>>,
    gpioa::PA7<Input<PullUp>>,
    gpioa::PA6<Input<PullUp>>,
    gpioa::PA5<Input<PullUp>>,
    gpioa::PA4<Input<PullUp>>,
    gpioa::PA3<Input<PullUp>>,
    gpioa::PA2<Input<PullUp>>,
    gpioa::PA1<Input<PullUp>>,
    gpioa::PA0<Input<PullUp>>,
    gpiob::PB15<Input<PullUp>>,
    gpioa::PA8<Input<PullUp>>,
);
impl_heterogenous_array! {
    Cols,
    dyn InputPin<Error = Infallible>,
    U14,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
}

pub struct Rows(
    gpiob::PB3<Output<PushPull>>,
    gpiob::PB4<Output<PushPull>>,
    gpiob::PB5<Output<PushPull>>,
    gpiob::PB6<Output<PushPull>>,
    gpiob::PB7<Output<PushPull>>,
);
impl_heterogenous_array! {
    Rows,
    dyn OutputPin<Error = Infallible>,
    U5,
    [0, 1, 2, 3, 4]
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<()> = keyberon::layout::layout! {
    {
        [ Escape Kb1  Kb2  Kb3 Kb4 Kb5 Kb6   Kb7 Kb8   Kb9  Kb0    Minus    Equal    BSpace ]
        [ Tab    Q    W    E   R   T   Y     U   I     O    P      LBracket RBracket Bslash ]
        [ (1)    A    S    D   F   G   H     J   K     L    SColon Quote    n        Enter  ]
        [ LShift Z    X    C   V   B   N     M   Comma Dot  Slash  n        n        RShift ]
        [ LCtrl  LGui LAlt n   n   n   Space n   n     RAlt n      RGui     n        RCtrl  ]
    }
    {
        [ n n    n    n     n n n n n n n n n n ]
        [ n n    Up   n     n n n n n n n n n n ]
        [ n Left Down Right n n n n n n n n n n ]
        [ n n    n    n     n n n n n n n n n n ]
        [ n n    n    n     n n n n n n n n n n ]
    }
};

pub struct Leds {}
impl keyberon::keyboard::Leds for Leds {}

#[app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
        matrix: Matrix<Cols, Rows>,
        debouncer: Debouncer<PressedKeys<U5, U14>>,
        layout: Layout<()>,
        timer: timer::Timer<stm32::TIM3>,
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

        let rcc = c.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(25.mhz())
            .sysclk(84.mhz())
            .require_pll48clk()
            .freeze();

        let gpioa = c.device.GPIOA.split();
        let gpiob = c.device.GPIOB.split();
        // let gpioc = c.device.GPIOC.split();

        let usb = USB {
            usb_global: c.device.OTG_FS_GLOBAL,
            usb_device: c.device.OTG_FS_DEVICE,
            usb_pwrclk: c.device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate_af10(),
            pin_dp: gpioa.pa12.into_alternate_af10(),
        };
        
        *USB_BUS = Some(UsbBusType::new(usb, EP_MEMORY));
        let usb_bus = USB_BUS.as_ref().unwrap();

        let leds = Leds {};
        let usb_class = keyberon::new_class(usb_bus, leds);
        let usb_dev = keyberon::new_device(usb_bus);

        let mut timer = timer::Timer::tim3(c.device.TIM3, 1.khz(), clocks);
        timer.listen(timer::Event::TimeOut);

        let matrix = Matrix::new(
            Cols(
                gpiob.pb10.into_pull_up_input(),
                gpiob.pb2.into_pull_up_input(),
                gpiob.pb1.into_pull_up_input(),
                gpiob.pb0.into_pull_up_input(),
                gpioa.pa7.into_pull_up_input(),
                gpioa.pa6.into_pull_up_input(),
                gpioa.pa5.into_pull_up_input(),
                gpioa.pa4.into_pull_up_input(),
                gpioa.pa3.into_pull_up_input(),
                gpioa.pa2.into_pull_up_input(),
                gpioa.pa1.into_pull_up_input(),
                gpioa.pa0.into_pull_up_input(),
                gpiob.pb15.into_pull_up_input(),
                gpioa.pa8.into_pull_up_input(),
            ),
            Rows(
                gpiob.pb3.into_push_pull_output(),
                gpiob.pb4.into_push_pull_output(),
                gpiob.pb5.into_push_pull_output(),
                gpiob.pb6.into_push_pull_output(),
                gpiob.pb7.into_push_pull_output(),
            ),
        );

        init::LateResources {
            usb_dev,
            usb_class,
            timer,
            debouncer: Debouncer::new(PressedKeys::default(), PressedKeys::default(), 5),
            matrix: matrix.unwrap(),
            layout: Layout::new(LAYERS),
        }
    }

    #[task(binds = OTG_FS, priority = 2, resources = [usb_dev, usb_class])]
    fn usb_tx(mut c: usb_tx::Context) {
        usb_poll(&mut c.resources.usb_dev, &mut c.resources.usb_class);
    }

    #[task(binds = OTG_FS_WKUP, priority = 2, resources = [usb_dev, usb_class])]
    fn usb_rx(mut c: usb_rx::Context) {
        usb_poll(&mut c.resources.usb_dev, &mut c.resources.usb_class);
    }

    #[task(binds = TIM3, priority = 1, resources = [usb_class, matrix, debouncer, layout, timer])]
    fn tick(mut c: tick::Context) {
        c.resources.timer.clear_interrupt(timer::Event::TimeOut);

        for event in c
            .resources
            .debouncer
            .events(c.resources.matrix.get().unwrap())
        {
            c.resources.layout.event(event);
        }
        match c.resources.layout.tick() {
            keyberon::layout::CustomEvent::Release(()) => cortex_m::peripheral::SCB::sys_reset(),
            _ => (),
        }
        send_report(c.resources.layout.keycodes(), &mut c.resources.usb_class);
    }
};

fn send_report(iter: impl Iterator<Item = KeyCode>, usb_class: &mut resources::usb_class<'_>) {
    use rtic::Mutex;
    let report: KbHidReport = iter.collect();
    if usb_class.lock(|k| k.device_mut().set_keyboard_report(report.clone())) {
        while let Ok(0) = usb_class.lock(|k| k.write(report.as_bytes())) {}
    }
}

fn usb_poll(usb_dev: &mut UsbDevice, keyboard: &mut UsbClass) {
    if usb_dev.poll(&mut [keyboard]) {
        keyboard.poll();
    }
}
