
// Тут 'a нужен потому что структура держит ссылку, и компилятор обязан
// знать, что Parser не переживёт input.
#[derive(Debug)]
struct Parser<'a> {
    input: &'a str,
    pos:    usize,
}

/*
    Аннотация говорит: возвращаемая ссылка живёт не дольше пересечения 
    лайфтаймов входов.
    Реальные времена жизни для a и b могут быть произвольными (разными)
     по своей фактической продолжительности. 
    В этом и заключается гибкость обобщённых параметров времени жизни.
*/
fn longest<'a>(a: &'a str, b: &'a str) ->&'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let s1 = String::from("abc") ;
    let s2 = "fehg" ;

    let res = longest(s1.as_str(), s2) ;
    println!("res: {res}") ;    // Out: res: fehg

    let v = Parser{
        input:  "abc",
        pos: 10,
    } ;
    println!("Parse: {v:?}") ;  // Out: Parse: Parser { input: "abc", pos: 10 }
}
