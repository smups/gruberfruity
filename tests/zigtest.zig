// Adler32 checksum.
//
// https://tools.ietf.org/html/rfc1950#section-9
// https://github.com/madler/zlib/blob/master/adler32.c


// All keywords in a row
align ; allowzero ; and ; anyframe ; anytype ; asm ; async ; await ; break ;
catch ; comptime ; const ; defer ; else ; enum ; errdefer ; error ; export ;
extern ; fn ; for ; if ; inline ; noalias ; nosuspend ; or ; orelse ; packed ;
pub ; return ; linksection ; struct ; switch ; test ; threadlocal ; try ; union ;
unreachable ; usingnamespace ; var ; volatile ; while ;

// All built-in types
i8 ; i16 ; i32 ; i64 ; i128 ; isize ;
u8 ; u16 ; u32 ; u64 ; u128 ; usize ;
c_short ; c_int ; c_long ; c_longlong ;
c_ushort ; c_uint ; c_ulong ; c_ulonglong ;
f16 ; f32 ; f64 ; f80 ; f128 ;
c_longdouble ;
bool ;
anyopaque ; void ; noreturn ; type ; anyerror ;
comptime_int ; comptime_float ;

//All built-in primitive values
true ; false ;
null ; undefined ;
1345 ; -1234 ; 0x231345 ; 0o1235 ; 0b1010101001
0.1235 ; 123.0e+77 ; 0x1324.70p-5 ; 

const std = @import("../std.zig");
const testing = std.testing;

pub const Adler32 = struct {
    const base = 65521;
    const nmax = 5552;

    adler: u32,

    pub fn init() Adler32 {
        return Adler32{ .adler = 1 };
    }

    // This fast variant is taken from zlib. It reduces the required modulos and unrolls longer
    // buffer inputs and should be much quicker.
    pub fn update(self: *Adler32, input: []const u8) void {
        var s1 = self.adler & 0xffff;
        var s2 = (self.adler >> 16) & 0xffff;

        if (input.len == 1) {
            s1 +%= input[0];
            if (s1 >= base) {
                s1 -= base;
            }
            s2 +%= s1;
            if (s2 >= base) {
                s2 -= base;
            }
        } else if (input.len < 16) {
            for (input) |b| {
                s1 +%= b;
                s2 +%= s1;
            }
            if (s1 >= base) {
                s1 -= base;
            }

            s2 %= base;
        } else {
            const n = nmax / 16; // note: 16 | nmax

            var i: usize = 0;

            while (i + nmax <= input.len) {
                var rounds: usize = 0;
                while (rounds < n) : (rounds += 1) {
                    comptime var j: usize = 0;
                    inline while (j < 16) : (j += 1) {
                        s1 +%= input[i + j];
                        s2 +%= s1;
                    }
                    i += 16;
                }

                s1 %= base;
                s2 %= base;
            }

            if (i < input.len) {
                while (i + 16 <= input.len) : (i += 16) {
                    comptime var j: usize = 0;
                    inline while (j < 16) : (j += 1) {
                        s1 +%= input[i + j];
                        s2 +%= s1;
                    }
                }
                while (i < input.len) : (i += 1) {
                    s1 +%= input[i];
                    s2 +%= s1;
                }

                s1 %= base;
                s2 %= base;
            }
        }

        self.adler = s1 | (s2 << 16);
    }

    pub fn final(self: *Adler32) u32 {
        return self.adler;
    }

    pub fn hash(input: []const u8) u32 {
        var c = Adler32.init();
        c.update(input);
        return c.final();
    }
};

test "adler32 sanity" {
    try testing.expectEqual(@as(u32, 0x620062), Adler32.hash("a"));
    try testing.expectEqual(@as(u32, 0xbc002ed), Adler32.hash("example"));
}

test "adler32 long" {
    const long1 = [_]u8{1} ** 1024;
    try testing.expectEqual(@as(u32, 0x06780401), Adler32.hash(long1[0..]));

    const long2 = [_]u8{1} ** 1025;
    try testing.expectEqual(@as(u32, 0x0a7a0402), Adler32.hash(long2[0..]));
}

test "adler32 very long" {
    const long = [_]u8{1} ** 5553;
    try testing.expectEqual(@as(u32, 0x707f15b2), Adler32.hash(long[0..]));
}

test "adler32 very long with variation" {
    const long = comptime blk: {
        @setEvalBranchQuota(7000);
        var result: [6000]u8 = undefined;

        var i: usize = 0;
        while (i < result.len) : (i += 1) {
            result[i] = @truncate(u8, i);
        }

        break :blk result;
    };

    try testing.e