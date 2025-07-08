use aarch64_cpu::registers::{Readable, Writeable, ReadWriteable};
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};
use core::ptr::NonNull;
use log::{info,error};

register_bitfields![u32,
    pub UARTDR [
        DATA OFFSET(0) NUMBITS(8) [],
        FE OFFSET(8) NUMBITS(1) [],
        PE OFFSET(9) NUMBITS(1) [],
        BE OFFSET(10) NUMBITS(1) [],
        OE OFFSET(11) NUMBITS(1) [],
    ],

    pub UARTECR [
        UARTECR OFFSET(0) NUMBITS(8) []
    ],

    pub UARTRSR [
        FE OFFSET(0) NUMBITS(1) [],
        PE OFFSET(1) NUMBITS(1) [],
        BE OFFSET(2) NUMBITS(1) [],
        OE OFFSET(3) NUMBITS(1) [],
    ],

    pub UARTFR [
        CTS OFFSET(0) NUMBITS(1) [],
        DSR OFFSET(1) NUMBITS(1) [],
        DCD OFFSET(2) NUMBITS(1) [],
        BUSY OFFSET(3) NUMBITS(1) [],
        RXFE OFFSET(4) NUMBITS(1) [],
        TXFF OFFSET(5) NUMBITS(1) [],
        RXFF OFFSET(6) NUMBITS(1) [],
        TXFE OFFSET(7) NUMBITS(1) [],
        RI OFFSET(8) NUMBITS(1) [],
    ],

    pub UARTILPR [
        ILPDVSR OFFSET(0) NUMBITS(8) [],
    ],

    pub UARTIBRD [
        BAUD_DIVINT OFFSET(0) NUMBITS(16) [],
    ],

    pub UARTFBRD [
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) [],
    ],

    pub UARTLCR_H [
        BRK OFFSET(0) NUMBITS(1) [],
        PEN OFFSET(1) NUMBITS(1) [],
        EPS OFFSET(2) NUMBITS(1) [],
        STP2 OFFSET(3) NUMBITS(1) [],
        FEN OFFSET(4) NUMBITS(1) [],
        WLEN OFFSET(5) NUMBITS(2) [
            FIVE_BITS = 0,
            SIX_BITS = 1,
            SEVEN_BITS = 2,
            EIGHT_BITS = 3,
        ],
        SPS OFFSET(7) NUMBITS(1) [],
    ],

    pub UARTCR [
        UARTEN OFFSET(0) NUMBITS(1) [],
        SIREN OFFSET(1) NUMBITS(1) [],
        TXE OFFSET(8) NUMBITS(1) [],
        RXE OFFSET(9) NUMBITS(1) [],
        DTR OFFSET(10) NUMBITS(1) [],
        RTS OFFSET(11) NUMBITS(1) [],
        OUT1 OFFSET(12) NUMBITS(1) [],
        OUT2 OFFSET(13) NUMBITS(1) [],
        RTSEN OFFSET(14) NUMBITS(1) [],
        CTSEN OFFSET(15) NUMBITS(1) []
    ],

    pub UARTIFLS [
        TXIFLSEL OFFSET(0) NUMBITS(3) [
            FULL_1_8 = 0,   // 1/8
            FULL_1_4 = 1,
            FULL_1_2 = 2,
            FULL_3_4 = 3,
            FULL_7_8 = 4,
        ],
        RXIFLSEL OFFSET(3) NUMBITS(3) [
            FULL_1_8 = 0,
            FULL_1_4 = 1,
            FULL_1_2 = 2,
            FULL_3_4 = 3,
            FULL_7_8 = 4,
        ]
    ],

    pub UARTIMSC [
        RIMIM OFFSET(0) NUMBITS(1) [],
        CTSMIM OFFSET(1) NUMBITS(1) [],
        DCDMIM OFFSET(2) NUMBITS(1) [],
        DSRMIM OFFSET(3) NUMBITS(1) [],
        RXIM OFFSET(4) NUMBITS(1) [],
        TXIM OFFSET(5) NUMBITS(1) [],
        RTIM OFFSET(6) NUMBITS(1) [],
        FEIM OFFSET(7) NUMBITS(1) [],
        PEIM OFFSET(8) NUMBITS(1) [],
        BEIM OFFSET(9) NUMBITS(1) [],
        OEIM OFFSET(10) NUMBITS(1) []
    ],

    pub UARTRIS [
        RIRMIS OFFSET(0) NUMBITS(1) [],
        CTSRMIS OFFSET(1) NUMBITS(1) [],
        DCDRMIS OFFSET(2) NUMBITS(1) [],
        DSRRMIS OFFSET(3) NUMBITS(1) [],
        RXRIS OFFSET(4) NUMBITS(1) [],
        TXRIS OFFSET(5) NUMBITS(1) [],
        RTRIS OFFSET(6) NUMBITS(1) [],
        FERIS OFFSET(7) NUMBITS(1) [],
        PERIS OFFSET(8) NUMBITS(1) [],
        BERIS OFFSET(9) NUMBITS(1) [],
        OERIS OFFSET(10) NUMBITS(1) [],
    ],

    pub UARTMIS [
        RIRMIS OFFSET(0) NUMBITS(1) [],
        CTSRMIS OFFSET(1) NUMBITS(1) [],
        DCDRMIS OFFSET(2) NUMBITS(1) [],
        DSRRMIS OFFSET(3) NUMBITS(1) [],
        RXRIS OFFSET(4) NUMBITS(1) [],
        TXRIS OFFSET(5) NUMBITS(1) [],
        RTRIS OFFSET(6) NUMBITS(1) [],
        FERIS OFFSET(7) NUMBITS(1) [],
        PERIS OFFSET(8) NUMBITS(1) [],
        BERIS OFFSET(9) NUMBITS(1) [],
        OERIS OFFSET(10) NUMBITS(1) [],
    ],

    pub UARTICR [
        RIRMIS OFFSET(0) NUMBITS(1) [],
        CTSMIC OFFSET(1) NUMBITS(1) [],
        DCDMIC OFFSET(2) NUMBITS(1) [],
        DSRMIC OFFSET(3) NUMBITS(1) [],
        RXIC OFFSET(4) NUMBITS(1) [],
        TXIC OFFSET(5) NUMBITS(1) [],
        RTIC OFFSET(6) NUMBITS(1) [],
        FEIC OFFSET(7) NUMBITS(1) [],
        PEIC OFFSET(8) NUMBITS(1) [],
        BEIC OFFSET(9) NUMBITS(1) [],
        OEIC OFFSET(10) NUMBITS(1) [],
    ],

    pub UARTDMACR [
        RXDMAE OFFSET(0) NUMBITS(1) [],
        TXDMAE OFFSET(1) NUMBITS(1) [],
        DMAONERR OFFSET(2) NUMBITS(1) [],
    ]
];

