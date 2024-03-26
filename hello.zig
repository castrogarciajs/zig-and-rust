const std = @import("std");

pub fn great() !void {
    const stdout = std.io.getStdOut().writer();

    stdout.print("Hello world");
}
