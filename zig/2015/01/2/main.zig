const std = @import("std");

const file_name = "input.txt";

pub fn main() !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    const input_file = std.fs.cwd().openFile(file_name, .{ .mode = .read_only }) catch |err| {
        try stderr.print("Error reading {s}: {}\n", .{ file_name, err });
        try stderr.flush();
        std.process.exit(1);
    };
    defer input_file.close();

    var read_buffer: [1024]u8 = undefined;
    var file_reader = input_file.reader(&read_buffer);

    var result: usize = 0;
    var floor: i32 = 0;

    while (file_reader.read(&read_buffer)) |bytes_read| {
        if (bytes_read == 0) break; // EOF

        if (processChunk(read_buffer[0..bytes_read], &floor)) |basement_index| {
            result += basement_index;
            break;
        }

        result += bytes_read;
    } else |err| if (err != error.EndOfStream) return err;

    try stdout.print("{d}\n", .{result});
    try stdout.flush();
}

fn processChunk(line: []const u8, floor: *i32) ?usize {
    for (line, 0..) |character, index| {
        switch (character) {
            '(' => floor.* += 1,
            ')' => floor.* -= 1,
            else => {},
        }

        if (floor.* == -1) {
            return index + 1;
        }
    }

    return null;
}

test "processChunk with example" {
    var floor: i32 = 0;
    try std.testing.expectEqual(1, processChunk(")", &floor));
    floor = 0;
    try std.testing.expectEqual(5, processChunk("()())", &floor));
}
