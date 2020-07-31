// use phf::phf_map;

// pub static INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map! {
//     "BRK" => "impl",
//     "ORA" => "X,ind",
// };
pub const INSTRUCTIONS: [[&'static str; 2]; 5] = [
    ["BRK", "impl"],
    ["ORA", "X,ind"],
    ["*", "*"],
    ["*", "*"],
    ["*", "*"],
];
