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

fn processResult(reader: *Reader) !usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var previous_line: []u8 = undefined;
    // Init first line
    if (try reader.takeDelimiter('\n')) |line| {
        std.debug.print("{s}\n", .{line});

        previous_line = try allocator.alloc(u8, line.len);
        @memcpy(previous_line, line);

        const s_idx = std.mem.indexOfScalar(u8, previous_line, 'S').?;
        previous_line[s_idx] = '|';
    }

    var result: usize = 0;

    while (try reader.takeDelimiter('\n')) |line| {
        for (line, 0..) |char, i| {
            switch (char) {
                '^' => {
                    if (previous_line[i] == '|') {
                        result += 1;
                        previous_line[i] = '^';
                        previous_line[i - 1] = '|';
                        previous_line[i + 1] = '|';
                    }
                },
                else => {
                    if (previous_line[i] == '^') {
                        previous_line[i] = '.';
                    }
                },
            }
        }
        std.debug.print("{s}\n", .{previous_line});
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

    try std.testing.expectEqual(21, processResult(&reader));
}
