const std = @import("std");

pub fn main() void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    const input_file = std.fs.cwd().openFile("input.txt", .{ .mode = .read_only }) catch |err| {
        stderr.print("Error reading input.txt: {}\n", .{err}) catch {};
        stderr.flush() catch {};
        std.process.exit(1);
    };
    defer input_file.close();

    var read_buffer: [1024]u8 = undefined;
    var file_reader = input_file.reader(&read_buffer);

    var result: i32 = 0;

    while (file_reader.read(&read_buffer)) |bytes_read| {
        if (bytes_read == 0) break; // EOF
        result += processChunk(read_buffer[0..bytes_read]);
    } else |err| {
        if (err != error.EndOfStream) {
            stderr.print("Error reading input.txt: {}\n", .{err}) catch {};
            stderr.flush() catch {};
            std.process.exit(2);
        }
    }

    stdout.print("{d}\n", .{result}) catch |err| {
        stderr.print("Error printing result: {}\n", .{err}) catch {};
        stderr.flush() catch {};
        std.process.exit(1);
    };
    stdout.flush() catch {};
}

fn processChunk(line: []const u8) i32 {
    var floor: i32 = 0;

    for (line) |character| {
        switch (character) {
            '(' => floor += 1,
            ')' => floor -= 1,
            else => {},
        }
    }

    return floor;
}

test "processChunk with example" {
    try std.testing.expectEqual(processChunk("(())"), 0);
    try std.testing.expectEqual(processChunk("()()"), 0);
    try std.testing.expectEqual(processChunk("((("), 3);
    try std.testing.expectEqual(processChunk("(()(()("), 3);
    try std.testing.expectEqual(processChunk("))((((("), 3);
    try std.testing.expectEqual(processChunk("())"), -1);
    try std.testing.expectEqual(processChunk("))("), -1);
    try std.testing.expectEqual(processChunk(")))"), -3);
    try std.testing.expectEqual(processChunk(")())())"), -3);
}
