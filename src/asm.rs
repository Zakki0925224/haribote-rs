use core::arch::asm;

pub fn load_eflags() -> i32
{
    let mut result;

    unsafe
    {
        asm!("pushfd");
        asm!("pop {}", out(reg) result);
    }
    return result;
}

pub fn store_eflags(flags: i32)
{
    unsafe
    {
        asm!("mov eax, {}", in(reg) flags);
        asm!("push eax");
        asm!("popfd");
    }
}

pub fn hlt()
{
    unsafe
    {
        asm!("hlt");
    }
}

pub fn cli()
{
    unsafe
    {
        asm!("cli");
    }
}

pub fn sti()
{
    unsafe
    {
        asm!("sti");
    }
}

pub fn out8(port: u32, data: u8)
{
    unsafe
    {
        asm!("out dx, al", in("edx") port, in("al") data);
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct Dtr
{
    limit: i16,
    base: i32,
}

pub fn load_gdtr(limit: i32, adr: i32)
{
    unsafe
    {
        asm!("lgdt [{}]", in(reg) &Dtr { limit: limit as i16, base: adr });
    }
}

pub fn load_idtr(limit: i32, adr: i32)
{
    unsafe
    {
        asm!("lidt [{}]", in(reg) &Dtr { limit: limit as i16, base: adr });
    }
}

#[macro_export]
macro_rules! handler
{
    ($name: ident) =>
    {{
        pub extern "C" fn wrapper() -> !
        {
            unsafe
            {
                asm!("push es");
                asm!("push ds");
                asm!("pushad");
                asm!("mov eax, esp");
                asm!("push eax");
                asm!("mov ax, ss");
                asm!("mov ds, ax");
                asm!("mov es, ax");
                asm!("call {}", in(reg) $name as extern "C" fn());
                asm!("pop eax");
                asm!("popad");
                asm!("pop ds");
                asm!("pop es");
                asm!("iret");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}