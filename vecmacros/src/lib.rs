#[macro_export]
macro_rules! avec {

    ($($element:expr),*) => {{
        const _: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::count![@COUNT; $($element),*]);
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr),*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::with_capacity($count);
        let x = $element;
        vs.resize($count, x.clone());
        vs
    }};

}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn empty_vec() {
        let x: Vec<u32> = avec![];
        assert!(x.is_empty());
    }

    #[test]
    fn double() {
        let x: Vec<u32> = avec![42, 43];
        assert!(!x.is_empty());
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 43);
    }

    #[test]
    fn clone_macro() {
        let mut y = Some(42);
        let x: Vec<u32> = avec![y.take().unwrap(); 2];
        assert!(!x.is_empty());
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }
}







