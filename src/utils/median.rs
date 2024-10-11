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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_median_empty() {
    let v: Vec<f64> = vec![];
    assert_eq!(get_median(&v), None);
  }

  #[test]
  fn test_get_median_single_element() {
    let v = vec![1.0];
    assert_eq!(get_median(&v), Some(1.0));
  }

  #[test]
  fn test_get_median_two_elements() {
    let v = vec![1.0, 3.0];
    assert_eq!(get_median(&v), Some(2.0));
  }

  #[test]
  fn test_get_median_odd_number_of_elements() {
    let v = vec![3.0, 1.0, 2.0];
    assert_eq!(get_median(&v), Some(2.0));
  }

  #[test]
  fn test_get_median_even_number_of_elements() {
    let v = vec![4.0, 1.0, 3.0, 2.0];
    assert_eq!(get_median(&v), Some(2.5));
  }

  #[test]
  fn test_get_median_with_integers() {
    let v = vec![4, 1, 3, 2];
    assert_eq!(get_median(&v), Some(2.5));
  }

  #[test]
  fn test_get_median_with_negative_numbers() {
    let v = vec![-1.0, -2.0, -3.0];
    assert_eq!(get_median(&v), Some(-2.0));
  }

  #[test]
  fn test_get_median_with_mixed_numbers() {
    let v = vec![1.0, -1.0, 0.0];
    assert_eq!(get_median(&v), Some(0.0));
  }
}
