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

fn parseRange(line: []const u8) !u128 {
    var range = std.mem.splitScalar(u8, line, '-');
    const range_0 = range.next().?;
    const range_1 = range.next().?;

    const range_start = try std.fmt.parseInt(u128, range_0, 10);
    const range_end = try std.fmt.parseInt(u128, range_1, 10);

    const number_of_digit_start = @divTrunc(range_0.len, 2);
    const number_of_digit_end = @divTrunc(range_1.len, 2);

    var total: u128 = 0;

    for (number_of_digit_start..number_of_digit_end + 1) |number_of_digit| {
        // FIXME: number_of_digit == 0 if len == 1 and div by 2
        std.debug.print("{d}\n", .{number_of_digit});
        const start_number_of_digit = if (number_of_digit == 0) 0 else number_of_digit - 1;
        const end_number_of_digit = if (number_of_digit == 0) 1 else number_of_digit;

        const start = std.math.pow(usize, 10, start_number_of_digit);
        const end = std.math.pow(usize, 10, end_number_of_digit);

        for (start..end) |number_to_double| {
            const doubled: u128 = @intCast(number_to_double + number_to_double * std.math.pow(usize, 10, number_of_digit));

            if (doubled >= range_start and doubled <= range_end) {
                total += doubled;
            }
        }
    }

    return total;
}

fn processResult(reader: *Reader) !u128 {
    var result: u128 = 0;

    while (try reader.takeDelimiter(',')) |line| {
        result += try parseRange(line);
    }

    return result;
}

test "parseRange with example" {
    try std.testing.expectEqual(33, parseRange("11-22"));
    try std.testing.expectEqual(99, parseRange("95-115"));
    try std.testing.expectEqual(1010, parseRange("998-1012"));
    try std.testing.expectEqual(1188511885, parseRange("1188511880-1188511890"));
    try std.testing.expectEqual(222222, parseRange("222220-222224"));
    try std.testing.expectEqual(0, parseRange("1698522-1698528"));
    try std.testing.expectEqual(446446, parseRange("446443-446449"));
    try std.testing.expectEqual(38593859, parseRange("38593856-38593862"));
    try std.testing.expectEqual(0, parseRange("565653-565659"));
    try std.testing.expectEqual(0, parseRange("824824821-824824827"));
    try std.testing.expectEqual(0, parseRange("2121212118-2121212124"));
}

test "processResult with example" {
    const input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(1227775554, processResult(&reader));
}
