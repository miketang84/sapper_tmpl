#[macro_use]
extern crate lazy_static;
extern crate tera;

use tera::Tera;
pub use tera::Context;

lazy_static! {
    static ref TERA: Tera = Tera::new("views/**/*");
}

pub fn render(path: &str, context: Context) -> String {
    TERA.render(path, context).unwrap_or_else(|e| {
        println!("rendering error: {:?}", e);
        // warn!("rendering error: {:?}", e);
        "rendering error".to_owned()
    })
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
