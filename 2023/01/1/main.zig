const std = @import("std");
const data = @embedFile("input.txt");
const print = std.debug.print;

const ascii_offset: usize = '0';

pub fn main() !void {
    var result: usize = 0;

    var lines = std.mem.tokenize(u8, data, "\n");
    while (lines.next()) |line| {
        result += processLine(line);
    }

    print("{d}\n", .{result});
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
