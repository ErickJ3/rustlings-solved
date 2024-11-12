#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // Don't unwrap() None - this would panic
    if let Some(value) = my_option {
        println!("{:?}", value);
    }

    let my_arr = &[
        -1, -2, -3,  // Added missing comma
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // Vec::resize returns (), so we need to create the vec first
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Use std::mem::swap for swapping values
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}