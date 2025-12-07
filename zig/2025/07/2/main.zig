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

fn processResult(reader: *Reader) !usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var previous_count_line: []usize = undefined;
    var count_line: []usize = undefined;

    // Init first line
    if (try reader.takeDelimiter('\n')) |line| {
        previous_count_line = try allocator.alloc(usize, line.len);
        count_line = try allocator.alloc(usize, line.len);

        for (line, 0..) |char, i| {
            if (char == 'S') {
                previous_count_line[i] = 1;
            } else {
                previous_count_line[i] = 0;
            }
            count_line[i] = 0;
        }
        std.debug.print("{any}\n", .{previous_count_line});
    }

    while (try reader.takeDelimiter('\n')) |line| {
        for (line, 0..) |char, i| {
            switch (char) {
                '^' => {
                    if (previous_count_line[i] != 0) {
                        count_line[i] = 0;
                        count_line[i - 1] += previous_count_line[i];
                        count_line[i + 1] += previous_count_line[i];
                    }
                },
                else => {
                    count_line[i] += previous_count_line[i];
                },
            }
        }
        @memcpy(previous_count_line, count_line);
        @memset(count_line, 0);
        std.debug.print("{any}\n", .{previous_count_line});
    }

    var result: usize = 0;
    for (previous_count_line) |count| {
        result += count;
    }
    return result;
}

test "processResult with example" {
    const input =
        \\.......S.......
        \\...............
        \\.......^.......
        \\...............
        \\......^.^......
        \\...............
        \\.....^.^.^.....
        \\...............
        \\....^.^...^....
        \\...............
        \\...^.^...^.^...
        \\...............
        \\..^...^.....^..
        \\...............
        \\.^.^.^.^.^...^.
        \\...............
    ;

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(40, processResult(&reader));
}
