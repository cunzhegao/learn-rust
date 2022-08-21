fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}
#[derive(Debug)]
pub(crate) struct Args {
  pub(crate) image_1: String,
  pub(crate) image_2: String,
  pub(crate) output: String
}

impl Args {
  pub(crate) fn new() -> Self {
    Args {
      image_1: get_nth_arg(1),
      image_2: get_nth_arg(2),
      output: get_nth_arg(3)
    }
  }
}