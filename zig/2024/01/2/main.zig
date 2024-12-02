const std = @import("std");
const file: []const u8 = @embedFile("input.txt");

pub fn main() !void {
    const result = try process(file);

    std.debug.print("{}\n", .{result});
}

fn process(input: []const u8) !u32 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var locations_1 = std.ArrayList(u32).init(allocator);
    var locations_2 = std.ArrayList(u32).init(allocator);

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| {
        var locations = std.mem.tokenizeScalar(u8, line, ' ');

        const location_1 = try std.fmt.parseInt(u32, locations.next().?, 10);
        try locations_1.append(location_1);

        const location_2 = try std.fmt.parseInt(u32, locations.next().?, 10);
        try locations_2.append(location_2);
    }

    std.mem.sort(u32, locations_1.items, {}, std.sort.asc(u32));
    std.mem.sort(u32, locations_2.items, {}, std.sort.asc(u32));

    var result: u32 = 0;
    for (locations_1.items) |location_1| {
        var number_of_match: u32 = 0;
        for (locations_2.items) |location_2| {
            if (location_1 == location_2) {
                number_of_match += 1;
            }
        }
        result += location_1 * number_of_match;
    }

    return result;
}

test "process with example" {
    const result = process(
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
    );

    try std.testing.expectEqual(31, result);
}
