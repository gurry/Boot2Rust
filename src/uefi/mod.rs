pub type EFI_HANDLE = *const ();
pub struct EFI_GUID(u32, u16, u16, [u8; 8]);

struct EFI_TABLE_HEADER {
    Signature  : u64,
    Revision   : u32,
    HeaderSize : u32,
    CRC32      : u32,
    Reserved : u32
}

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
        // let mut buf = [0u16; 4096];

        let mut buf = ['H' as u16,
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

        // let mut i = 0;
        // for c in str.chars() {
        //     if i >= buf.len() {
        //         break;
        //     }
        //     buf[i] = c as u16;
        //     i += 1;
        // }

        // buf[buf.len() - 1] = 0;
        
        unsafe {
            let (p, _) = unpack(&buf);
            // self.write_raw(p);
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
        let sys_table = &*sys_table;
        let vendor = sys_table.FirmwareVendor;
        let conout = sys_table.ConOut;
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
        let (hello_ptr, _) = buf_ptr(&hello);

        // let m = 1/0;
        if output {

        }
        output(conout, hello_ptr);

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
