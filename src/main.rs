fn main() {
    // loopキーワードを使用することで、ネストされたコードがループする
    let mut n: u32 = 10;
    loop {
        println!("Hello, world!: {}", n);
        // n--の様にデクリメントは出来ず、n-=1のようにしなければならない
        n-=1;
        // breakを使用することでループから脱出することが可能
        // continueを使用することで、そのループから脱出し、次のループへ移行可能
        if n == 0 { break };
    }

    // 上のコードのwhileバージョン
    // whileは式の値がfalseの時、内部でbreakを呼び出す
    let mut n: u32 = 10; // nの再定義。これはシャドーイング
    while n > 0 {
        // {}の中に、直に変数を書くことが可能
        println!("This sentence is while: {n}");
        n -= 1;
    }

    { // ラベルを使ったループ文
        let mut count = 0;
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;

            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    // ラベルを付けることで、外側にあるループをbreakすることが可能
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {}", count);
    }
    
    { // for loop
        let a = [10, 20, 30, 40, 50];
        // for inを使用することで、配列の中にある要素を1つ1つチェックしていくことが可能
        for element in a {
            println!("value is: {element}");
        };
    }

    { // for loop using Range type
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("END LOOP")    }
}
