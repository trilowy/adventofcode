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

    var columns = ArrayList(ArrayList(u8)).empty;

    // Init all columns with first row
    if (try reader.takeDelimiter('\n')) |line| {
        for (line) |char| {
            var column = ArrayList(u8).empty;
            try column.append(allocator, char);

            try columns.append(allocator, column);
        }
    }
    while (try reader.takeDelimiter('\n')) |line| {
        for (line, 0..) |char, i| {
            try columns.items[i].append(allocator, char);
        }
    }

    const row_len = columns.items.len;
    const column_len = columns.items[0].items.len - 1;
    const sign_column_idx = column_len;

    var result: u128 = 0;
    var sign: u8 = '+';
    var op_result: u128 = 0;

    for (0..row_len) |row_idx| {
        const sign_value = columns.items[row_idx].items[sign_column_idx];
        if (sign_value != ' ') {
            result += op_result;
            sign = sign_value;
            op_result = if (sign == '+') 0 else 1;
        }

        const value = std.mem.trim(u8, columns.items[row_idx].items[0..column_len], " ");
        if (value.len > 0) {
            const parsed_value = try std.fmt.parseInt(u128, value, 10);
            if (sign == '+') {
                op_result += parsed_value;
            } else {
                op_result *= parsed_value;
            }
        }
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
