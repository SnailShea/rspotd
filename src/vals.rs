pub const DEFAULT_SEED: &str = "MPSJKMDHAI";
pub const DEFAULT_DES: &str = "DB.B5.CB.D6.11.17.D6.EB";
pub const ALPHANUM: [char; 36] = [
    '0', '1', '2', '3', '4', '5',
    '6', '7', '8', '9', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub const TABLE1: [[i32; 5]; 7] = [
    [15, 15, 24, 20, 24],
    [13, 14, 27, 32, 10],
    [29, 14, 32, 29, 24],
    [23, 32, 24, 29, 29],
    [14, 29, 10, 21, 29],
    [34, 27, 16, 23, 30],
    [14, 22, 24, 17, 13],
];
pub const TABLE2: [[i32; 10]; 6] = [
    [0, 1, 2, 9, 3, 4, 5, 6, 7, 8],
    [1, 4, 3, 9, 0, 7, 8, 2, 5, 6],
    [7, 2, 8, 9, 4, 1, 6, 0, 3, 5],
    [6, 3, 5, 9, 1, 8, 2, 7, 4, 0],
    [4, 7, 0, 9, 5, 2, 3, 1, 8, 6],
    [5, 6, 1, 9, 8, 0, 4, 3, 2, 7],
];
