extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld)]
pub fn hw(ts: TokenStream) -> TokenStream {
    println!("{:?}", &ts);
    let mut ast: syn::DeriveInput = syn::parse(ts).unwrap();
    println!("{}", &ast.ident.to_string());
    let name = &ast.ident;
    let gen = quote! {
        impl HelloWorld for #name {
            fn hw(&self) -> () {
                println!("Hello, derive {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro]
pub fn hwx(_: TokenStream) -> TokenStream {
    r#"println!("Hello, World. hwx !");"#.parse().unwrap()
}


fn test_raw_pointers() {
    let mut num = 1;
    // 将引用转为裸指针
    let num_raw_point = &mut num as *mut i32;
    unsafe {
        *num_raw_point = 100;
        println!("{} {} {:p}", num, *num_raw_point, &num);
        // Output: 100 100 0x8d8c6ff6bc
    }

    let address = num_raw_point as usize;
    // 将一个 usize 对象，转化为 裸指针
    let raw = address as *mut i32;
    unsafe {
        *raw = 200;
        println!("{} {} {:p} {}", num, *num_raw_point, &num, address);
        // Output: 200 200 0x8d8c6ff6bc 607946536636
    }
}




