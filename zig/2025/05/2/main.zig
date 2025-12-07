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

    var ranges = ArrayList(Range).empty;

    while (try reader.takeDelimiter('\n')) |line| {
        if (line.len == 0) {
            break; // Stop parsing range of fresh food
        }
        const range = try Range.from(line);
        try ranges.append(allocator, range);
    }

    std.mem.sort(Range, ranges.items, {}, Range.asc);

    var merged_ranges = ArrayList(Range).empty;

    for (ranges.items) |range| {
        if (merged_ranges.items.len == 0) {
            try merged_ranges.append(allocator, range);
            continue;
        }
        if (merged_ranges.getLast().merge(range)) |merged_range| {
            _ = merged_ranges.pop();
            merged_ranges.appendAssumeCapacity(merged_range);
        } else {
            try merged_ranges.append(allocator, range);
        }
    }

    var result: u128 = 0;

    for (merged_ranges.items) |range| {
        result += (range.end - range.start + 1);
    }

    return result;
}

const Range = struct {
    start: u128,
    end: u128,

    fn from(string: []const u8) !Range {
        var range = std.mem.splitScalar(u8, string, '-');
        const range_0 = range.next().?;
        const range_1 = range.next().?;

        const start = try std.fmt.parseInt(u128, range_0, 10);
        const end = try std.fmt.parseInt(u128, range_1, 10);

        return Range{
            .start = start,
            .end = end,
        };
    }

    fn merge(self: Range, to_merge: Range) ?Range {
        if (self.end + 1 < to_merge.start) {
            return null;
        }

        const end = if (self.end > to_merge.end) self.end else to_merge.end;

        return Range{
            .start = self.start,
            .end = end,
        };
    }

    fn asc(_: void, a: Range, b: Range) bool {
        return a.start < b.start;
    }
};

test "merge non overlaping ranges" {
    const range = Range{ .start = 1, .end = 3 };
    const result = range.merge(Range{ .start = 5, .end = 6 });

    try std.testing.expectEqual(null, result);
}

test "merge contigus ranges" {
    const range = Range{ .start = 1, .end = 3 };
    const result = range.merge(Range{ .start = 4, .end = 5 });

    try std.testing.expectEqual(1, result.?.start);
    try std.testing.expectEqual(5, result.?.end);
}

test "merge equal ranges" {
    const range = Range{ .start = 1, .end = 3 };
    const result = range.merge(Range{ .start = 1, .end = 3 });

    try std.testing.expectEqual(1, result.?.start);
    try std.testing.expectEqual(3, result.?.end);
}

test "merge ranges same start, bigger" {
    const range = Range{ .start = 1, .end = 3 };
    const result = range.merge(Range{ .start = 1, .end = 4 });

    try std.testing.expectEqual(1, result.?.start);
    try std.testing.expectEqual(4, result.?.end);
}

test "merge ranges same start, smaller" {
    const range = Range{ .start = 1, .end = 3 };
    const result = range.merge(Range{ .start = 1, .end = 2 });

    try std.testing.expectEqual(1, result.?.start);
    try std.testing.expectEqual(3, result.?.end);
}

test "merge ranges contained" {
    const range = Range{ .start = 1, .end = 4 };
    const result = range.merge(Range{ .start = 2, .end = 3 });

    try std.testing.expectEqual(1, result.?.start);
    try std.testing.expectEqual(4, result.?.end);
}

test "merge overlaping ranges" {
    const range = Range{ .start = 1, .end = 4 };
    const result = range.merge(Range{ .start = 2, .end = 5 });

    try std.testing.expectEqual(1, result.?.start);
    try std.testing.expectEqual(5, result.?.end);
}

test "processResult with example" {
    const input =
        \\3-5
        \\10-14
        \\16-20
        \\12-18
        \\
        \\1
        \\5
        \\8
        \\11
        \\17
        \\32
    ;

    var reader = Reader.fixed(input);

    try std.testing.expectEqual(14, processResult(&reader));
}
