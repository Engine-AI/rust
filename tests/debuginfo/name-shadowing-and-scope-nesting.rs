//@ min-lldb-version: 310

//@ compile-flags:-g

// === GDB TESTS ===================================================================================

// gdb-command:run

// gdb-command:print x
// gdb-check:$1 = false
// gdb-command:print y
// gdb-check:$2 = true
// gdb-command:continue

// gdb-command:print x
// gdb-check:$3 = 10
// gdb-command:print y
// gdb-check:$4 = true
// gdb-command:continue

// gdb-command:print x
// gdb-check:$5 = 10.5
// gdb-command:print y
// gdb-check:$6 = 20
// gdb-command:continue

// gdb-command:print x
// gdb-check:$7 = true
// gdb-command:print y
// gdb-check:$8 = 2220
// gdb-command:continue

// gdb-command:print x
// gdb-check:$9 = 203203.5
// gdb-command:print y
// gdb-check:$10 = 2220
// gdb-command:continue

// gdb-command:print x
// gdb-check:$11 = 10.5
// gdb-command:print y
// gdb-check:$12 = 20
// gdb-command:continue


// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:v x
// lldbg-check:[...] false
// lldbr-check:(bool) x = false
// lldb-command:v y
// lldbg-check:[...] true
// lldbr-check:(bool) y = true
// lldb-command:continue

// lldb-command:v x
// lldbg-check:[...] 10
// lldbr-check:(i32) x = 10
// lldb-command:v y
// lldbg-check:[...] true
// lldbr-check:(bool) y = true
// lldb-command:continue

// lldb-command:v x
// lldbg-check:[...] 10.5
// lldbr-check:(f64) x = 10.5
// lldb-command:v y
// lldbg-check:[...] 20
// lldbr-check:(i32) y = 20
// lldb-command:continue

// lldb-command:v x
// lldbg-check:[...] true
// lldbr-check:(bool) x = true
// lldb-command:v y
// lldbg-check:[...] 2220
// lldbr-check:(i32) y = 2220
// lldb-command:continue

// lldb-command:v x
// lldbg-check:[...] 203203.5
// lldbr-check:(f64) x = 203203.5
// lldb-command:v y
// lldbg-check:[...] 2220
// lldbr-check:(i32) y = 2220
// lldb-command:continue

// lldb-command:v x
// lldbg-check:[...] 10.5
// lldbr-check:(f64) x = 10.5
// lldb-command:v y
// lldbg-check:[...] 20
// lldbr-check:(i32) y = 20
// lldb-command:continue

#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

fn main() {
    let x = false;
    let y = true;

    zzz(); // #break
    sentinel();

    let x = 10;

    zzz(); // #break
    sentinel();

    let x = 10.5f64;
    let y = 20;

    zzz(); // #break
    sentinel();

    {
        let x = true;
        let y = 2220;

        zzz(); // #break
        sentinel();

        let x = 203203.5f64;

        zzz(); // #break
        sentinel();
    }

    zzz(); // #break
    sentinel();
}

fn zzz() {()}
fn sentinel() {()}
