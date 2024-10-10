pub fn get_median<T: PartialOrd + Copy + Into<f64>>(v: &[T]) -> Option<f64>
{
  let mut sorted_v = v.to_vec();
  sorted_v.sort_by(|a, b| a.partial_cmp(b).unwrap());

  let len = sorted_v.len();
  if len == 0 {
    return None;
  }

  let median = if len % 2 == 0 {
    let mid = len / 2;
    (sorted_v[mid - 1].into() + sorted_v[mid].into()) / 2.0
  } else {
    sorted_v[len / 2].into()
  };

  Some(median)
}