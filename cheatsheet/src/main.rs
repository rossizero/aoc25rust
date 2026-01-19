fn main() {
    println!("Hello, world!");
    vectors();
    strings();
}

#[derive(Debug)]
enum Pan {
    A(String),
    B(i32)
}

fn vectors() {
    let vec1 = vec![1, 2, 3];
    println!("{:?}", vec1);

    let vec2: Vec<_> = (0..5).map(|x| x + 3).collect();
    println!("{:?}", vec2);

    let vec3: Vec<_> = (0..5).map(|i| Pan::A(format!("test {}", i))).collect();
    println!("{:?}", vec3);

    let mut vec11 = vec1;
    add_sth(&mut vec11, 13);

    println!("{:?}", vec11);

    println!("{}, {}, {}", vec11.len(), vec11.len() as i32 -7, vec11.len().saturating_sub(7));

}

fn refs() {
    let result: Result<String, std::io::Error> = Ok("hello".to_string());
    // With `ref` – borrow instead of move:
    if let Ok(ref s) = result {
        // s: &String (borrowed from result)
    }

    // Without `ref` – this would try to move the String out:
    if let Ok(s) = result {
        // s: String  (moved out of result)
    }


    let result: Result<String, std::io::Error> = Ok("hello".to_string());

    let borrowed: Result<&String, _> = result.as_ref();
    {
        let tmp = &result;
    }

    if result.as_ref().is_ok_and(|s| s.contains('-')) {
        // s: &String here
    }
}

fn strings() {
    let l = ".....S.....";
    let l = l.replace("S", "|");
    println!("replaced string {l}");

    let pos: Vec<usize> = l.chars().enumerate()
        .filter_map(|(id, val)| if val == '|' {Some(id)} else {None })
        .collect();
    println!("{:?}", pos)
}

fn add_sth<T>(vec: &mut Vec<T>, value: T) {
    vec.push(value);
}