register_structs! {
    pub Pl011Regs {
        (0x000 => pub uartdr: ReadWrite<u32, UARTDR::Register>),
        (0x004 => pub uartecr: ReadOnly<u32>),
        (0x008 => _reserved0),
        (0x018 => pub uartfr: ReadOnly<u32, UARTFR::Register>),
        (0x01c => _reserved1),
        (0x024 => pub uartibrd: ReadWrite<u32, UARTIBRD::Register>),
        (0x028 => pub uartfbrd: ReadWrite<u32, UARTFBRD::Register>),
        (0x02c => pub uartlcrh: ReadWrite<u32, UARTLCR_H::Register>),
        (0x030 => pub uartcr: ReadWrite<u32, UARTCR::Register>),
        (0x034 => pub uartifls: ReadWrite<u32, UARTIFLS::Register>),
        (0x038 => pub uartimsc: ReadWrite<u32, UARTIMSC::Register>),
        (0x03c => pub uartris: ReadOnly<u32, UARTRIS::Register>),
        (0x040 => pub uartmis: ReadOnly<u32>),
        (0x044 => pub uarticr: WriteOnly<u32, UARTICR::Register>),
        (0x048 => pub uartdmacr: ReadWrite<u32>),
        (0x04c => @END),
    }
}

pub struct Pl011 {
    base: NonNull<Pl011Regs>,
    clock_hz: u32,
    baudrate: u32,
}

impl Pl011 {
    pub const fn new(base: NonNull<u8>) -> Self {
        Self {
            base: base.cast(),
            clock_hz: 100_000_000,
            baudrate: 0,
        }
    }

    fn set_baudrate(&mut self, baudrate: u32) {
        if (baudrate * 2) > self.clock_hz {
            error!("Pl011: set baudrate {} too high", baudrate);
            return
        }

        let mut temp = 16 * baudrate;
        let divider = self.clock_hz / temp;
        let remainder = self.clock_hz % temp;
        temp = (128 * remainder) / temp;
        let mut fraction = temp / 2;

        if 0 != (temp & 1) {
            fraction += 1;
        }
        // todo ClearSpecificOptions

        // set baudrate
        unsafe {
            self.base.as_ref().uartibrd.write(UARTIBRD::BAUD_DIVINT.val(divider));
            self.base.as_ref().uartfbrd.write(UARTFBRD::BAUD_DIVFRAC.val(fraction));
        }
        
        // todo SetSpecificOptions

        self.baudrate = baudrate;
        info!("Pl011: set baudrate {} done", baudrate);
    }

    pub unsafe fn init(&mut self) {
        self.set_baudrate(115200);
        /*
        * Set up the default data format: 8 bit data, 1 stop bit, no
        * parity
        */
        self.base.as_ref().
            uartlcrh.modify(UARTLCR_H::WLEN::EIGHT_BITS);
        /* Set the RX FIFO trigger at 8 data bytes.Tx FIFO trigger is 8 data bytes*/
        self.base.as_ref().uartifls.modify(UARTIFLS::TXIFLSEL::FULL_1_4
                                          + UARTIFLS::RXIFLSEL::FULL_1_4);

        self.base.as_ref().uartcr.modify(UARTCR::UARTEN::SET
                                    + UARTCR::TXE::SET
                                    + UARTCR::RXE::SET
                                    + UARTCR::DTR::SET
                                    + UARTCR::RTS::SET);
        /* Disable all interrupts, polled mode is the default */
        self.base.as_ref().uartimsc.set(0u32);
        info!("Pl011: init done");
    }

    pub unsafe fn send_byte(&self, data: u8) {
        self.base.as_ref().uartdr.modify(UARTDR::DATA.val(data as u32));
    }

    pub unsafe fn recv_byte(&self) -> u8 {
        self.base.as_ref().uartdr.read(UARTDR::DATA) as u8
    }


}

