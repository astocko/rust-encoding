// AUTOGENERATED FROM index-macintosh.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-macintosh.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    196, 197, 199, 201, 209, 214, 220, 225, 224, 226, 228, 227, 229, 231, 233,
    232, 234, 235, 237, 236, 238, 239, 241, 243, 242, 244, 246, 245, 250, 249,
    251, 252, 8224, 176, 162, 163, 167, 8226, 182, 223, 174, 169, 8482, 180,
    168, 8800, 198, 216, 8734, 177, 8804, 8805, 165, 181, 8706, 8721, 8719,
    960, 8747, 170, 186, 937, 230, 248, 191, 161, 172, 8730, 402, 8776, 8710,
    171, 187, 8230, 160, 192, 195, 213, 338, 339, 8211, 8212, 8220, 8221, 8216,
    8217, 247, 9674, 255, 376, 8260, 8364, 8249, 8250, 64257, 64258, 8225, 183,
    8218, 8222, 8240, 194, 202, 193, 203, 200, 205, 206, 207, 204, 211, 212,
    63743, 210, 218, 219, 217, 305, 710, 732, 175, 728, 729, 730, 184, 733,
    731, 711,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

#[inline]
pub fn backward(code: u16) -> u8 {
    match code {
        196 => 0, 197 => 1, 199 => 2, 201 => 3, 209 => 4, 214 => 5, 220 => 6,
        225 => 7, 224 => 8, 226 => 9, 228 => 10, 227 => 11, 229 => 12,
        231 => 13, 233 => 14, 232 => 15, 234 => 16, 235 => 17, 237 => 18,
        236 => 19, 238 => 20, 239 => 21, 241 => 22, 243 => 23, 242 => 24,
        244 => 25, 246 => 26, 245 => 27, 250 => 28, 249 => 29, 251 => 30,
        252 => 31, 8224 => 32, 176 => 33, 162 => 34, 163 => 35, 167 => 36,
        8226 => 37, 182 => 38, 223 => 39, 174 => 40, 169 => 41, 8482 => 42,
        180 => 43, 168 => 44, 8800 => 45, 198 => 46, 216 => 47, 8734 => 48,
        177 => 49, 8804 => 50, 8805 => 51, 165 => 52, 181 => 53, 8706 => 54,
        8721 => 55, 8719 => 56, 960 => 57, 8747 => 58, 170 => 59, 186 => 60,
        937 => 61, 230 => 62, 248 => 63, 191 => 64, 161 => 65, 172 => 66,
        8730 => 67, 402 => 68, 8776 => 69, 8710 => 70, 171 => 71, 187 => 72,
        8230 => 73, 160 => 74, 192 => 75, 195 => 76, 213 => 77, 338 => 78,
        339 => 79, 8211 => 80, 8212 => 81, 8220 => 82, 8221 => 83, 8216 => 84,
        8217 => 85, 247 => 86, 9674 => 87, 255 => 88, 376 => 89, 8260 => 90,
        8364 => 91, 8249 => 92, 8250 => 93, 64257 => 94, 64258 => 95,
        8225 => 96, 183 => 97, 8218 => 98, 8222 => 99, 8240 => 100, 194 => 101,
        202 => 102, 193 => 103, 203 => 104, 200 => 105, 205 => 106, 206 => 107,
        207 => 108, 204 => 109, 211 => 110, 212 => 111, 63743 => 112,
        210 => 113, 218 => 114, 219 => 115, 217 => 116, 305 => 117, 710 => 118,
        732 => 119, 175 => 120, 728 => 121, 729 => 122, 730 => 123, 184 => 124,
        733 => 125, 731 => 126, 711 => 127, _ => 255
    }
}