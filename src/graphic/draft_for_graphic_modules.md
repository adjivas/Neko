##### TO DO:
- Define the type for the attributes Posture and emotion
- Update function descriptions and fill the "stuff" in `impl Manager`
- Define an err catching in constructor fot `Struct Sprite`

```rust
mod err;
 
struct Sprite {
    const MAX_X : usize = 7,
    const MAX_Y : usize = 10,
    interval : usize
    sheet : Vec<(Posture, [[(Emotion, Texel) ; MAX_X] ; MAX_Y])>
}

impl Sprite {

    /// The constructor function `new` makes a Sprite (I guess).

    pub fn new(sheet : Vec<(Posture, [[(Emotion, Texel) ; MAX_X] ; MAX_Y])>, interval : usize) -> Result<Self> {
        // stuff
    }
}


/////////////////////////////////////


Struct Manager<Default> {
    texel : HashMap<(Posture, Emotion), Texel>,
    sprite : HashMap<String, Sprite>,
}

impl Manager {

    /// function insert_texel description

    pub fn insert_texel(&mut self, key : (Posture, Emotion), value : Texel) -> Result<()> {
        // stuff
    }
    
    /// function insert_sprite description
    
    pub fn insert_sprite(&mut self, key : String, value : Sprite) -> Result<()> {
        // Stuff
    }
}


////////////////////////////////////


enum Posture {
    LotusHandsOnFloor,
    LyingOnSomething,
}
    
enum Emotion {
    Happy,
    Malicious,
    None,
}
```
