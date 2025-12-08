const std = @import("std");
const Reader = std.Io.Reader;
const ArrayList = std.ArrayList;

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

fn processResult(reader: *Reader) !u128 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var rows = ArrayList([]u8).empty;

    while (try reader.takeDelimiter('\n')) |line| {
        const row = try allocator.alloc(u8, line.len);
        @memcpy(row, line);
        try rows.append(allocator, row);
    }

    var result: u128 = 0;
    var has_picked_up_roll = true;

    while (has_picked_up_roll) {
        has_picked_up_roll = false;
        for (0..rows.items.len) |row_idx| {
            for (0..rows.items[0].len) |col_idx| {
                if (rows.items[row_idx][col_idx] == '@' and has_less_than_4_neighbors(rows.items, row_idx, col_idx)) {
                    result += 1;
                    has_picked_up_roll = true;
                }
            }
        }
    }

    return result;
}

fn has_less_than_4_neighbors(map: [][]u8, row_idx: usize, col_idx: usize) bool {
    const start_row = if (row_idx > 0) row_idx - 1 else row_idx;
    const end_row = if (row_idx < map.len - 1) row_idx + 1 else row_idx;
    const start_col = if (col_idx > 0) col_idx - 1 else col_idx;
    const end_col = if (col_idx < map[0].len - 1) col_idx + 1 else col_idx;

    var neighbors: usize = 0;
    for (start_row..(end_row + 1)) |row| {
        for (start_col..(end_col + 1)) |col| {
            if (!(row == row_idx and col == col_idx) and map[row][col] == '@') {
                neighbors += 1;
                if (neighbors >= 4) {
                    return false;
                }
            }
        }
    }
    map[row_idx][col_idx] = 'x';
    return true;
}

test "processResult with example" {
    const input =
        \\..@@.@@@@.
        \\@@@.@.@.@@
        \\@@@@@.@.@@
        \\@.@@@@..@.
        \\@@.@@@@.@@
        \\.@@@@@@@.@
        \\.@.@.@.@@@
        \\@.@@@.@@@@
        \\.@@@@@@@@.
        \\@.@.@@@.@.
    ;

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(43, processResult(&reader));
}
