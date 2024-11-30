const std = @import("std");
const file = @embedFile("input.txt");
const print = std.debug.print;

const ascii_offset: usize = '0';

pub fn main() void {
    const result = processFile(file);

    print("{d}\n", .{result});
}

pub fn processFile(data: []const u8) usize {
    var result: usize = 0;

    var lines = std.mem.tokenizeScalar(u8, data, '\n');
    while (lines.next()) |line| {
        result += processLine(line);
    }

    return result;
}

fn processLine(line: []const u8) usize {
    var result: usize = 0;
    for (line) |c| {
        if (std.ascii.isDigit(c)) {
            result = c - ascii_offset;
            break;
        }
    }
    result = result * 10;

    var i: usize = line.len;
    while (i > 0) {
        i -= 1;
        if (std.ascii.isDigit(line[i])) {
            result += line[i] - ascii_offset;
            break;
        }
    }

    return result;
}

test "processLine with 2 numbers" {
    const result = processLine("pqr3stu8vwx");

    try std.testing.expectEqual(38, result);
}

test "processLine with 1 number" {
    const result = processLine("treb7uchet");

    try std.testing.expectEqual(77, result);
}

test "processFile with example" {
    const result = processFile(
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    );

    try std.testing.expectEqual(142, result);
}
