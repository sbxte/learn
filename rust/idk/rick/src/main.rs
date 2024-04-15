fn main() {
    let v = IntoIterator::into_iter(
        vec![None; 2]
            .into_iter()
            .chain(vec![Some("gnu_c_ver=2"); 6]),
    );
    dbg!(&v);
    let v = v
        .chain(vec![Some("ableos"), Some("gnomed")])
        .chain(vec![Some("igneous vector yeet p"); 9])
        .into_iter();
    dbg!(&v);
    let v = v.map(|x| format!("{x:?} "));
    dbg!(&v);
    let v = v.zip(unsafe {
        (0..19)
            .map(|i| {
                b"admtgbhhgohgouvknl{bepuhciihpihqvwmom|"
                    .as_ptr()
                    .add(i as usize)
            })
            .map(|p| (*p - 97) as usize..((*p.add(19)) - 97) as usize)
            .collect::<Vec<_>>()
    });
    dbg!(&v);
    let v = v
        .map(|(x, y)| ToString::to_string(&x[y]))
        .collect::<Vec<_>>()
        .join("");
    dbg!(&v);

    println!(
        "{}",
        IntoIterator::into_iter(
            vec![None; 2]
                .into_iter()
                .chain(vec![Some("gnu_c_ver=2"); 6])
        )
        .chain(vec![Some("ableos"), Some("gnomed")])
        .chain(vec![Some("igneous vector yeet p"); 9])
        .into_iter()
        .map(|x| format!("{x:?} "))
        .zip(unsafe {
            (0..19)
                .map(|i| {
                    b"admtgbhhgohgouvknl{bepuhciihpihqvwmom|"
                        .as_ptr()
                        .add(i as usize)
                })
                .map(|p| (*p - 97) as usize..((*p.add(19)) - 97) as usize)
                .collect::<Vec<_>>()
        })
        .map(|(x, y)| ToString::to_string(&x[y]))
        .collect::<Vec<_>>()
        .join("")
    );
}
