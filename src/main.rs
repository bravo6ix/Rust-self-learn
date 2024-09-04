fn main() {
    // println!("Hello, world!");
    greet_world();
}


fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界!";
    let spanish = "Hola Mundo";
    let english = "Hello World";
    let regions = [southern_germany, chinese, spanish, english];
    for region in regions.iter() {
        println!("{}", region);
    }
}