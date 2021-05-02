use jsonschema_custom_validation::format;//this will be jsonschema::format in the full package

#[format]
fn test(_:&str) -> bool {
    true
}

fn main() {

}