const std = @import("std");
const Reader = std.Io.Reader;

const file_name = "input.txt";
const line_max_length = 1024;
const number_of_batteries = 12;

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

fn parseLine(line: []const u8) !u128 {
    var batteries: [number_of_batteries]u8 = undefined;
    @memcpy(&batteries, line[0..number_of_batteries]);

    var i: u8 = 1;
    while (i < line.len - (number_of_batteries - 1)) {
        std.debug.print("i:{d}\n", .{i});
        for (i..i + number_of_batteries) |j| {
            // TODO: logic on 12 batteries
            std.debug.print("j:{d}\n", .{j});
        }

        i += 1;
    }

    return try std.fmt.parseInt(u128, &batteries, 10);
}

fn processResult(reader: *Reader) !u128 {
    var result: u128 = 0;

    while (try reader.takeDelimiter('\n')) |line| {
        result += try parseLine(line);
    }

    return result;
}

test "parseLine with example" {
    try std.testing.expectEqual(987654321111, parseLine("987654321111111"));
    try std.testing.expectEqual(811111111119, parseLine("811111111111119"));
    try std.testing.expectEqual(434234234278, parseLine("234234234234278"));
    try std.testing.expectEqual(888911112111, parseLine("818181911112111"));
}

test "processResult with example" {
    const input =
        \\987654321111111
        \\811111111111119
        \\234234234234278
        \\818181911112111
    ;

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(3121910778619, processResult(&reader));
}
