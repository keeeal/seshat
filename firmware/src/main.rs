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
    gpiob::PB10<Output<PushPull>>,
    gpiob::PB2<Output<PushPull>>,
    gpiob::PB1<Output<PushPull>>,
    gpiob::PB0<Output<PushPull>>,
    gpioa::PA7<Output<PushPull>>,
    gpioa::PA6<Output<PushPull>>,
    gpioa::PA5<Output<PushPull>>,
    gpioa::PA4<Output<PushPull>>,
    gpioa::PA3<Output<PushPull>>,
    gpioa::PA2<Output<PushPull>>,
    gpioa::PA1<Output<PushPull>>,
    gpioa::PA0<Output<PushPull>>,
    gpiob::PB15<Output<PushPull>>,
    gpioa::PA8<Output<PushPull>>,
);
impl_heterogenous_array! {
    Cols,
    dyn OutputPin<Error = Infallible>,
    U14,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
}

pub struct Rows(
    gpiob::PB3<Input<PullUp>>,
    gpiob::PB4<Input<PullUp>>,
    gpiob::PB5<Input<PullUp>>,
    gpiob::PB6<Input<PullUp>>,
    gpiob::PB7<Input<PullUp>>,
);
impl_heterogenous_array! {
    Rows,
    dyn InputPin<Error = Infallible>,
    U5,
    [0, 1, 2, 3, 4]
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<()> = keyberon::layout::layout! {
    {
        [ Escape Tab (1) LShift LCtrl ]
        [ Kb1 Q A Z LGui]
        [ Kb2 W S X LAlt ]
        [ Kb3 E D n n ]
        [ Kb4 R F C n ]
        [ Kb5 T G V n]
        [ Kb6 Y H B Space ]
        [ Kb7 U J N n ]
        [ Kb8 I K M n ]
        [ Kb9 O L Comma n ]
        [ Kb0 P SColon Dot RAlt ]
        [ Minus LBracket Quote Slash RGui ]
        [ Equal RBracket n n n ]
        [ BSpace Bslash Enter RShift RCtrl ]
    }
    {
        [ n n n n n ]
        [ n n Left n n ]
        [ n Up Down n n ]
        [ n n Right n n ]
        [ n n n n n ]
        [ n n n n n ]
        [ n n n n n ]
        [ n n n n n ]
        [ n n n n n ]
        [ n n n n n ]
        [ n n n n n ]
        [ n n n n n ]
        [ n n n n n ]
        [ n n n n n ]
    }
};

pub struct Leds {}
impl keyberon::keyboard::Leds for Leds {}

#[app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
        matrix: Matrix<Rows, Cols>,
        debouncer: Debouncer<PressedKeys<U14, U5>>,
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
            Rows(
                gpiob.pb3.into_pull_up_input(),
                gpiob.pb4.into_pull_up_input(),
                gpiob.pb5.into_pull_up_input(),
                gpiob.pb6.into_pull_up_input(),
                gpiob.pb7.into_pull_up_input(),
            ),
            Cols(
                gpiob.pb10.into_push_pull_output(),
                gpiob.pb2.into_push_pull_output(),
                gpiob.pb1.into_push_pull_output(),
                gpiob.pb0.into_push_pull_output(),
                gpioa.pa7.into_push_pull_output(),
                gpioa.pa6.into_push_pull_output(),
                gpioa.pa5.into_push_pull_output(),
                gpioa.pa4.into_push_pull_output(),
                gpioa.pa3.into_push_pull_output(),
                gpioa.pa2.into_push_pull_output(),
                gpioa.pa1.into_push_pull_output(),
                gpioa.pa0.into_push_pull_output(),
                gpiob.pb15.into_push_pull_output(),
                gpioa.pa8.into_push_pull_output(),
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
