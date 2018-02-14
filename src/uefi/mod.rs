pub type EFI_HANDLE = *const ();
pub struct EFI_GUID(u32, u16, u16, [u8; 8]);

#[repr(C)]
struct EFI_TABLE_HEADER {
    Signature  : u64,
    Revision   : u32,
    HeaderSize : u32,
    CRC32      : u32,
    Reserved : u32
}

#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    Hdr : EFI_TABLE_HEADER,
    FirmwareVendor : *const u16,
    FirmwareRevision : u32,
    ConsoleInHandle : EFI_HANDLE,
    ConIn : *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle : EFI_HANDLE,
    ConOut : *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ConsoleErrorHandle : EFI_HANDLE,
    StdErr : *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    RuntimeServices : *const EFI_RUNTIME_SERVICES,
    BootServices : *const EFI_BOOT_SERVICES,
    NumberOfTableEntries : usize,
    ConfigurationTable : *const EFI_CONFIGURATION_TABLE
}

pub static mut SYSTEM_TABLE : *const EFI_SYSTEM_TABLE = 0 as *const EFI_SYSTEM_TABLE;

struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset : EFI_TEXT_RESET,
    OutputString : unsafe extern "win64" fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *const u16), //-> Status, //EFI_TEXT_STRING,
    // ... and more stuff that we're ignoring.
}

type EFI_TEXT_RESET = *const ();

type EFI_TEXT_STRING = extern "win64" fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
                                         *const u16);

struct EFI_RUNTIME_SERVICES;

struct EFI_BOOT_SERVICES;

struct EFI_CONFIGURATION_TABLE {
    VendorGuid : EFI_GUID,
    VendorTable : *const ()
}

pub struct SystemTable(*const EFI_SYSTEM_TABLE);


impl SystemTable {
    pub fn console(&self) -> Console {
        unsafe {
            let &SystemTable(tbl) = self;
            Console {
                input:  (*tbl).ConIn,
                output: (*tbl).ConOut,
            }
        }
    }
}

fn unpack<T>(slice: &[T]) -> (*const T, usize) {
    unsafe {
        transmute(slice)
    }
}

pub trait SimpleTextOutput {
    unsafe fn write_raw(&self, str: *const u16);
    
    fn write(&self, str: &str) {
        let mut buf2 = ['X' as u16,
                     'e' as u16,
                     'l' as u16,
                     'l' as u16,
                     'o' as u16,
                     ',' as u16,
                     ' ' as u16,
                     'W' as u16,
                     'o' as u16,
                     'r' as u16,
                     'l' as u16,
                     'd' as u16,
                     '\r' as u16,
                     '\n' as u16,
                     0u16];


        let mut buf = [0u16; 2024];
        // buf[0] = 'X' as u16;
        // buf[1] = 'e' as u16;
        // buf[2] = 'l' as u16;
        // buf[3] = 'l' as u16;
        // buf[4] = 'o' as u16;
        // buf[5] = ',' as u16;
        // buf[6] = ' ' as u16;
        // buf[7] = 'W' as u16;
        // buf[8] = 'o' as u16;
        // buf[9] = 'r' as u16;
        // buf[10] = 'l' as u16;
        // buf[11] = 'd' as u16;
        // buf[12] = '\r' as u16;
        // buf[13] = '\n' as u16;
        // buf[14] = 0u16;

        let mut i = 0;
        let mut char_iter = str.chars();
        while i < buf.len() - 3 {
            let c = char_iter.next();
            match c {
                Some(c) => {
                    // TODO: make sure the characters are all ascii
                    buf[i] = c as u16;
                },
                None =>  {
                    buf[i] = '\r' as u16;
                    buf[i + 1] = '\n' as u16;
                    buf[i + 2] = 0u16;
                    break
                }

            }
            i += 1;
        }

        buf[buf.len() - 1] = 0;

        unsafe { 
            let (p, _) = unpack(&buf2);
            self.write_raw(p);
        }
    }
}

pub trait SimpleTextInput {
}

pub struct Console {
    input  : *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    output : *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
}

impl SimpleTextOutput for Console {
    unsafe fn write_raw(&self, str: *const u16) {
        ((*(*self).output).OutputString)(self.output, str);
    }
}

impl SimpleTextInput for Console {
}

extern "rust-intrinsic" {
    fn transmute<T,U>(val: T) -> U;
}

// We also need some helpers to find a pointer to the hello world string.
fn buf_ptr<T>(buf: &[T]) -> (*const T, usize) {
    unsafe { transmute(buf) }
}

#[no_mangle]
pub extern "win64" fn efi_start(_ImageHandle : EFI_HANDLE,
                                sys_table : *const EFI_SYSTEM_TABLE) -> isize {
    // unsafe { SYSTEM_TABLE = sys_table; }
    // ::efi_main(SystemTable(sys_table));
    // 0

    unsafe {
        let st = SystemTable(sys_table);
        let console = st.console();
        let conout = console.output;

        // let conout = (*sys_table).ConOut;
        let output = (*conout).OutputString;


        let hello = ['H' as u16,
                     'e' as u16,
                     'l' as u16,
                     'l' as u16,
                     'o' as u16,
                     ',' as u16,
                     ' ' as u16,
                     'W' as u16,
                     'o' as u16,
                     'r' as u16,
                     'l' as u16,
                     'd' as u16,
                     '\r' as u16,
                     '\n' as u16,
                     0u16];
        let (hello_ptr, _) = unpack(&hello);

        //output(conout, hello_ptr);
        console.write("Fuck everyone");

        // loop {
        // }
    }
    0
}

#[no_mangle]
pub fn __morestack() {
    // Horrible things will probably happen if this is ever called.
}

#[no_mangle]
pub extern fn memset(s : *const u8, c : isize, n : usize) -> *const u8 {
    unsafe {
        let s : &mut [u8] = transmute((s, n));
        let mut i = 0;
        while i < n {
            s[i] = c as u8;
            i += 1;
            // Use inline assembly here to defeat LLVM's loop-idiom pass
            asm!("");
        }
    }

    s
}
