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

    var rows = ArrayList([]const u8).empty;

    while (try reader.takeDelimiter('\n')) |line| {
        const row: []u8 = try allocator.alloc(u8, line.len);
        @memcpy(row, line);

        try rows.append(allocator, row);
    }

    const row_len = rows.items[0].len;
    const column_len = rows.items.len - 1;
    const sign_column_idx = column_len;

    var result: u128 = 0;
    var sign: u8 = '+';
    var value = try ArrayList(u8).initCapacity(allocator, column_len);
    var op_result: u128 = 0;

    for (0..row_len) |row_idx| {
        if (rows.items[sign_column_idx][row_idx] != ' ') {
            result += op_result;
            sign = rows.items[sign_column_idx][row_idx];
            op_result = if (sign == '+') 0 else 1;
        }

        for (0..column_len) |column_idx| {
            if (rows.items[column_idx][row_idx] != ' ') {
                value.appendAssumeCapacity(rows.items[column_idx][row_idx]);
            }
        }

        if (value.items.len > 0) {
            const parsed_value = try std.fmt.parseInt(u128, value.items, 10);
            if (sign == '+') {
                op_result += parsed_value;
            } else {
                op_result *= parsed_value;
            }
        }

        value.clearRetainingCapacity();
    }
    result += op_result;

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

    try std.testing.expectEqual(3263827, processResult(&reader));
}
