const std = @import("std");
const Reader = std.Io.Reader;

const file_name = "input.txt";
const line_max_length = 1024;
const dial_size = 100;

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

fn processResult(reader: *Reader) !i32 {
    var result: i32 = 0;
    var dial: i32 = 50;

    while (try reader.takeDelimiter('\n')) |line| {
        const movement = try parseLine(line);

        const new_dial = dial + movement;

        if (new_dial <= 0) {
            if (dial != 0) {
                result += 1;
            }
            result += -1 * @divTrunc(new_dial, dial_size);
        } else if (new_dial >= 100) {
            result += @divTrunc(new_dial, dial_size);
        }

        dial = @mod(new_dial, dial_size);
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

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(6, processResult(&reader));
}

test "processResult 1: 0 zero" {
    var reader = Reader.fixed("L49");
    try std.testing.expectEqual(0, processResult(&reader));
}

test "processResult 0: 1 zero" {
    var reader = Reader.fixed("L50");
    try std.testing.expectEqual(1, processResult(&reader));
}

test "processResult -1: 1 zero" {
    var reader = Reader.fixed("L51");
    try std.testing.expectEqual(1, processResult(&reader));
}

test "processResult -99: 1 zero" {
    var reader = Reader.fixed("L149");
    try std.testing.expectEqual(1, processResult(&reader));
}

test "processResult -100: 2 zeros" {
    var reader = Reader.fixed("L150");
    try std.testing.expectEqual(2, processResult(&reader));
}

test "processResult -101: 2 zeros" {
    var reader = Reader.fixed("L151");
    try std.testing.expectEqual(2, processResult(&reader));
}

test "processResult -199: 2 zeros" {
    var reader = Reader.fixed("L249");
    try std.testing.expectEqual(2, processResult(&reader));
}

test "processResult -200: 3 zeros" {
    var reader = Reader.fixed("L250");
    try std.testing.expectEqual(3, processResult(&reader));
}

test "processResult -201: 3 zeros" {
    var reader = Reader.fixed("L251");
    try std.testing.expectEqual(3, processResult(&reader));
}

test "processResult 99: 0 zero" {
    var reader = Reader.fixed("R49");
    try std.testing.expectEqual(0, processResult(&reader));
}

test "processResult 100: 1 zero" {
    var reader = Reader.fixed("R50");
    try std.testing.expectEqual(1, processResult(&reader));
}

test "processResult 101: 1 zero" {
    var reader = Reader.fixed("R51");
    try std.testing.expectEqual(1, processResult(&reader));
}

test "processResult 199: 1 zero" {
    var reader = Reader.fixed("R149");
    try std.testing.expectEqual(1, processResult(&reader));
}

test "processResult 200: 2 zero" {
    var reader = Reader.fixed("R150");
    try std.testing.expectEqual(2, processResult(&reader));
}

test "processResult 201: 2 zero" {
    var reader = Reader.fixed("R151");
    try std.testing.expectEqual(2, processResult(&reader));
}

test "processResult 0 and -1: 1 zero" {
    const input =
        \\L50
        \\L1
    ;

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(1, processResult(&reader));
}

test "processResult 0 and -100: 2 zero" {
    const input =
        \\L50
        \\L100
    ;

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(2, processResult(&reader));
}
