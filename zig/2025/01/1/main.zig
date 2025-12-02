const std = @import("std");

const file_name = "input.txt";
const line_max_length = 1024;

pub fn main() !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    const input_file = std.fs.cwd().openFile(file_name, .{ .mode = .read_only }) catch |err| {
        try stderr.print("Error opening {s}: {}\n", .{ file_name, err });
        try stderr.flush();
        std.process.exit(1);
    };
    defer input_file.close();

    var read_buffer: [line_max_length]u8 = undefined;
    var file_reader = input_file.reader(&read_buffer);
    const reader = &file_reader.interface;

    const result = processResult(reader) catch |err|
        switch (err) {
            error.StreamTooLong => {
                try stderr.print("Error: line is longer than {d} bytes, change the buffer size\n", .{line_max_length});
                try stderr.flush();
                std.process.exit(1);
            },
            else => {
                return err;
            },
        };

    try stdout.print("{d}\n", .{result});
    try stdout.flush();
}

fn parseLine(line: []const u8) !i32 {
    const number = try std.fmt.parseInt(i32, line[1..], 10);

    return switch (line[0]) {
        'L' => -number,
        'R' => number,
        else => error.UnknownDirection,
    };
}

fn processResult(reader: *std.Io.Reader) !i32 {
    var result: i32 = 0;
    var dial: i32 = 50;

    while (try reader.takeDelimiter('\n')) |line| {
        const movement = try parseLine(line);
        dial = @mod(dial + movement, 100);
        if (dial == 0) {
            result += 1;
        }
    }

    return result;
}

test "parseLine with example" {
    try std.testing.expectEqual(-68, parseLine("L68"));
    try std.testing.expectEqual(48, parseLine("R48"));
}

test "processResult with example" {
    const input =
        \\L68
        \\L30
        \\R48
        \\L5
        \\R60
        \\L55
        \\L1
        \\L99
        \\R14
        \\L82
    ;

    var reader = std.Io.Reader.fixed(input);

    try std.testing.expectEqual(3, processResult(&reader));
}
