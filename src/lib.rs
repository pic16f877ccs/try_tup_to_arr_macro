#![forbid(unsafe_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate proc_macro;
use proc_macro::TokenStream;

fn generic_type_param(n: usize) -> String {
    (0..=n).map(|i| format!("T{i}, ")).collect::<String>()
}                                                                                                                                                            
fn from_tup_fn_ident(n: usize) -> String {
    (0..=n) 
        .map(|i| format!(
"            match U::try_from_int_str(self.{i}) {{
                    Ok(ok) => ok,
                    Err(err) => {{
                        return Err(TryFromTupErr {{
                            source: err,
                            posice: {pos},
                        }})
                 }},
            }},\n", pos = i + 1))
        .collect::<String>()
}           
            
fn from_tup_type_bound(n: usize) -> String {
    (0..=n) 
        .map(|i| format!("TryFromIntStr<T{i}> + "))
        .collect::<String>()
}

#[rustfmt::skip]
fn from_tup_impl_code(n: usize) -> String {
    (0..=n).map(|i| format!(
"    
impl<U, {type_param}> TupToArr<U> for ({type_param})
where
    U: {type_bound},
{{   
    type A = [U; {i}];
     
    #[doc = \"Converts tuple ({type_doc}) to array [Self; {i}].\"]
    #[inline] 
    fn into_arr(self) -> Result<Self::A, TryFromTupErr> {{
        Ok([
{fn_ident}        
        ])  
    }}   
}}
",
        type_param = generic_type_param(i),
        fn_ident = from_tup_fn_ident(i),
        type_bound = from_tup_type_bound(i),
        type_doc = generic_type_param(i).trim_end(),
        i = i + 1,)).collect::<String>()
}

macro_rules! try_from_tup_trait {
    ($to:expr) => {

        #[proc_macro]
        pub fn tup_from_impl(_item: TokenStream) -> TokenStream {
            from_tup_impl_code($to - 1).parse().unwrap()
        }
    };
}

#[cfg(all(
    feature = "try_from_tup_8",
    not(feature = "try_from_tup_16"),
    not(feature = "try_from_tup_32"),
    not(feature = "try_from_tup_64")
))]
try_from_tup_trait!(8);

#[cfg(all(
    feature = "try_from_tup_16",
    not(feature = "try_from_tup_8"),
    not(feature = "try_from_tup_32"),
    not(feature = "try_from_tup_64")
))]
try_from_tup_trait!(16);
#[cfg(all(
    feature = "try_from_tup_16",
    feature = "try_from_tup_8",
    not(feature = "try_from_tup_32"),
    not(feature = "try_from_tup_64")
))]
try_from_tup_trait!(16);

#[cfg(all(
    feature = "try_from_tup_32",
    not(feature = "try_from_tup_8"),
    not(feature = "try_from_tup_16"),
    not(feature = "try_from_tup_64")
))]
try_from_tup_trait!(32);
#[cfg(all(
    feature = "try_from_tup_32",
    feature = "try_from_tup_8",
    not(feature = "try_from_tup_16"),
    not(feature = "try_from_tup_64")
))]
try_from_tup_trait!(32);
#[cfg(all(
    feature = "try_from_tup_32",
    feature = "try_from_tup_16",
    not(feature = "try_from_tup_8"),
    not(feature = "try_from_tup_64")
))]
try_from_tup_trait!(32);
#[cfg(all(
    feature = "try_from_tup_32",
    feature = "try_from_tup_8",
    feature = "try_from_tup_16",
    not(feature = "try_from_tup_64")
))]
try_from_tup_trait!(32);

#[cfg(all(
    feature = "try_from_tup_64",
    not(feature = "try_from_tup_8"),
    not(feature = "try_from_tup_16"),
    not(feature = "try_from_tup_32")
))]
try_from_tup_trait!(64);
#[cfg(all(
    feature = "try_from_tup_64",
    feature = "try_from_tup_8",
    not(feature = "try_from_tup_16"),
    not(feature = "try_from_tup_32")
))]
try_from_tup_trait!(64);
#[cfg(all(
    feature = "try_from_tup_64",
    feature = "try_from_tup_16",
    not(feature = "try_from_tup_8"),
    not(feature = "try_from_tup_32")
))]
try_from_tup_trait!(64);
#[cfg(all(
    feature = "try_from_tup_64",
    feature = "try_from_tup_32",
    not(feature = "try_from_tup_8"),
    not(feature = "try_from_tup_16")
))]
try_from_tup_trait!(64);
#[cfg(all(
    feature = "try_from_tup_64",
    feature = "try_from_tup_8",
    feature = "try_from_tup_16",
    not(feature = "try_from_tup_32")
))]
try_from_tup_trait!(64);
#[cfg(all(
    feature = "try_from_tup_64",
    feature = "try_from_tup_8",
    feature = "try_from_tup_32",
    not(feature = "try_from_tup_16")
))]
try_from_tup_trait!(64);
#[cfg(all(
    feature = "try_from_tup_64",
    feature = "try_from_tup_16",
    feature = "try_from_tup_32",
    not(feature = "try_from_tup_8")
))]
try_from_tup_trait!(64);
#[cfg(all(
    feature = "try_from_tup_64",
    feature = "try_from_tup_8",
    feature = "try_from_tup_16",
    feature = "try_from_tup_32"
))]
try_from_tup_trait!(64);
