fn main() {
    let address: usize = [...];

    let recursive_index = 0o777;
    let sign_extension = 0o177777 << 48;

    // retrieve page table indices of the address for translation
    let l4_idx = (address >> 39) & 0o777; // level 4 index
    let l3_idx = (address >> 30) & 0o777; // level 3 index
    let l2_idx = (address >> 21) & 0o777; // level 2 index
    let l1_idx = (address >> 12) & 0o777; // level 1 index
    let page_offset = address & 0o7777;

    let level_4_table_address =
        sign | ( recursive_index << 39 ) | ( recursive_index << 30 ) | ( recursive_index << 21 ) | ( recursive_index << 12 );
    let level_3_table_address =
        sign | ( recursive_index << 39 ) | ( recursive_index << 30 ) | ( recursive_index << 21 ) | ( l4_idx << 12 );
    let level_2_table_address =
        sign | ( recursive_index << 39 ) | ( recursive_index << 30 ) | ( recursive_index << 21 ) | ( l3_idx << 12 );
    let level_1_table_address =
        sign | ( recursive_index << 39 ) | ( recursive_index << 30 ) | ( recursive_index << 21 ) | ( l2_idx << 12 );

}
