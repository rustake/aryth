pub fn duobound_obsolete<I>(it: I) -> (BoundVector<f32>, BoundVector<f32>) where
    I: IntoIterator + Copy,
    I::Item: fmt::Display,
{
    let len = (&it).into_iter().count();
    let (mut vec_x, mut min_x, mut max_x, mut count_x) =
        (vec![None; len], None, None, 0);
    let (mut vec_y, mut min_y, mut max_y, mut count_y) =
        (vec![None; len], None, None, 0);
    (&it).indexed_iterate(|i, x| {
        let tx = x.to_string();
        return match tx.parse::<f32>() {
            Err(_) => {
                count_x += 1;
                let v = str_value(&tx) as f32;
                if min_x == None {
                    min_x = Some(v);
                    max_x = Some(v);
                }
                if v > max_x.unwrap() { max_x = Some(v) } else if v < min_y.unwrap() { min_y = Some(v) }
                vec_x[i] = Some(v)
            }
            Ok(v) => {
                count_y += 1;
                if min_y == None {
                    min_y = Some(v);
                    max_y = Some(v);
                }
                if v > max_y.unwrap() { max_y = Some(v) } else if v < min_y.unwrap() { min_y = Some(v) }
                vec_y[i] = Some(v)
            }
        };
    });
    return (
        BoundVector { vec: vec_x, min: min_x, max: max_x, count: count_x },
        BoundVector { vec: vec_y, min: min_y, max: max_y, count: count_y }
    );
}