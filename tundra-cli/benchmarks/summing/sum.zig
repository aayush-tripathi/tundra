// File: sum_squares/sum_squares.zig
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const N: usize = 3_000_000;
    var total: i64 = 0;
    var i: usize = 1;

    while (i <= N) {
        const i_as_i64: i64 = @as(i64, @intCast(i));
        const ii: i64 = i_as_i64 * i_as_i64;
        total += ii;
        i += 1;
    }

    try stdout.print("{d}\n", .{total});
}
