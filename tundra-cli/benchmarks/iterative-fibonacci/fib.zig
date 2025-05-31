const std = @import("std");
pub fn main() void {
    var a: usize = 0;
    var b: usize = 1;
    const n = 80;
    for (0..n) | _ | {
        const tmp = a + b;
        a = b;
        b = tmp;
    }
    std.debug.print("{}\n", .{a});
}