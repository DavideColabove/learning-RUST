extern crate privacy_examples;
use privacy_examples::outermost;

fn main(){
    outermost::inside::inner_function();
    privacy_examples::try_me()
}