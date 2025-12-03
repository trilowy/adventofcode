const std = @import("std");
const Reader = std.Io.Reader;

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
    var tens = line[0];
    var units = line[1];

    var i: u8 = 1;
    while (i < line.len - 1) {
        if (line[i] > tens) {
            tens = line[i];
            units = line[i + 1];
        } else if (line[i + 1] > units) {
            units = line[i + 1];
        }

        i += 1;
    }

    var number: [2]u8 = undefined;
    number[0] = tens;
    number[1] = units;

    return try std.fmt.parseInt(i32, &number, 10);
}

fn processResult(reader: *Reader) !i32 {
    var result: i32 = 0;

    while (try reader.takeDelimiter('\n')) |line| {
        result += try parseLine(line);
    }

    return result;
}

test "parseLine with example" {
    try std.testing.expectEqual(98, parseLine("987654321111111"));
    try std.testing.expectEqual(89, parseLine("811111111111119"));
    try std.testing.expectEqual(78, parseLine("234234234234278"));
    try std.testing.expectEqual(92, parseLine("818181911112111"));
}

test "processResult with example" {
    const input =
        \\987654321111111
        \\811111111111119
        \\234234234234278
        \\818181911112111
    ;

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(357, processResult(&reader));
}
