// enums1.rs
//
// No hints this time! ;)

/*
### Scott's Notes:

This was an unusually hard task for me.  I was plagued by newby mistakes that I couldn't figure out from the compiler errors:

* I was expecting a more "packaged" problem even though the "No hints this time! ;)" was warning me that I was more on my own.
* So I had to review how to pass a parameter to an enum variant.
* Then I had to generate the sample parameter data.
* The stumbling block was the error message that `Message::Echo`'s value wasn't handled by the `Debug` trait.  I didn't know what `trait` meant (that's later in "The Book"), and I wondered why the authors of this exercise would introduce that at this point.
* It turned out that I missed another error message that indicated that I hadn't specified the argument required by the enum variant.
* At one (well, several) points, I was convinced the Rustlings output messaging was malfunctioning; I restarted `rustlings watch` several times. In retrospect, I thing **rustlings** was working fine.

All in all, a painful experience, but I learned from it.

One thing that helped was **[copilot](github.com/copilot)** suggesting various ideas.  At times it was misleading, but I could instantly see that and move on. But sometimes it was helpful.
 */

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move {x: i32, y: i32 },
    ChangeColor((u8, u8, u8)),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("echo")));
    println!("{:?}", Message::Move {x: 50, y: 25});
    println!("{:?}", Message::ChangeColor ((255, 0, 0)) );
}
