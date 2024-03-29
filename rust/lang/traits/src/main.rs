trait Sound {
    fn play(&self);
}

#[derive(Default)]
struct Cat;
#[derive(Default)]
struct Dog;

impl Sound for Cat {
    fn play(&self) {
        println!("Meow!");
    }
}

impl Sound for Dog {
    fn play(&self) {
        println!("Woof!");
    }
}

fn make_noise_static(x: impl Sound) {
    x.play();
}

fn make_noise_dynamic(x: Box<dyn Sound>) {
    x.play();
}

trait Eat {
    fn eat(&self);
}

impl Eat for Cat {
    fn eat(&self) {
        println!("Nyom nyom nyom");
    }
}

fn eat_and_sound<T: Sound + Eat + ?Sized>(x: &T) {
    x.play();
    x.eat();
}

fn main() {
    println!("Static dispatch");
    let cat = Cat {};
    let dog = Dog {};
    make_noise_static(cat);
    make_noise_static(dog);

    println!("Dynamic dispatch");
    let boxed_cat: Box<Cat> = Default::default();
    let boxed_dog: Box<Dog> = Default::default();
    make_noise_dynamic(boxed_cat);
    make_noise_dynamic(boxed_dog);

    println!("Dynamic dispatch with arrays");
    let audibles: Vec<Box<dyn Sound>> = vec![
        Box::new(Cat),
        Box::new(Dog),
        Box::new(Cat),
    ];
    for audible in audibles {
        (*audible).play();
    }

    println!("Multi trait dynamic dispatch");
    let boxed_cat: Box<Cat> = Default::default();
    eat_and_sound(&*boxed_cat); // "&*" lookin funky. This actually dereferences and then borrows
                                // in order to match the function signature
}
