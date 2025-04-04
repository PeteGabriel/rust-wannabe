
/// Echo repeats input.
///
/// # Examples
///
/// ```
/// <read "Hello" input from cmd line>
/// echo::echo(to_echo);
///
/// <prints "Hello">
/// ```
pub fn echo(txt: String){
  println!("{txt}");
}