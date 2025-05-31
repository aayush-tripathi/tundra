// File: fib_recursive.zig
const std = @import("std");

fn fib(n: u64) u64 {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

pub fn main() !void { // `!void` allows error propagation
    const stdout = std.io.getStdOut().writer();
    const n: u64 = 30;
    // The `try` here handles any write‐errors on stdout
    try stdout.print("{d}\n", .{fib(n)});
}
