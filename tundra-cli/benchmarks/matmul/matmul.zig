const std = @import("std");

// Compile-time constant n = 200
const N: usize = 200;

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    // 1) Build A and B as fixed-size arrays: [N][N]usize

    var a: [N][N]usize = undefined;
    // Fill matrix A: a[i][j] = i * N + j
    for (0..N) |i| {
        for (0..N) |j| {
            a[i][j] = i * N + j;
        }
    }

    var b: [N][N]usize = undefined;
    // Fill matrix B: b[i][j] = j * N + i
    for (0..N) |i| {
        for (0..N) |j| {
            b[i][j] = j * N + i;
        }
    }

    // 2) Allocate C as a zero matrix
    var c: [N][N]usize = std.mem.zeroes([N][N]usize);

    // 3) Compute C = A × B
    // Compute c[i][j] = sum(a[i][k] * b[k][j]) for k from 0 to N-1
    for (0..N) |i| {
        for (0..N) |j| {
            var sum: usize = 0;
            for (0..N) |k| {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }

    // 4) Print the result C[0][0]
    try stdout.print("{d}\n", .{c[0][0]});
}
