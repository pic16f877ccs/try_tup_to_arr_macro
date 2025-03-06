#![forbid(unsafe_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate proc_macro;
use core::fmt::Write;
use proc_macro::TokenStream;

fn generic_type_param(n: usize) -> String {
    (0..=n).fold("".to_string(), |mut s, i| {
        let _ = write!(s, "T{}, ", i);
        s
    })
}
fn tup_to_fn_ident(n: usize) -> String {
    (0..=n).fold("".to_string(), |mut s, i| {
        let _ = write!(
            s,
            "            match U::try_from_int_str(self.{i}) {{
                    Ok(ok) => ok,
                    Err(err) => {{
                        return Err(TryTupToArrErr {{
                            source: err,
                            position: {pos},
                        }})
                 }},
            }},\n",
            pos = i + 1
        );
        s
    })
}

fn tup_to_type_bound(n: usize) -> String {
    (0..=n).fold("".to_string(), |mut s, i| {
        let _ = write!(s, "TryFromIntStr<T{}> + ", i);
        s
    })
}

#[rustfmt::skip]
fn tup_to_impl_code(n: usize) -> String {
    (0..=n).fold("".to_string(), |mut s, i| {
        let _ = write!(s,
"    
impl<U, {type_param}> TryTupToArr<U> for ({type_param})
where
    U: {type_bound},
{{   
    type A = [U; {i}];
     
    #[doc = \"Converts tuple ({type_doc}) to array [Self; {i}].\"]
    #[inline] 
    fn try_into_arr(self) -> Result<Self::A, TryTupToArrErr> {{
        Ok([
{fn_ident}        
        ])  
    }}   
}}
",
        type_param = generic_type_param(i),
        fn_ident = tup_to_fn_ident(i),
        type_bound = tup_to_type_bound(i),
        type_doc = generic_type_param(i).trim_end(),
        i = i + 1,); s }
    )
}

macro_rules! try_tup_to_arr_trait {
    ($to:expr) => {
        #[proc_macro]
        pub fn try_tup_to_arr_impl(_item: TokenStream) -> TokenStream {
            tup_to_impl_code($to - 1).parse().unwrap()
        }
    };
}

#[cfg(all(
    feature = "try_tup_to_arr_8",
    not(feature = "try_tup_to_arr_16"),
    not(feature = "try_tup_to_arr_32"),
    not(feature = "try_tup_to_arr_64")
))]
try_tup_to_arr_trait!(8);

#[cfg(all(
    feature = "try_tup_to_arr_16",
    not(feature = "try_tup_to_arr_8"),
    not(feature = "try_tup_to_arr_32"),
    not(feature = "try_tup_to_arr_64")
))]
try_tup_to_arr_trait!(16);

#[cfg(all(
    feature = "try_tup_to_arr_16",
    feature = "try_tup_to_arr_8",
    not(feature = "try_tup_to_arr_32"),
    not(feature = "try_tup_to_arr_64")
))]
try_tup_to_arr_trait!(16);

#[cfg(all(
    feature = "try_tup_to_arr_32",
    not(feature = "try_tup_to_arr_8"),
    not(feature = "try_tup_to_arr_16"),
    not(feature = "try_tup_to_arr_64")
))]
try_tup_to_arr_trait!(32);

#[cfg(all(
    feature = "try_tup_to_arr_32",
    feature = "try_tup_to_arr_8",
    not(feature = "try_tup_to_arr_16"),
    not(feature = "try_tup_to_arr_64")
))]
try_tup_to_arr_trait!(32);

#[cfg(all(
    feature = "try_tup_to_arr_32",
    feature = "try_tup_to_arr_16",
    not(feature = "try_tup_to_arr_8"),
    not(feature = "try_tup_to_arr_64")
))]
try_tup_to_arr_trait!(32);

#[cfg(all(
    feature = "try_tup_to_arr_32",
    feature = "try_tup_to_arr_8",
    feature = "try_tup_to_arr_16",
    not(feature = "try_tup_to_arr_64")
))]
try_tup_to_arr_trait!(32);

#[cfg(all(
    feature = "try_tup_to_arr_64",
    not(feature = "try_tup_to_arr_8"),
    not(feature = "try_tup_to_arr_16"),
    not(feature = "try_tup_to_arr_32")
))]
try_tup_to_arr_trait!(64);

#[cfg(all(
    feature = "try_tup_to_arr_64",
    feature = "try_tup_to_arr_8",
    not(feature = "try_tup_to_arr_16"),
    not(feature = "try_tup_to_arr_32")
))]
try_tup_to_arr_trait!(64);

#[cfg(all(
    feature = "try_tup_to_arr_64",
    feature = "try_tup_to_arr_16",
    not(feature = "try_tup_to_arr_8"),
    not(feature = "try_tup_to_arr_32")
))]
try_tup_to_arr_trait!(64);

#[cfg(all(
    feature = "try_tup_to_arr_64",
    feature = "try_tup_to_arr_32",
    not(feature = "try_tup_to_arr_8"),
    not(feature = "try_tup_to_arr_16")
))]
try_tup_to_arr_trait!(64);

#[cfg(all(
    feature = "try_tup_to_arr_64",
    feature = "try_tup_to_arr_8",
    feature = "try_tup_to_arr_16",
    not(feature = "try_tup_to_arr_32")
))]
try_tup_to_arr_trait!(64);

#[cfg(all(
    feature = "try_tup_to_arr_64",
    feature = "try_tup_to_arr_8",
    feature = "try_tup_to_arr_32",
    not(feature = "try_tup_to_arr_16")
))]
try_tup_to_arr_trait!(64);

#[cfg(all(
    feature = "try_tup_to_arr_64",
    feature = "try_tup_to_arr_16",
    feature = "try_tup_to_arr_32",
    not(feature = "try_tup_to_arr_8")
))]
try_tup_to_arr_trait!(64);

#[cfg(all(
    feature = "try_tup_to_arr_64",
    feature = "try_tup_to_arr_8",
    feature = "try_tup_to_arr_16",
    feature = "try_tup_to_arr_32"
))]
try_tup_to_arr_trait!(64);
