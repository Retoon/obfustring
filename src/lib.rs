#![doc(html_root_url = "https://docs.rs/obfustring/latest/obfustring")]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use rand::Rng;
use syn::parse_macro_input;

/// Obfuscates a `str` at compile time.
///
/// Instead of a string literal being stored in your binary, it
/// is substituted for a `[u8; str_lit_size*2]` array literal of the original `str`.
/// The values are offset by a random value and recalculated on each call.
/// This means that it will won't show up when inspecting the binary
/// outright or through a debugger or hexeditor. The offset is stored
/// alongside the actual data, so any string literal will be guaranteed
/// to be double in size, and therefore take up twice as much memory.
///
/// This is especially useful if you're trying to protect the
/// more sensetive parts of your program to basic reverse engineering
/// techniques like string reference lookup.
///
/// # Intended Usage
/// Though it doesn't make it absolutely impossible, this macro
/// tries to make it **a lot** harder to look for string references
/// that would be physically near some sensetive functions that handle sensetive
/// things, like decrypting file indexies or handling security checks to name a few.
///
/// <br>
///
/// # Example
/// ```
/// use obfustring::obfustring;
///
/// let obfuscated_string = obfustring!("Hello obfustring!"); // <-- Won't show up in binaries or hex editors
/// let generic_string = String::from("Hello regular string!"); // <-- Will show up in binaries or hex editors
///
/// println!("obfuscated_string: {}", obfuscated_string);
/// println!("generic_string: {}", generic_string);
/// ```
///
/// <br>
///
/// # Expansion
/// `obfustring!("Hello obfustring!");` will expand into something like:
/// ```ignore
/// || -> String {
///     let sec_slice = [
///         151u8, 79u8, 149u8, 48u8, 168u8, 60u8, 139u8, 31u8, 163u8, 52u8, 63u8, 31u8, 153u8,
///         42u8, 118u8, 20u8, 160u8, 58u8, 176u8, 59u8, 135u8, 20u8, 195u8, 79u8, 174u8, 60u8,
///         181u8, 76u8, 179u8, 69u8, 119u8, 16u8, 103u8, 70u8,
///     ];
///     let mut str_vec: Vec<u8> = Vec::default();
///     let mut skip = false;
///     for (idx, val) in sec_slice.iter().enumerate() {
///         if skip {
///             skip = !skip;
///             continue;
///         }
///         str_vec.push(val - sec_slice[idx + 1]);
///         skip = !skip;
///     }
///     String::from_utf8_lossy(&str_vec).to_string()
/// }()
/// ```
///
/// <br>
///
/// # Disclaimer
/// Note that you should **never** have any encryption keys or
/// sensetive data hardcoded into your program. Though this macro
/// would make it harder, it wouldn't absolutely hide it from
/// someone looking hard enough.
///
#[proc_macro]
pub fn obfustring(item: TokenStream) -> TokenStream {
    let mut rng = rand::thread_rng();

    let input_str = parse_macro_input!(item as syn::LitStr).value();

    let mut output_vec: Vec<u8> = Vec::new();

    for current_byte in input_str.as_bytes() {
        let mut rand_add: u8 = rng.gen_range(0..80);
        let offset_value: u8;

        loop {
            match current_byte.checked_add(rand_add) {
                Some(sum) => {
                    offset_value = sum;
                    break;
                }
                None => {
                    rand_add -= 1;
                    continue;
                }
            }
        }

        output_vec.push(offset_value);
        output_vec.push(rand_add);
    }

    // Concatenate all the u8's together so they fit in a static array expression
    let mut array_literal_tokenstream = quote! {};
    for char_byte in output_vec {
        array_literal_tokenstream = quote! {#array_literal_tokenstream #char_byte,};
    }

    let final_tokenstream = quote!(|| -> String {
        let sec_slice = [#array_literal_tokenstream];
        let mut str_vec: Vec<u8> = Vec::default();
        let mut skip = false;
        for (idx, val) in sec_slice.iter().enumerate() {
            if skip {
                skip = !skip;
                continue;
            }
            str_vec.push(val - sec_slice[idx + 1]);
            skip = !skip;
        }

        String::from_utf8_lossy(&str_vec).to_string()
    }());

    final_tokenstream.into()
}
