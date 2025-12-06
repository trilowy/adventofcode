const std = @import("std");
const Reader = std.Io.Reader;
const ArrayList = std.ArrayList;

const file_name = "input.txt";
const line_max_length = 4096;

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

fn processResult(reader: *Reader) !u128 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var columns = ArrayList(ArrayList(u128)).empty;

    // Init all columns with first row
    if (try reader.takeDelimiter('\n')) |line| {
        var tokens = std.mem.tokenizeScalar(u8, line, ' ');
        while (tokens.next()) |token| {
            const value = try std.fmt.parseInt(u128, token, 10);

            var column = ArrayList(u128).empty;
            try column.append(allocator, value);

            try columns.append(allocator, column);
        }
    }

    var result: u128 = 0;

    while (try reader.takeDelimiter('\n')) |line| {
        var tokens = std.mem.tokenizeScalar(u8, line, ' ');
        var i: usize = 0;
        while (tokens.next()) |token| {
            if (token[0] == '+') {
                for (columns.items[i].items) |value| {
                    result += value;
                }
            } else if (token[0] == '*') {
                var column_result: u128 = 1;
                for (columns.items[i].items) |value| {
                    column_result *= value;
                }
                result += column_result;
            } else {
                const value = try std.fmt.parseInt(u128, token, 10);
                try columns.items[i].append(allocator, value);
            }
            i += 1;
        }
    }

    return result;
}

test "processResult with example" {
    const input =
        \\123 328  51 64 
        \\ 45 64  387 23 
        \\  6 98  215 314
        \\*   +   *   +  
    ;

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(4277556, processResult(&reader));
}
