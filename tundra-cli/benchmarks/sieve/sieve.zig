const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    // 1) Compile-time constant N
    const N: usize = 200000;

    // 2) Allocate an [N+1] bool array and initialize to false
    var is_composite = [_]bool{false} ** (N + 1);

    // 3) Mark multiples of each prime p up to sqrt(N)
    var p: usize = 2;
    while (p * p <= N) {
        if (!is_composite[p]) {
            var m: usize = p * p;
            while (m <= N) : (m += p) {
                is_composite[m] = true;
            }
        }
        p += 1;
    }

    // 4) Count how many entries from 2..N are still false
    var count: usize = 0;
    for (2..(N + 1)) |i| {
        if (!is_composite[i]) {
            count += 1;
        }
    }

    // 5) Print the result
    try stdout.print("{d}\n", .{count});
}
